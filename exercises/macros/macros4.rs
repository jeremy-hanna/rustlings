// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

macro_rules! my_macro {
    ($val:tt) => {
        println!("Look at this other macro: {}", $val);
    };
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
