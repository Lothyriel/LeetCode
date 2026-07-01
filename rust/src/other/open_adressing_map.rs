use super::Hashable;

pub struct OpenAddressingHashMap<K: Hashable, V> {
    slots: Vec<Slot<K, V>>,
    len: usize,
    tombstones: usize,
}

enum Slot<K, V> {
    Occupied(Entry<K, V>),
    Tombstone,
    Empty,
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

impl<K: Hashable + PartialEq, V> OpenAddressingHashMap<K, V> {
    const LOAD_FACTOR_NUM: usize = 2;
    const LOAD_FACTOR_DEN: usize = 3;

    pub fn new() -> Self {
        Self {
            slots: vec![],
            len: 0,
            tombstones: 0,
        }
    }

    fn overloaded(&self) -> bool {
        (self.len + self.tombstones + 1) * Self::LOAD_FACTOR_DEN
            > self.slots.len() * Self::LOAD_FACTOR_NUM
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.slots.is_empty() || self.overloaded() {
            self.grow();
        }

        let entry = Entry::new(key, value);
        let start = entry.hash % self.slots.len();

        let mut first_tombstone = None;

        for i in 0..self.slots.len() {
            let idx = (start + i) % self.slots.len();
            let slot = &mut self.slots[idx];

            match slot {
                Slot::Occupied(e) if e.hash == entry.hash && e.key == entry.key => {
                    let old_value = std::mem::replace(&mut e.value, entry.value);
                    return Some(old_value);
                }
                Slot::Tombstone => {
                    if first_tombstone.is_none() {
                        first_tombstone = Some(idx);
                    }
                }
                Slot::Empty => {
                    let insert_idx = if let Some(idx) = first_tombstone {
                        self.tombstones -= 1;
                        idx
                    } else {
                        idx
                    };

                    self.slots[insert_idx] = Slot::Occupied(entry);
                    self.len += 1;
                    return None;
                }
                Slot::Occupied(_) => continue,
            }
        }

        unreachable!("should always have an empty slot");
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if self.slots.is_empty() {
            return None;
        }

        let hash = key.hash();
        let start = hash % self.slots.len();

        for i in 0..self.slots.len() {
            let idx = (start + i) % self.slots.len();

            match &self.slots[idx] {
                Slot::Occupied(e) if e.hash == hash && &e.key == key => {
                    return Some(&e.value);
                }
                Slot::Empty => return None,
                Slot::Occupied(_) | Slot::Tombstone => {}
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.slots.is_empty() {
            return None;
        }

        let hash = key.hash();
        let start = hash % self.slots.len();

        for i in 0..self.slots.len() {
            let idx = (start + i) % self.slots.len();
            let slot = &mut self.slots[idx];

            match slot {
                Slot::Occupied(e) if e.hash == hash && &e.key == key => {
                    let Slot::Occupied(e) = std::mem::replace(slot, Slot::Tombstone) else {
                        unreachable!("just matched an occupied slot");
                    };

                    self.len -= 1;
                    self.tombstones += 1;
                    return Some(e.value);
                }
                Slot::Empty => return None,
                Slot::Occupied(_) | Slot::Tombstone => continue,
            }
        }

        unreachable!("should always have an empty slot");
    }

    fn grow(&mut self) {
        let new_cap = self.slots.len().max(2) * 2;

        let mut new = Vec::with_capacity(new_cap);
        new.resize_with(new_cap, || Slot::Empty);

        for slot in self.slots.drain(..) {
            if let Slot::Occupied(entry) = slot {
                Self::reinsert_entry(&mut new, entry);
            }
        }

        self.tombstones = 0;
        self.slots = new;
    }

    fn reinsert_entry(slots: &mut [Slot<K, V>], entry: Entry<K, V>) {
        let start = entry.hash % slots.len();

        for i in 0..slots.len() {
            let idx = (start + i) % slots.len();

            if let Slot::Empty = slots[idx] {
                slots[idx] = Slot::Occupied(entry);
                return;
            }
        }

        unreachable!("just resized, should have an empty slot");
    }
}

impl<K: Hashable + PartialEq, V> Default for OpenAddressingHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
