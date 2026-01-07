use vstd::prelude::*;

verus! {
    // Example 8 (Chapter 3: reasoning about specs)
    spec fn min(x: int, y: int) -> int {
        if x <= y { x } else { y }
    }

    proof fn example_min_comm(x: int, y: int)
        ensures
            min(x, y) == min(y, x),
    {
    }
}
