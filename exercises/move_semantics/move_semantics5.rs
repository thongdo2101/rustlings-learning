// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

// I AM NOT DONE

fn main() {
    let mut x = 100;
    let mut y = x;
    let mut z = y;
    y += 100;
    z += 1000;
    assert_eq!(x, 1200);
}
