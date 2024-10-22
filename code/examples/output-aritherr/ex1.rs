use vstd::prelude::*;
fn main() {}
verus! {
fn myfun(x: i32, y: i32) -> (r: i32)
  requires
    x < 10000,
    y < 10000,
    0 < x,
    0 < y,
{
    let mut i: usize = 0;
    let mut z: i32 = 0;
    while i < 4
        invariant
            x < 10000, // Added by AI
            y < 10000, // Added by AI
            0 < x, // Added by AI
            0 < y, // Added by AI
            0 <= i,
            i <= 4,
    {
      z = x + y;
      i += 1;
    }
    z
}
}
