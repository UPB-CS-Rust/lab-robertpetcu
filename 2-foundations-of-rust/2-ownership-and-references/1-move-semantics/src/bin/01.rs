fn main() {
    let s0 = String::new();

    let mut s1 = append_to_string(s0.clone());

    println!("{} == `{}`", stringify!(s1), s1);

    s0.push_str("!");

    println!("{} == `{}`", stringify!(s1), s1);
}

fn append_to_string(s:&mut String) -> String {
    let mut s = s;

    s.push_str("Hello");
    s.push_str(" ");
    s.push_str("World");

    s
}
