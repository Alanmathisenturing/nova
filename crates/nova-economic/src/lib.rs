use nova_types::{Digest, StateRoot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapitalState {
    pub balance: i128,
    pub reserved: i128,
    pub exposure_limit: i128,
    pub epoch: u64,
}

impl CapitalState {
    pub fn root(&self) -> StateRoot {
        let mut b=Vec::new();
        b.extend_from_slice(&self.balance.to_le_bytes());
        b.extend_from_slice(&self.reserved.to_le_bytes());
        b.extend_from_slice(&self.exposure_limit.to_le_bytes());
        b.extend_from_slice(&self.epoch.to_le_bytes());
        StateRoot(Digest::of(b"nova.capital_state.v1", &b).0)
    }
    pub fn available(&self)->Result<i128, CapitalError>{ self.balance.checked_sub(self.reserved).ok_or(CapitalError::ArithmeticOverflow) }
    pub fn reserve(&self, amount:i128, prior_root:StateRoot)->Result<Self,CapitalError>{
        if prior_root!=self.root(){return Err(CapitalError::StaleStateRoot)}
        if amount<=0{return Err(CapitalError::InvalidAmount)}
        if amount>self.exposure_limit{return Err(CapitalError::ExposureLimit)}
        if amount>self.available()?{return Err(CapitalError::InsufficientCapital)}
        Ok(Self{balance:self.balance,reserved:self.reserved.checked_add(amount).ok_or(CapitalError::ArithmeticOverflow)?,exposure_limit:self.exposure_limit,epoch:self.epoch+1})
    }
    pub fn release(&self, amount:i128, prior_root:StateRoot)->Result<Self,CapitalError>{
        if prior_root!=self.root(){return Err(CapitalError::StaleStateRoot)}
        if amount<=0||amount>self.reserved{return Err(CapitalError::InvalidAmount)}
        Ok(Self{balance:self.balance,reserved:self.reserved-amount,exposure_limit:self.exposure_limit,epoch:self.epoch+1})
    }
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub enum CapitalError{StaleStateRoot,InvalidAmount,ExposureLimit,InsufficientCapital,ArithmeticOverflow}

#[cfg(test)]
mod tests{
 use super::*;
 fn s()->CapitalState{CapitalState{balance:1000,reserved:0,exposure_limit:600,epoch:0}}
 #[test]fn reserve_is_root_bound(){let a=s();let b=a.reserve(100,a.root()).unwrap();assert_eq!(b.reserved,100);assert_ne!(a.root(),b.root());}
 #[test]fn stale_root_rejected(){let a=s();assert_eq!(a.reserve(100,StateRoot([9;32])),Err(CapitalError::StaleStateRoot));}
 #[test]fn exposure_limit_rejected(){let a=s();assert_eq!(a.reserve(601,a.root()),Err(CapitalError::ExposureLimit));}
 #[test]fn insufficient_capital_rejected(){let a=CapitalState{balance:50,..s()};assert_eq!(a.reserve(100,a.root()),Err(CapitalError::InsufficientCapital));}
 #[test]fn release_cannot_exceed_reserved(){let a=s();assert_eq!(a.release(1,a.root()),Err(CapitalError::InvalidAmount));}
}
