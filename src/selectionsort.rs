use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted ]
        for unsorted in 0..slice.len() {
            let (smallest_in_rest, _) =  slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
              //  .map(|(i, _)| unsorted + i )
                .expect("slice is non-empty");
            
            let smallest_in_rest = unsorted + smallest_in_rest;
            
            // let mut smallest_in_rest_2 = unsorted ;

            // for i in (unsorted + 1)..slice.len() {
            //     if slice[i] < slice[smallest_in_rest] {
            //         smallest_in_rest_2 = i;
            //     }
            // }

            //assert_eq!(smallest_in_rest, smallest_in_rest_2);

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}
 
#[test]
fn selection_works() {
    let mut things = vec![4, 3, 1, 2];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
