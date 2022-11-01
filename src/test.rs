#[cfg(test)]
use crate::number_of_steps;

#[test]
fn test_1() {
    let result = number_of_steps(14);

    assert_eq!(result, 6);
}

#[test]
fn test_2() {
    let result = number_of_steps(8);

    assert_eq!(result, 4);
}

#[test]
fn test_3() {
    let result = number_of_steps(123);

    assert_eq!(result, 12);
}
