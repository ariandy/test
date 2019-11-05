fn main() {
    // it's just a sandbox
    prng(&mut 3,4,7,15);
    prng(&mut 2,2,7,10);
    println!("{}", i32::max_value());
    println!("{}", 2u32.pow(31)-1);
    park_miller(&mut 11);
    dec20_fortran(&mut 11);
}

fn prng(seed: &mut i32, mult: i32, inc: i32, m: i32) {
    for _x in 0..10 {
        *seed = (mult * *seed + inc) % m;
        println!("{:?}", seed)
    }
}

fn lehmer(seed: &mut i32, a: i32, m: i32, repeat: i8) {
    for _x in 0..repeat {
        *seed = (a * *seed) % m;
        println!("{:?}", seed)
    }
}

fn park_miller(seed: &mut i32) {
    lehmer(seed, 7i32.pow(5), i32::max_value(), 1)
    //according to PRIMOS, IMSL, SIMPL/I, APL
}

fn dec20_fortran(seed: &mut i32) {
    lehmer(seed, 630360016, i32::max_value(), 1)
    //according to SIMSCRIPT II.5, DEC-20 FORTRAN
}