//! A simple program to be proven inside the zkVM.
//! Consists in a 5 question multiple choice quiz
//! with 3 possible answers each.

#![no_main]

use tiny_keccak::{Hasher, Sha3};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let answers = sp1_zkvm::io::read::<String>();
    let mut sha3 = Sha3::v256();
    let mut output = [0u8; 32];

    sha3.update(answers.as_bytes());

    sha3.finalize(&mut output);

    if output
        != [216, 11, 193, 177, 136, 178, 254, 150, 59, 128, 97, 103, 97, 128, 55, 57, 22, 242, 26,
             1, 186, 223, 215, 118, 206, 47, 12, 206, 114, 118, 220, 93
            ]
    {
        panic!("Answers do not match");
    }
}
