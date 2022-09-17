use super::Sorter;

pub struct QuickSort;


fn quicksort<T: Ord>(slice: &mut [T]){

    match slice.len(){
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return
        }
        _ => {}
    }


    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty"); 
    let mut left  =0;
    let mut right = rest.len() -1;

    while left <= right {
        if &rest[left] <= pivot{
            left += 1;
        }else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1 ;
        }
        else {
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }


    // re-align left and right to account for the pivot at 0
    let left = left + 1;
    
    // place the pivot at its final location
    slice.swap(0, left - 1 );

    let (left, right) = slice.split_at_mut(left - 1);

    assert!(left.last() <= right.first()); 

    quicksort(left);
    quicksort(&mut right[1..]);

    // merge them together 

}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [unsorted | pivot | sorted ]
        quicksort(slice);
        // for unsorted in 0..slice.len() {
           
        // }
    }
}
 
#[test]
fn quicksort_works() {
    let mut things = vec![4, 3, 1, 2];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
