fn main() {
    diverges();
}

fn diverges() -> ! {
    panic!("This funcion never returns!");
 }
