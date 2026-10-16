pub trait Canonical {
    fn canonical_bytes(&self) -> Vec<u8>;
}

pub fn content_hash<T: Canonical>(value: &T) -> crate::Hash {
    crate::Hash::from_bytes(&value.canonical_bytes())
}

impl Canonical for u8 {
    fn canonical_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Canonical for u32 {
    fn canonical_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Canonical for u64 {
    fn canonical_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Canonical for bool {
    fn canonical_bytes(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }
}

impl Canonical for String {
    fn canonical_bytes(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let mut result = Vec::new();
        result.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        result.extend_from_slice(bytes);
        result
    }
}

impl<T: Canonical> Canonical for Vec<T> {
    fn canonical_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&(self.len() as u64).to_le_bytes());
        for item in self {
            let bytes = item.canonical_bytes();
            result.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
            result.extend_from_slice(&bytes);
        }
        result
    }
}

impl<T: Canonical> Canonical for Option<T> {
    fn canonical_bytes(&self) -> Vec<u8> {
        match self {
            None => vec![0],
            Some(v) => {
                let mut result = vec![1];
                result.extend_from_slice(&v.canonical_bytes());
                result
            }
        }
    }
}
