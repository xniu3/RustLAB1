use std::mem;

#[derive(Debug)]
struct Bag1<u8> {
    t: [u8; 3],
}

fn Bag1Size<u8>(bag:&Bag1<u8>) -> usize{
    let mut size = 0;
    for i in bag.t.iter(){
        size += mem::size_of_val(i);
    }
    return size;
}

#[derive(Debug)]
struct Bag2<u32> {
    t: [u32; 3],
}

fn Bag2Size<u32>(bag:&Bag2<u32>) -> usize{
    let mut size = 0;
    for i in bag.t.iter(){
        size += mem::size_of_val(i);
    }
    return size;
}


fn main() {
    let b1 = Bag1 {t: [1u8, 2u8, 3u8], };
    let b2 = Bag2 {t: [1u32, 2u32, 3u32], };
    println!("size of First Bag is {:?} bytes ",Bag1Size(&b1));
    println!("size of Second Bag is {:?} bytes ",Bag2Size(&b2));
}


