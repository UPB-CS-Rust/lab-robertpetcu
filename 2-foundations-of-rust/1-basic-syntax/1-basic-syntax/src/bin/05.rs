fn main() {
    let mut data = [22, 12, 13, 17, 18];
    for i in 0..5 {
        data[i] = floored_half(data[i]);
    }
}

fn floored_half(value: i32) -> i32 {
    value / 2
}
