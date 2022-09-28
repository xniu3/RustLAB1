use rand::prelude::*;
use apint::Int;
use apint::ApInt;
use primes::is_prime;// Need this for rng.gen_range(…) function to work.
pub(crate) fn function(n: i32) -> Int {
    let mut rng = rand::thread_rng();
    loop {
        let mut candidate:Int = Int::from(ApInt::from_i32(rng.gen_range(0, n)));
        candidate.set_bit_at(0);
        let mut i = candidate.resize_to_i64() as u64;
        if is_prime(i){
            return candidate
        }
        else{
            println!("{} is not a prime",i);
        }
    }
}