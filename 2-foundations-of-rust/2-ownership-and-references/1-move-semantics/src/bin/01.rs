fn main() {
    let mut s0 = String::new(); 

    let s1 = append_to_string(&mut s0); 

    println!("{} == `{}`", stringify!(s1), s1); 
    s0.push_str("!");
    println!("{} == `{}`", stringify!(s0), s0);
}

fn append_to_string(s: &mut String) -> String {
    s.push_str("Hello");
    s.push_str(" ");
    s.push_str("World");

    s.clone()
}
