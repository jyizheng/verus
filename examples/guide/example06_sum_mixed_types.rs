use vstd::prelude::*;

verus! {
    // Example 6 (Chapter 3: integer casts in ghost code)
    proof fn example_sum_mixed_types(x: u8, y: u16) {
        assert((x as nat) + (y as nat) >= (y as nat));
        assert((x as int) - (y as int) <= x as int);
    }
}
