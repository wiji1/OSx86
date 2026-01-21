use unsafe_demo::split_at_mut;

fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let tuple = split_at_mut(array.as_mut(), 5);

    println!("{:?}", tuple.0);
    println!("{:?}", tuple.1);
}
