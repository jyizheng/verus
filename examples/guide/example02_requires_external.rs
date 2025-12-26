use vstd::prelude::*;

verus! {
    // Example 2 (Chapter 2: requires/ensures) — preconditions for an external function
    #[verifier::external_body]
    fn print_two_digit_number(i: i8)
        requires
            -99 <= i < 100,
    {
        println!("The answer is {}", i);
    }
}
