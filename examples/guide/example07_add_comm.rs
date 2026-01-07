use vstd::prelude::*;

verus! {
    // Example 7 (Chapter 3: simple proof function)
    proof fn example_add_comm(a: int, b: int)
        ensures
            a + b == b + a,
    {
    }
}
