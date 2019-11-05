fn main() {
    // prng(&mut 3,4,7,15);
    // prng(&mut 2,2,7,10);
    println!("{}", i32::max_value());
    pml(&mut 11111111)
}

fn prng(seed: &mut i32, mult: i32, inc: i32, m: i32) {
    for _x in 0..10 {
        *seed = (mult * *seed + inc) % m;
        println!("{:?}", seed)
    }
}

fn lcg(seed: &mut i32, a: i32, m: i32) {
    for _x in 0..10 {
        *seed = (a * *seed) % m;
        println!("{:?}", seed)
    }
}

fn pml(seed: &mut i32) {
    lcg(&mut 726347, 7^5, i32::max_value()-1)
    //according to PRIMOS, IMSL, SIMPL/I, APL, 
}