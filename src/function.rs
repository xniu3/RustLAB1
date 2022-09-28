use rand::prelude::*;
use num_bigint::BigInt;
use primes::is_prime;// Need this for rng.gen_range(…) function to work.

pub(crate) fn function(n: i32) -> BigInt {
    let mut rng = rand::thread_rng();
    loop {
        //let mut candidate:Int = Int::from(ApInt::from_i32(rng.gen_range(0, n)));
        
        let mut candidate = BigInt::from(rng.gen_range(0, n));
        candidate.set_bit(0,true);
        let mut i = candidate.to_string().parse::<u64>().unwrap();
        if is_prime(i){
            return candidate
        }
        else{
            println!("{} is not a prime",i);
        }
    }
}