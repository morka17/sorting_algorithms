pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}


mod bubblesort;
mod insertionsort;
mod selectionsort; 
mod quicksort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;


#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where T: Ord { 
            slice.sort();
        }  
    }

    #[test]
    fn it_works() {
        let mut things = vec![4,3,1,2];

        StdSorter.sort(&mut things);
        assert_eq!(things, &[1,2, 3, 4]);

    }
}
