use vstd::prelude::*;

verus! {
    // Example 10 (Chapter 4: recursive exec function linked to spec)
    spec fn triangle(n: nat) -> nat
        decreases n,
    {
        if n == 0 { 0 } else { n + triangle((n - 1) as nat) }
    }

    fn example_triangle_exec(n: u32) -> (sum: u32)
        requires
            triangle(n as nat) < 0x1_0000_0000,
        ensures
            sum == triangle(n as nat),
        decreases n,
    {
        if n == 0 {
            0
        } else {
            n + example_triangle_exec(n - 1)
        }
    }
}
