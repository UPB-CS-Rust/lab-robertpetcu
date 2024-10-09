fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let largest = input.iter().max().unwrap_or(&0);
    let smallest = input.iter().min().unwrap_or(&0);

    println!("{} is largest and {} is smallest", largest, smallest);
}
