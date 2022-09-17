# Sorting algorithms in rust 


### Bubble sort


### Insertion sort
> is a simple sorting algorithm that builds the final sorted array (or list) on item at a time. it must less efficient on large lists than morw advanced algorithms suchs **quicksort, heapsort or mergesort** 
> *```rust 
        1 Take a list and divide it into two part [sorted] and [unsorted]  :
        2 first part is the sorted 
        3 second part is the unsorted
        4 pick the first item in the unsorted items and move over to the sorted
        5 given swapping until the  item is greater than the item before it.
        6 repeat [4] until the list become empty      

        let unsorted = 0;
        for unsorted in 1..slice.len() {
            // slice[unsorted...] is not sorted
            // take slice[unsorted] and place in sorted location in slice..[unsorted]
            // [1, 3, 4     | 2 ]  // now we're going to pick up the two and place it in sorted in the right side,
            // Then start swapping until two is greater than the value before it.
            // [1, 3, 4, 2, |   ] 
            // [1, 3, 2, 4, |   ]
            // [1, 2, 3, 4, |   ]
        }
       ``` *

### Selection sort
> **selection sort** is an *in-place comparsion sorting algorithm*, it has an 0(n^2) time complexity, which makes it inefficient on large lists, and generally performs worse than the similar __insertion sort__. Selection sort is noted for its simplicity and has performance advantages over morw complicated algorithms in certain situations, particularly where auxiliary memory is limited.

**The algorithm divides the input list into two parts**
- a sorted sublist of items which is built up from left to right
- an unsorted sublist of items that occupy the rest of the list
__The algorithm proceeds by finding the smallest(depending of on the ordering) element in the unsorted sublist, swapping it with the leftmost unsorted element(putting it in sorted order), and moving the sublist boundaries one element to the right__

### Quicksort
> Quicksort (sometimes called partition-exchange sort) is an efficient sorting algorithm. Quicksort is a __divide-and-conquer algorithm__ it works by selecting a _pivot_ element from the array and partitioning the other elements into two sub-arrays, according to whether they are less than or greater than the _pivot_. The subarrays are then sorted _recursively_. This can be done __in place__, requiring small additional amount of __memory__ to perform the sorting.