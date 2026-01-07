use vstd::prelude::*;

verus! {
    // Example 4 (Chapter 3: specification expressions) — mixing integer kinds safely
    fn example_int_constants() {
        let u: u8 = 1u8;
        assert({
            let i: int = 2int;
            let n: nat = 3nat;
            0int <= u < i < n < 4int
        });
    }
}
