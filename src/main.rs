use std::mem;

#[derive(Debug)]
struct Bag<T> {
    t: [T; 3],
}

fn BagSize<T>(bag:&Bag<T>) -> usize{
    let mut size = 0;
    for i in bag.t.iter(){
        size += mem::size_of_val(i);
    }
    return size;
}

fn main() {
    let b1 = Bag {t: [1u8, 2u8, 3u8], };
    let b2 = Bag {t: [1u32, 2u32, 3u32], };
    println!("size of First Bag is {:?} bytes ",BagSize(&b1));
    println!("size of Second Bag is {:?} bytes ",BagSize(&b2));
}


