mod rust_math;

use crate::rust_math::*;

fn main() {
    println!("call rust by third lib: {}", treble(10));

    println!("call rust by include file : {}", rust_math::treble(10))
}
