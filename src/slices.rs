fn main() {
    let a = [ 0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in a[]
    // & is a reference symbol probably referencing an addr in memory
    let middle = &a[1..4]; // A slice of a: only printing out 1 2 3 
    println!("let's print our slices: {}", a);
    println!("let's print our slices: {}", complete);
    println!("let's print our slices: {}", middle);
    }
