use super::Hashable;

pub struct SeparateChainHashMap<K: Hashable, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
    len: usize,
}

struct Entry<K, V> {
    key: K,
    value: V,
    hash: usize,
}

impl<K: Hashable, V> Entry<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            hash: key.hash(),
            key,
            value,
        }
    }
}

impl<K: Hashable + PartialEq, V> SeparateChainHashMap<K, V> {
    const LOAD_FACTOR_NUM: usize = 6;
    const LOAD_FACTOR_DEN: usize = 7;

    pub fn new() -> Self {
        Self {
            buckets: vec![],
            len: 0,
        }
    }

    fn overloaded(&self) -> bool {
        (self.len + 1) * Self::LOAD_FACTOR_DEN > self.buckets.len() * Self::LOAD_FACTOR_NUM
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.buckets.is_empty() || self.overloaded() {
            self.grow();
        }

        let entry = Entry::new(key, value);

        let idx = entry.hash % self.buckets.len();

        let bucket = &mut self.buckets[idx];

        if let Some(e) = bucket
            .iter_mut()
            .find(|e| e.hash == entry.hash && e.key == entry.key)
        {
            e.value = entry.value;
        } else {
            bucket.push(entry);
            self.len += 1;
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.buckets.is_empty() {
            return None;
        }

        let hash = key.hash();
        let idx = hash % self.buckets.len();

        self.buckets
            .get(idx)
            .and_then(|b| b.iter().find(|e| e.hash == hash && &e.key == key))
            .map(|e| &e.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.buckets.is_empty() {
            return None;
        }

        let hash = key.hash();
        let idx = hash % self.buckets.len();

        let bucket = self.buckets.get_mut(idx)?;

        let pos = bucket
            .iter()
            .position(|e| e.hash == hash && &e.key == key)?;

        let entry = bucket.swap_remove(pos);

        self.len -= 1;

        Some(entry.value)
    }

    fn grow(&mut self) {
        let new_cap = self.buckets.len().max(2) * 2;

        let mut new = Vec::with_capacity(new_cap);
        new.resize_with(new_cap, Vec::new);

        for bucket in self.buckets.drain(..) {
            for entry in bucket {
                let idx = entry.hash % new_cap;
                new[idx].push(entry);
            }
        }

        self.buckets = new;
    }
}

impl<K: Hashable + PartialEq, V> Default for SeparateChainHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
