use std::{collections::VecDeque, mem::MaybeUninit};

pub struct VersionCollection<T> {
    items: Vec<DataWithVersion<T>>,
    free_places: VecDeque<usize>,
    // reserved_places: VecDeque<usize>,
    count: usize,
}

unsafe impl<S: Send> Send for VersionCollection<S> { }
unsafe impl<S: Sync> Sync for VersionCollection<S> { }

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct VersionIndex {
    pub index: usize,
    pub version: usize,
}

pub struct DataWithVersion<T> {
    pub version: usize,
    pub data: MaybeUninit<T>,
}

impl<T> VersionCollection<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            free_places: VecDeque::new(),
            count: 0,
        }
    }

    pub fn insert(&mut self, data: T) -> VersionIndex {
        if let Some(index) = self.free_places.pop_front() {
            let version = self.items[index].version;

            self.items[index] = DataWithVersion::<T> {
                data: MaybeUninit::new(data),
                version,
            };

            return VersionIndex {
                index,
                version,
            }
        }

        let index: usize = self.items.len();
        let version = 0;

        self.items.push(DataWithVersion::<T> {
            data: MaybeUninit::new(data),
            version,
        });

        self.count += 1;
        
        VersionIndex {
            index,
            version,
        }
    }

    pub fn remove(&mut self, index: VersionIndex) -> Option<T> {
        let Some(data_with_version) = self.get_data_with_version_mut(index) else {
            return None;
        };

        data_with_version.version += 1;

        let data = unsafe {
            let data: T = std::mem::transmute_copy(&data_with_version.data);
            data_with_version.data = MaybeUninit::uninit();
            data
        };

        self.free_places.push_back(index.index);

        self.count -= 1;

        Some(data)
    }

    pub fn get(&self, index: VersionIndex) -> Option<&T> {
        self.get_data_with_version(index).map(|d| unsafe { d.data.assume_init_ref() })
    }

    pub fn get_mut(&mut self, index: VersionIndex) -> Option<&mut T> {
        self.get_data_with_version_mut(index).map(|d| unsafe { d.data.assume_init_mut() })
    }

    fn get_data_with_version(&self, index: VersionIndex) -> Option<&DataWithVersion<T>> {
        let Some(data_with_version) = self.items.get(index.index) else {
            return None;
        };

        if index.version != data_with_version.version {
            return None;
        }

        Some(data_with_version)
    }

    fn get_data_with_version_mut(&mut self, index: VersionIndex) -> Option<&mut DataWithVersion<T>> {
        let data_with_version = self.items.get_mut(index.index)?;

        if index.version != data_with_version.version {
            return None;
        }

        Some(data_with_version)
    }

    pub fn contains_index(&self, index: VersionIndex) -> bool {
        let Some(data_with_version) = self.items.get(index.index) else {
            return false;
        };

        index.version == data_with_version.version
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

impl<T> Drop for VersionCollection<T> {
    fn drop(&mut self) {
        self.free_places.make_contiguous();
        self.free_places.as_mut_slices().0.sort();

        let mut free_places_iter = self.free_places.iter();

        let mut free_place = free_places_iter.next();

        for (index, item) in self.items.iter_mut().enumerate() {
            let should_drop = loop {
                let Some(&free_index) = free_place else {
                    break true;
                };

                if free_index == index {
                    break false;
                }

                if free_index < index {
                    free_place = free_places_iter.next();
                }
            };

            if should_drop {
                drop(unsafe { std::mem::transmute_copy::<_, T>(item.data.assume_init_ref()) });
                continue;
            }
        }
    }
}