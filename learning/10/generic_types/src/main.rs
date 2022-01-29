use ::std::ops::Add;
use std::cmp::PartialOrd;

fn largest_element<T: Add<Output = T> + Copy + PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_element(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);
}
