use std::mem;

fn main() {
    let vec1 = vec![12, 32, 13];
    let vec2 = vec![44, 55, 16];
    { 
        let vec1_iter = vec1.iter(); 
        //VecSize);
        println!("{}",mem::size_of_val(&vec1_iter));
    }

    {
        let vec_chained = vec1.iter().chain(vec2.iter());
        println!("{}",mem::size_of_val(&vec_chained));
    }

    { 
        let vec1_2=vec![vec1, vec2];
        let vec_flattened = vec1_2.iter().flatten(); 
        println!("{}",mem::size_of_val(&vec_flattened));
    }
}


