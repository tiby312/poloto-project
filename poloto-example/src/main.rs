mod examples;
mod support;
mod tests;
use shower::source;
use std::fmt;

fn main() {
    support::finish(tagu::build::from_stack_escapable(|w| tests::foo(w)));

    //https://docs.rs/syntect/latest/syntect/html/index.html
}
