use p3_koala_bear::{KoalaBear, default_koalabear_poseidon2_24};
use p3_symmetric::Permutation;

fn main() {
    // Basic field operations
    let a = KoalaBear::new(42);
    let b = KoalaBear::new(7);

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("a + b = {:?}", a + b);
    println!("a * b = {:?}", a * b);
    println!();

    // Use Poseidon2 permutation
    let perm = default_koalabear_poseidon2_24();
    let mut state = [KoalaBear::new(0); 24];
    state[0] = a;
    state[1] = b;

    perm.permute_mut(&mut state);

    println!("Poseidon2 hash of [a, b, 0, ...]: {:?}", &state[..3]);
}
