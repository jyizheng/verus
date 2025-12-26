use vstd::prelude::*;

verus! {
    // Example 5 (Chapter 3: preventing overflow via requires)
    fn example_sum_with_precondition(x: u8, y: u8)
        requires
            x + y < 256,
    {
        let sum1: u8 = x + y;  // succeeds because of the precondition
        assert(sum1 as nat == (x + y) as nat);
    }
}
