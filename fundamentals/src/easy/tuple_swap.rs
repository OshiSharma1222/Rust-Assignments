/*
  Problem 3: Tuple Swap

  Write a function that takes a tuple of two i32 values and returns a new tuple with the elements swapped.

  Run the tests for this problem with:
    cargo test --test tuple_swap_test
*/

pub fn swap_tuple(t: (i32, i32)) -> (i32, i32) {
    let x: i32 = t.0;
    let y: i32 = t.1;
    (y, x)
    //explanation: The function takes a tuple `t` as input, extracts the first and second elements into variables `x` and `y`, and then returns a new tuple with the order of the elements swapped.
}
