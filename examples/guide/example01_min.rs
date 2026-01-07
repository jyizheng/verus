use vstd::prelude::*;

verus! {
    // Example 1 (Chapter 1: Getting started) — basic spec function and assertions
    spec fn min(x: int, y: int) -> int {
        if x <= y { x } else { y }
    }

    fn example_min_properties() {
        assert(min(10, 20) == 10);
        assert(min(-10, -20) == -20);
        assert(forall|i: int, j: int| min(i, j) <= i && min(i, j) <= j);
        assert(forall|i: int, j: int| min(i, j) == i || min(i, j) == j);
    }
}
