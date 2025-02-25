#[cfg(test)]
use prodcast::app::lang::{mutate_x_ptr, mutate_x_val};

/// Tests that mutate_x_val does not change the value of x in the calling function
#[test]
fn test_mutate_x_val() {
    let x = 10;
    mutate_x_val(x);
    // x should remain unchanged because mutate_x_val takes x by value
    assert_eq!(x, 10);
}

/// Tests that mutate_x_ptr changes the value of x in the calling function
#[test]
fn test_mutate_x_ptr() {
    let mut x = 10;
    mutate_x_ptr(&mut x);
    // x should be incremented by 1
    assert_eq!(x, 11);
}
