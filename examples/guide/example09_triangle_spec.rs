use vstd::prelude::*;

verus! {
    // Example 9 (Chapter 4: recursive spec function)
    spec fn triangle(n: nat) -> nat
        decreases n,
    {
        if n == 0 { 0 } else { n + triangle((n - 1) as nat) }
    }

    fn example_triangle_values() {
        assert(triangle(0) == 0);
        assert(triangle(1) == 1);
        assert(triangle(2) == 3);
        assert(triangle(3) == 6);
        assert(triangle(4) == 10);
        assert(triangle(5) == 15);
        assert(triangle(6) == 21);
        assert(triangle(7) == 28);
        assert(triangle(8) == 36);
        assert(triangle(9) == 45);
        assert(triangle(10) == 55);
    }
}
