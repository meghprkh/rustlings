// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    // M1, M2: let vec0 = Vec::new();
    let mut vec0 = Vec::new();

    // M1: let mut vec1 = fill_vec(Vec::clone(&vec0));
    // M2: let mut vec1 = fill_vec(&vec0);
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // Commented in M3:
    // vec1.push(88);

    // Commented in M3:
    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// M1: fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
// M2: fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
