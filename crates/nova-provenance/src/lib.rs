use nova_types::{Digest,EventId,StateRoot};
#[derive(Clone,Debug,Eq,PartialEq)] pub struct Witness { pub event:EventId,pub previous:StateRoot,pub next:StateRoot,pub event_digest:Digest,pub transition_digest:Digest }
