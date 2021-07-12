use super::*;
use alloc::{format, vec, vec::Vec};

trait RangeMapExt<K, V> {
    fn to_vec(&self) -> Vec<(Range<K>, V)>;
}

impl<K, V> RangeMapExt<K, V> for RangeMap<K, V>
where
    K: Ord + Clone,
    V: Eq + Clone,
{
    fn to_vec(&self) -> Vec<(Range<K>, V)> {
        self.iter().map(|(kr, v)| (kr.clone(), v.clone())).collect()
    }
}

//
// Insertion tests
//

#[test]
fn empty_map_is_empty() {
    let range_map: RangeMap<u32, bool> = RangeMap::new();
    assert_eq!(range_map.to_vec(), vec![]);
}

#[test]
fn insert_into_empty_map() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(0..50, false);
    assert_eq!(range_map.to_vec(), vec![(0..50, false)]);
}

#[test]
fn new_same_value_immediately_following_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..3, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
    range_map.insert(3..5, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-------◌ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
}

#[test]
fn new_different_value_immediately_following_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..3, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
    range_map.insert(3..5, true);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
}

#[test]
fn new_same_value_overlapping_end_of_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..4, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
    range_map.insert(3..5, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-------◌ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
}

#[test]
fn new_different_value_overlapping_end_of_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-----◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..4, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
    range_map.insert(3..5, true);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
}

#[test]
fn new_same_value_immediately_preceding_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
    range_map.insert(3..5, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..3, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-------◌ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
}

#[test]
fn new_different_value_immediately_preceding_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
    range_map.insert(3..5, true);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..3, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    // ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..3, false), (3..5, true)]);
}

#[test]
fn new_same_value_wholly_inside_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-------◌ ◌ ◌ ◌ ◌
    range_map.insert(1..5, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(2..4, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-------◌ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
}

#[test]
fn new_different_value_wholly_inside_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◆-------◇ ◌ ◌ ◌ ◌
    range_map.insert(1..5, true);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(2..4, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
    // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌
    // ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌
    assert_eq!(
        range_map.to_vec(),
        vec![(1..2, true), (2..4, false), (4..5, true)]
    );
}

#[test]
fn replace_at_end_of_existing_range_should_coalesce() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..3, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
    range_map.insert(3..5, true);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ●---◌ ◌ ◌ ◌ ◌
    range_map.insert(3..5, false);
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-------◌ ◌ ◌ ◌ ◌
    assert_eq!(range_map.to_vec(), vec![(1..5, false)]);
}

#[test]
// Test every permutation of a bunch of touching and overlapping ranges.
fn lots_of_interesting_ranges() {
    use crate::stupid_range_map::StupidU32RangeMap;
    use permutator::Permutation;

    let mut ranges_with_values = [
        (2..3, false),
        // A duplicate duplicates
        (2..3, false),
        // Almost a duplicate, but with a different value
        (2..3, true),
        // A few small ranges, some of them overlapping others,
        // some of them touching others
        (3..5, true),
        (4..6, true),
        (5..7, true),
        // A really big range
        (2..6, true),
    ];

    ranges_with_values.permutation().for_each(|permutation| {
        let mut range_map: RangeMap<u32, bool> = RangeMap::new();
        let mut stupid: StupidU32RangeMap<bool> = StupidU32RangeMap::new();

        for (k, v) in permutation {
            // Insert it into both maps.
            range_map.insert(k.clone(), v);
            // NOTE: Clippy's `range_minus_one` lint is a bit overzealous here,
            // because we _can't_ pass an open-ended range to `insert`.
            #[allow(clippy::range_minus_one)]
            stupid.insert(k.start..=(k.end - 1), v);

            // At every step, both maps should contain the same stuff.
            let stupid2: StupidU32RangeMap<bool> = range_map.clone().into();
            assert_eq!(stupid, stupid2);
        }
    });
}

//
// Get* tests
//

#[test]
fn get() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(0..50, false);
    assert_eq!(range_map.get(&49), Some(&false));
    assert_eq!(range_map.get(&50), None);
}

#[test]
fn get_key_value() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(0..50, false);
    assert_eq!(range_map.get_key_value(&49), Some((&(0..50), &false)));
    assert_eq!(range_map.get_key_value(&50), None);
}

//
// Removal tests
//

#[test]
fn remove_from_empty_map() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.remove(0..50);
    assert_eq!(range_map.to_vec(), vec![]);
}

#[test]
fn remove_non_covered_range_before_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(25..75, false);
    range_map.remove(0..25);
    assert_eq!(range_map.to_vec(), vec![(25..75, false)]);
}

#[test]
fn remove_non_covered_range_after_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(25..75, false);
    range_map.remove(75..100);
    assert_eq!(range_map.to_vec(), vec![(25..75, false)]);
}

#[test]
fn remove_overlapping_start_of_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(25..75, false);
    range_map.remove(0..30);
    assert_eq!(range_map.to_vec(), vec![(30..75, false)]);
}

#[test]
fn remove_middle_of_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(25..75, false);
    range_map.remove(30..70);
    assert_eq!(range_map.to_vec(), vec![(25..30, false), (70..75, false)]);
}

#[test]
fn remove_overlapping_end_of_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(25..75, false);
    range_map.remove(70..100);
    assert_eq!(range_map.to_vec(), vec![(25..70, false)]);
}

