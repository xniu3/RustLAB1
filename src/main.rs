#[derive(Debug)]
struct Bag<T> {
    t: T,
}

fn myFunction<T>(arg: T) { 

}


fn main() {
    let b = Bag { t: 42 };
    println!("b is{:?}",b);
}


