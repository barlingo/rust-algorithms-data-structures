fn main() {
    // copy onto the HEAP in a new place
    let mut s = "  Hello  ".to_string();
    let p = s.trim();
    let p = p.to_string();
    // s is this pointer to memory that will never change.
    // p is a substring of s.
    s.push_str("goodbye");
    println!("=={}==", p);
    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr = {}", ffstr);
    println!("chosen = {}", choose_str(1));
}

fn string_find_f<'a>(s: &'a str) -> &'a str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
    }
}
