# Rust Sorting Library

## Usage

This Rust library crate provides sorting algorithms for sorting various types of objects. It includes implementations of Quick Sort, Selection Sort, Insertion Sort, and Merge Sort.

To use this library in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
assign1 = "0.1.0"

## Demo Screenshots



## Examples

Here's a simple example demonstrating how to use the sorting functions provided by this library:
```
use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // Sort using Quick Sort
    quick_sort(&mut numbers);
    println!("Sorted using Quick Sort: {:?}", numbers);

    // Sort using Selection Sort
    selection_sort(&mut numbers);
    println!("Sorted using Selection Sort: {:?}", numbers);

    // Sort using Insertion Sort
    insertion_sort(&mut numbers);
    println!("Sorted using Insertion Sort: {:?}", numbers);

    // Sort using Merge Sort
    merge_sort(&mut numbers);
    println!("Sorted using Merge Sort: {:?}", numbers);
}
```
