mod chash_table {
    pub trait CHasher<T> {
        fn hash(t: &T) -> usize;
    }

    pub struct CHashTable<T, V, K>
    where
        T: Clone + PartialEq,
        V: Clone,
        K: CHasher<T>,
    {
        buckets: Vec<Option<(T, V)>>,
        objects_count: usize,
        _hasher: std::marker::PhantomData<K>,
    }

    impl<T, V, K> CHashTable<T, V, K>
    where
        T: Clone + PartialEq,
        V: Clone,
        K: CHasher<T>,
    {
        pub fn new() -> Self {
            CHashTable::<T, V, K> {
                buckets: vec![None; 8],
                objects_count: 0,
                _hasher: std::marker::PhantomData,
            }
        }

        fn realloc(&mut self, new_size: usize) {
            if new_size <= self.buckets.len() {
                return;
            }

            let new_buckets = vec![None; new_size];
            let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);

            for (key, value) in old_buckets.into_iter().filter_map(|pair| pair) {
                self.insert(&key, value);
            }
        }

        fn hash(&self, key: &T) -> usize {
            K::hash(key) % self.buckets.len()
        }

        pub fn insert(&mut self, key: &T, value: V) {
            // Reallocate if there are too few empty cells left
            if self.objects_count * 2 >= self.buckets.len() {
                self.realloc(self.buckets.len() * 2);
            }

            let mut hashed_key = self.hash(key);
            while !self.buckets[hashed_key].is_none() {
                hashed_key = (hashed_key + 1) % self.buckets.len();
            }
            self.buckets[hashed_key] = Some((key.clone(), value));
            self.objects_count += 1;
        }

        pub fn get(&mut self, key: &T) -> Option<V> {
            let hashed_key = self.hash(key);

            for i in hashed_key..hashed_key + self.buckets.len() {
                match &self.buckets[i] {
                    Some((possible_key, possible_value)) => {
                        if *possible_key == *key {
                            return Some(possible_value.clone());
                        }
                    }
                    None => {
                        return None;
                    }
                }
            }

            None
        }
        
        pub fn reserve(&mut self, size: usize) {
            if size * 2 >= self.buckets.len() {
                self.realloc(size * 2);
            }
        }
    }
}

struct CHasherU32Impl {}

impl chash_table::CHasher<u32> for CHasherU32Impl {
    fn hash(t: &u32) -> usize {
        (*t as u64 * 1000000007u64 % 0x1_0000_0000u64) as usize
    }
}

#[test]
fn chash_table_test() {
    let mut ht = chash_table::CHashTable::<u32, u32, CHasherU32Impl>::new();

    ht.reserve(100);

    for i in (2u32..100).step_by(2) {
        ht.insert(&i, i * 2);
    }

    assert_eq!(ht.get(&10), Some(20));
    assert_eq!(ht.get(&18), Some(36));
    assert_eq!(ht.get(&21), None);
    assert_eq!(ht.get(&37), None);
}
