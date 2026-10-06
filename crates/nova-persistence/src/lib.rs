use nova_event_bus::{Event, EventKind};
use nova_types::{Digest, EventId, LogicalTime, Provenance, SchemaVersion};
use sha2::{Digest as ShaDigest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const MAGIC: &[u8; 8] = b"NOVAJRNL";
const VERSION: u32 = 1;
const MAX_RECORD: u64 = 16 * 1024 * 1024;

#[derive(Debug)]
pub enum JournalError {
    Io(io::Error),
    InvalidFormat(&'static str),
    Corruption { offset: u64 },
    Event(nova_event_bus::EventError),
    LengthOverflow,
}
impl From<io::Error> for JournalError { fn from(e: io::Error) -> Self { Self::Io(e) } }
impl From<nova_event_bus::EventError> for JournalError { fn from(e: nova_event_bus::EventError) -> Self { Self::Event(e) } }

pub struct Journal {
    path: PathBuf,
    file: File,
}

impl Journal {
    pub fn open(path: impl AsRef<Path>) -> Result<(Self, Vec<Event>), JournalError> {
        let path = path.as_ref().to_path_buf();
        let mut file = OpenOptions::new().create(true).read(true).append(true).open(&path)?;
        let len = file.metadata()?.len();
        if len == 0 {
            file.write_all(MAGIC)?;
            file.write_all(&VERSION.to_le_bytes())?;
            file.sync_data()?;
            return Ok((Self { path, file }, Vec::new()));
        }
        file.seek(SeekFrom::Start(0))?;
        let mut header = [0u8; 12];
        file.read_exact(&mut header)?;
        if &header[..8] != MAGIC { return Err(JournalError::InvalidFormat("bad journal magic")); }
        if u32::from_le_bytes(header[8..12].try_into().unwrap()) != VERSION {
            return Err(JournalError::InvalidFormat("unsupported journal version"));
        }

        let mut events = Vec::new();
        let mut offset = 12u64;
        loop {
            let mut lenbuf = [0u8; 8];
            match file.read_exact(&mut lenbuf) {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    return Err(JournalError::Corruption { offset });
                }
                Err(e) => return Err(e.into()),
            }
            let record_len = u64::from_le_bytes(lenbuf);
            if record_len == 0 || record_len > MAX_RECORD { return Err(JournalError::Corruption { offset }); }
            let total = record_len.checked_add(32).ok_or(JournalError::LengthOverflow)?;
            let mut record = vec![0u8; total as usize];
            file.read_exact(&mut record).map_err(|_| JournalError::Corruption { offset })?;
            let body = &record[..record_len as usize];
            let stored = &record[record_len as usize..];
            let actual = Sha256::digest(body);
            if stored != actual.as_slice() { return Err(JournalError::Corruption { offset }); }
            let event = decode_event(body).map_err(|_| JournalError::Corruption { offset })?;
            event.validate()?;
            events.push(event);
            offset += 8 + total;
        }
    }

    pub fn append(&mut self, event: &Event) -> Result<(), JournalError> {
        event.validate()?;
        let body = encode_event(event);
        let checksum = Sha256::digest(&body);
        self.file.write_all(&(body.len() as u64).to_le_bytes())?;
        self.file.write_all(&body)?;
        self.file.write_all(&checksum)?;
        self.file.sync_data()?;
        Ok(())
    }

    pub fn path(&self) -> &Path { &self.path }
}

fn put_u16(v: &mut Vec<u8>, x: u16) { v.extend_from_slice(&x.to_le_bytes()); }
fn put_u32(v: &mut Vec<u8>, x: u32) { v.extend_from_slice(&x.to_le_bytes()); }
fn put_u64(v: &mut Vec<u8>, x: u64) { v.extend_from_slice(&x.to_le_bytes()); }
fn put_bytes(v: &mut Vec<u8>, b: &[u8]) { put_u64(v, b.len() as u64); v.extend_from_slice(b); }
fn get_u16(c: &mut Cursor<&[u8]>) -> io::Result<u16> { let mut b=[0;2]; c.read_exact(&mut b)?; Ok(u16::from_le_bytes(b)) }
fn get_u32(c: &mut Cursor<&[u8]>) -> io::Result<u32> { let mut b=[0;4]; c.read_exact(&mut b)?; Ok(u32::from_le_bytes(b)) }
fn get_u64(c: &mut Cursor<&[u8]>) -> io::Result<u64> { let mut b=[0;8]; c.read_exact(&mut b)?; Ok(u64::from_le_bytes(b)) }
fn get_bytes(c: &mut Cursor<&[u8]>) -> io::Result<Vec<u8>> {
    let n=get_u64(c)? as usize; let mut b=vec![0;n]; c.read_exact(&mut b)?; Ok(b)
}
fn encode_event(e: &Event) -> Vec<u8> {
    let mut v=Vec::new();
    put_u64(&mut v,e.id.0);
    v.push(match e.kind { EventKind::Increment=>1, EventKind::Observation=>2 });
    put_u16(&mut v,e.schema.0);
    put_u64(&mut v,e.logical_time.0);
    match e.parent { Some(x)=>{v.push(1);put_u64(&mut v,x.0)},None=>v.push(0) }
    put_bytes(&mut v,&e.payload);
    put_bytes(&mut v,e.provenance.origin.as_bytes());
    put_bytes(&mut v,e.provenance.trace_id.as_bytes());
    v.extend_from_slice(&e.digest.0);
    v
}
fn decode_event(b: &[u8]) -> io::Result<Event> {
    let mut c=Cursor::new(b);
    let id=EventId(get_u64(&mut c)?);
    let mut k=[0;1];c.read_exact(&mut k)?;
    let kind=match k[0]{1=>EventKind::Increment,2=>EventKind::Observation,_=>return Err(io::Error::new(io::ErrorKind::InvalidData,"kind"))};
    let schema=SchemaVersion(get_u16(&mut c)?);
    let time=LogicalTime(get_u64(&mut c)?);
    c.read_exact(&mut k)?;
    let parent=if k[0]==1{Some(EventId(get_u64(&mut c)?))}else if k[0]==0{None}else{return Err(io::Error::new(io::ErrorKind::InvalidData,"parent"))};
    let payload=get_bytes(&mut c)?;
    let origin=String::from_utf8(get_bytes(&mut c)?).map_err(|_|io::Error::new(io::ErrorKind::InvalidData,"origin"))?;
    let trace_id=String::from_utf8(get_bytes(&mut c)?).map_err(|_|io::Error::new(io::ErrorKind::InvalidData,"trace"))?;
    let mut d=[0;32];c.read_exact(&mut d)?;
    if c.position()!=b.len() as u64{return Err(io::Error::new(io::ErrorKind::InvalidData,"trailing"))}
    Ok(Event{id,kind,schema,logical_time:time,parent,payload,provenance:Provenance{origin,trace_id},digest:Digest(d)})
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime,UNIX_EPOCH};
    fn path() -> PathBuf { std::env::temp_dir().join(format!("nova-journal-{}.bin",SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos())) }
    fn event(id:u64,parent:Option<u64>,delta:i64)->Event { Event::increment(EventId(id),LogicalTime(id),parent,delta,Provenance{origin:"test".into(),trace_id:format!("t-{id}")}) }

    #[test]
    fn restart_round_trip() {
        let p=path();
        let e1=event(1,None,4); let e2=event(2,Some(1),-1);
        { let (mut j, old)=Journal::open(&p).unwrap(); assert!(old.is_empty()); j.append(&e1).unwrap(); j.append(&e2).unwrap(); }
        let (_j, recovered)=Journal::open(&p).unwrap();
        assert_eq!(recovered,vec![e1,e2]);
        std::fs::remove_file(p).unwrap();
    }

    #[test]
    fn corruption_is_rejected_before_recovery() {
        let p=path();
        { let (mut j,_)=Journal::open(&p).unwrap(); j.append(&event(1,None,4)).unwrap(); }
        let mut bytes=std::fs::read(&p).unwrap();
        let n=bytes.len(); bytes[n-1]^=1; std::fs::write(&p,bytes).unwrap();
        assert!(matches!(Journal::open(&p),Err(JournalError::Corruption{..})));
        std::fs::remove_file(p).unwrap();
    }
}
