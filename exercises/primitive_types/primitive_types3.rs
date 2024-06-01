// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut a : [i32; 102] = [0; 102];
    let mut b = (0..102);
    while (!b.is_empty()) {
        let c = b.next();
        let d: usize;
        d = match c {
                Option::Some(i32) => c.expect("how"), //implicit type cast to usize because d is usize
                _ => panic!("how"),
        };
        println!("{}", d);

        a[d] = d as i32;
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
