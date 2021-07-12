use core::ops::Range;

use super::RangeMap;

pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V> {
    pub fn or_insert(self, range: Range<K>, value: V) -> &'a mut V {
        match self {
            Self::Occupied(entry) => entry.into_mut(),
            Self::Vacant(entry) => entry.insert(range, value),
        }
    }
}

pub struct OccupiedEntry<'a, K, V> {
    range: &'a Range<K>,
    value: &'a mut V,
    map: &'a mut RangeMap<K, V>,
}

pub struct VacantEntry<'a, K, V> {
    point: K,
    map: &'a mut RangeMap<K, V>,
}

impl<'a, K, V> VacantEntry<'a, K, V> {}
