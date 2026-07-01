pub mod separate_chain_map;

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for i32 {
    // MurmurHash3 finalizer step
    fn hash(&self) -> usize {
        let mut x = *self as u32;

        x ^= x >> 16;
        x *= 0x85ebca6b;
        x ^= x >> 13;
        x *= 0xc2b2ae35;
        x ^= x >> 16;

        x as usize
    }
}

impl Hashable for str {
    // FNV-1a
    fn hash(&self) -> usize {
        const OFFSET: usize = 0xcbf29ce484222325;
        const PRIME: usize = 0x100000001b3;

        let mut hash: usize = OFFSET;

        for byte in self.as_bytes() {
            hash ^= *byte as usize;
            hash = hash.wrapping_mul(PRIME);
        }

        hash
    }
}
