fn main() {
    prng(&mut 3,4,7,15);
    prng(&mut 2,2,7,10);
}

fn prng(seed: &mut i32, mult: i32, inc: i32, m: i32) {
    for _x in 0..10 {
        *seed = (mult * *seed + inc) % m;
        println!("{:?}", seed)
    }
}