#[test]
fn remove_exactly_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(25..75, false);
    range_map.remove(25..75);
    assert_eq!(range_map.to_vec(), vec![]);
}

#[test]
fn remove_superset_of_stored() {
    let mut range_map: RangeMap<u32, bool> = RangeMap::new();
    range_map.insert(25..75, false);
    range_map.remove(0..100);
    assert_eq!(range_map.to_vec(), vec![]);
}

// Gaps tests

#[test]
fn whole_range_is_a_gap() {
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌ ◌
    let range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◆-------------◇ ◌
    let outer_range = 1..8;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield the entire outer range.
    assert_eq!(gaps.next(), Some(1..8));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn whole_range_is_covered_exactly() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---------◌ ◌ ◌ ◌
    range_map.insert(1..6, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◆---------◇ ◌ ◌ ◌
    let outer_range = 1..6;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield no gaps.
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_before_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---◌ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..3, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
    let outer_range = 5..8;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield the entire outer range.
    assert_eq!(gaps.next(), Some(5..8));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_touching_start_of_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●-------◌ ◌ ◌ ◌ ◌
    range_map.insert(1..5, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
    let outer_range = 5..8;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield the entire outer range.
    assert_eq!(gaps.next(), Some(5..8));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_overlapping_start_of_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ●---------◌ ◌ ◌ ◌
    range_map.insert(1..6, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
    let outer_range = 5..8;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield from the end of the stored item
    // to the end of the outer range.
    assert_eq!(gaps.next(), Some(6..8));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_starting_at_start_of_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
    range_map.insert(5..6, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
    let outer_range = 5..8;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield from the item onwards.
    assert_eq!(gaps.next(), Some(6..8));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn items_floating_inside_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ●-◌ ◌ ◌ ◌
    range_map.insert(5..6, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ●-◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(3..4, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◆-------------◇ ◌
    let outer_range = 1..8;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield gaps at start, between items,
    // and at end.
    assert_eq!(gaps.next(), Some(1..3));
    assert_eq!(gaps.next(), Some(4..5));
    assert_eq!(gaps.next(), Some(6..8));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_ending_at_end_of_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◌ ◌ ●-◌ ◌
    range_map.insert(7..8, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◆-----◇ ◌
    let outer_range = 5..8;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield from the start of the outer range
    // up to the start of the stored item.
    assert_eq!(gaps.next(), Some(5..7));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_overlapping_end_of_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ●---◌ ◌ ◌ ◌
    range_map.insert(4..6, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌
    let outer_range = 2..5;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield from the start of the outer range
    // up to the start of the stored item.
    assert_eq!(gaps.next(), Some(2..4));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_touching_end_of_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ●-------◌ ◌
    range_map.insert(4..8, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
    let outer_range = 1..4;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield the entire outer range.
    assert_eq!(gaps.next(), Some(1..4));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn item_after_outer_range() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◌ ●---◌ ◌
    range_map.insert(6..7, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
    let outer_range = 1..4;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield the entire outer range.
    assert_eq!(gaps.next(), Some(1..4));
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn empty_outer_range_with_items_away_from_both_sides() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(1..3, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌
    range_map.insert(5..7, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
    let outer_range = 4..4;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield no gaps.
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn empty_outer_range_with_items_touching_both_sides() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◆---◇ ◌ ◌ ◌ ◌ ◌ ◌
    range_map.insert(2..4, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◆---◇ ◌ ◌ ◌
    range_map.insert(4..6, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
    let outer_range = 4..4;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield no gaps.
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

#[test]
fn empty_outer_range_with_item_straddling() {
    let mut range_map: RangeMap<u32, ()> = RangeMap::new();
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◆-----◇ ◌ ◌ ◌ ◌ ◌
    range_map.insert(2..5, ());
    // 0 1 2 3 4 5 6 7 8 9
    // ◌ ◌ ◌ ◌ ◆ ◌ ◌ ◌ ◌ ◌
    let outer_range = 4..4;
    let mut gaps = range_map.gaps(&outer_range);
    // Should yield no gaps.
    assert_eq!(gaps.next(), None);
    // Gaps iterator should be fused.
    assert_eq!(gaps.next(), None);
    assert_eq!(gaps.next(), None);
}

///
/// impl Debug
///

#[test]
fn map_debug_repr_looks_right() {
    let mut map: RangeMap<u32, ()> = RangeMap::new();

    // Empty
    assert_eq!(format!("{:?}", map), "{}");

    // One entry
    map.insert(2..5, ());
    assert_eq!(format!("{:?}", map), "{2..5: ()}");

    // Many entries
    map.insert(6..7, ());
    map.insert(8..9, ());
    assert_eq!(format!("{:?}", map), "{2..5: (), 6..7: (), 8..9: ()}");
}

// Iterator Tests

// TODO: more iterator tests

// TODO: uncomment
// #[test]
// fn into_iter_matches_iter() {
//     // Just use vec since that's the same implementation we'd expect
//     let mut range_map: RangeMap<u32, bool> = RangeMap::new();
//     range_map.insert(1..3, false);
//     range_map.insert(3..5, true);

//     let cloned = range_map.to_vec();
//     let consumed = range_map.into_iter().collect::<Vec<_>>();

//     // Correct value
//     assert_eq!(cloned, vec![(1..3, false), (3..5, true)]);

//     // Equality
//     assert_eq!(cloned, consumed);
// }
