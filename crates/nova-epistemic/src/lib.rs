use nova_types::Digest;
#[derive(Clone, Debug, Eq, PartialEq)] pub struct Evidence { pub source:String, pub observation:Vec<u8>, pub digest:Digest }
impl Evidence { pub fn new(source:impl Into<String>, observation:Vec<u8>)->Self { let digest=Digest::of(b"nova.evidence.v1",&observation);Self{source:source.into(),observation,digest} } }
#[derive(Clone, Copy, Debug, Eq, PartialEq)] pub enum BeliefStatus { Unknown, Hypothetical, Supported, Contradicted, Rejected }
#[derive(Clone, Debug, Eq, PartialEq)] pub struct Belief { pub statement:String, pub status:BeliefStatus, pub confidence:u8, pub evidence:Vec<Digest> }
