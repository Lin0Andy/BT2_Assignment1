# Rust Sorting Library

## Usage

This Rust library crate provides sorting algorithms for sorting various types of objects. It includes implementations of Quick Sort, Selection Sort, Insertion Sort, and Merge Sort.

To use this library in your Rust project, add the following line to your 'Cargo.toml' file:

`.toml file:`

`[dependencies]
assign1 = "0.1.0"`


## Demo Screenshots

### The code:
![image](https://github.com/Lin0Andy/BT2_Assignment1/assets/121328479/2b58f42a-fa09-4f2b-869a-a746d01bce99)
### Demo of usage:
![image](https://github.com/Lin0Andy/BT2_Assignment1/assets/121328479/44680b2f-36df-4223-9946-030c4f1285fb)


## Examples

Here's a simple example demonstrating how to use the sorting functions provided by this library:

```use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};

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
}```

