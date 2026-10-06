use nova_event_bus::{Event,EventBus};
use nova_persistence::Journal;
use nova_provenance::Witness;
use nova_replay::replay;
use nova_state::{transition,State};
use nova_types::{EventId,LogicalTime,Provenance,StateRoot};

#[derive(Debug)]
pub struct Runtime {
    state:State,
    history:Vec<Event>,
    bus:EventBus,
    pub witnesses:Vec<Witness>
}

impl Default for Runtime {
    fn default()->Self {
        Self{state:State::default(),history:Vec::new(),bus:EventBus::default(),witnesses:Vec::new()}
    }
}

impl Runtime {
    pub fn submit(&mut self,event:Event)->Result<StateRoot,String>{
        self.bus.publish(event);
        let e=self.bus.pop().ok_or("empty bus")?;
        if self.history.last().map(|x|x.id>=e.id).unwrap_or(false){
            return Err("duplicate or out-of-order event".into())
        }
        if e.parent!=self.history.last().map(|x|x.id){
            return Err("invalid parent".into())
        }
        let previous=self.state.root();
        let proposal=transition(&self.state,&e).map_err(|x|format!("transition: {x:?}"))?;
        let next=proposal.next;
        let root=next.root();
        self.witnesses.push(Witness{
            event:e.id,
            previous,
            next:root,
            event_digest:e.digest,
            transition_digest:proposal.digest
        });
        self.state=next;
        self.history.push(e);
        Ok(root)
    }

    pub fn submit_persistent(&mut self,event:Event,journal:&mut Journal)->Result<StateRoot,String>{
        if self.history.last().map(|x|x.id>=event.id).unwrap_or(false){
            return Err("duplicate or out-of-order event".into())
        }
        if event.parent!=self.history.last().map(|x|x.id){
            return Err("invalid parent".into())
        }
        transition(&self.state,&event).map_err(|x|format!("transition: {x:?}"))?;
        journal.append(&event).map_err(|e|format!("journal: {e:?}"))?;
        self.submit(event)
    }

    pub fn recover(path:impl AsRef<std::path::Path>)->Result<Self,String>{
        let (_journal,events)=Journal::open(path).map_err(|e|format!("journal: {e:?}"))?;
        let mut runtime=Self::default();
        for event in events { runtime.submit(event)?; }
        Ok(runtime)
    }

    pub fn replay(&self)->Result<StateRoot,String>{
        replay(&self.history,Some(self.state.root())).map(|r|r.root).map_err(|e|format!("{e:?}"))
    }
    pub fn state(&self)->&State{&self.state}
    pub fn history(&self)->&[Event]{&self.history}
    pub fn increment(&mut self,id:u64,delta:i64)->Result<StateRoot,String>{
        let parent=self.history.last().map(|e|e.id);
        self.submit(Event::increment(
            EventId(id),LogicalTime(id),parent,delta,
            Provenance{origin:"runtime".into(),trace_id:format!("trace-{id}")}
        ))
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::time::{SystemTime,UNIX_EPOCH};

    #[test]
    fn end_to_end_commit_and_replay(){
        let mut r=Runtime::default();
        r.increment(1,4).unwrap();
        r.increment(2,-1).unwrap();
        assert_eq!(r.state().counter,3);
        assert_eq!(r.replay().unwrap(),r.state().root());
    }

    #[test]
    fn tampered_history_is_rejected(){
        let mut r=Runtime::default();
        r.increment(1,4).unwrap();
        r.history[0].payload[0]^=1;
        assert!(r.replay().is_err());
    }

    #[test]
    fn persistent_restart_preserves_state_root(){
        let path=std::env::temp_dir().join(format!(
            "nova-runtime-{}.journal",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        let mut journal=Journal::open(&path).unwrap().0;
        let mut r=Runtime::default();
        r.submit_persistent(
            Event::increment(EventId(1),LogicalTime(1),None,4,
                Provenance{origin:"test".into(),trace_id:"persist-1".into()}),
            &mut journal
        ).unwrap();
        r.submit_persistent(
            Event::increment(EventId(2),LogicalTime(2),Some(EventId(1)),-1,
                Provenance{origin:"test".into(),trace_id:"persist-2".into()}),
            &mut journal
        ).unwrap();
        let root=r.state().root();
        drop(journal);

        let recovered=Runtime::recover(&path).unwrap();
        assert_eq!(recovered.state().counter,3);
        assert_eq!(recovered.state().root(),root);
        assert_eq!(recovered.replay().unwrap(),root);
        std::fs::remove_file(path).unwrap();
    }
}

fn main(){
    let mut r=Runtime::default();
    let root=r.increment(1,1).expect("commit");
    println!("NOVA M0 PASS: root={root}, replay={}",r.replay().expect("replay"));
}
