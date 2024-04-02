fn main() {
    println!("Hello, world!");
}

#[allow(unused)]
fn foo(i: i32) {
    match i {
        0 => println!("max_width is 60 chars,       here."),
        1 => println!("1st argument: {}", "2nd argument __"),
        2 => println!("1st argument: {}", "2nd argument ___"),
        3 => println!("1st argument: {}", "2nd argument ____"),
        _ => {},
    }
}

/*
Observed state after `cargo +nightly fmt`:

fn foo(i: i32) {
    match i {
        0 => println!("max_width is 60 chars,       here."),
        1 =>
            println!("1st argument: {}", "2nd argument __"),
        2 =>
            println!("1st argument: {}", "2nd argument ___"),
        3 => println!(
            "1st argument: {}",
            "2nd argument ____"
        ),
        _ => {},
    }
}

This will emit a `line formatted` error on line 12 (and 30).
If the `2 =>` branch moved the `println!` arguments into a
vertical style -- as the `3 =>` branch does -- the `line
formatted` error wouldn't occur.
*/