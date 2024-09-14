#[allow(warnings)]
mod bindings;

use bindings::Guest;
use bindings::name;

struct Toxoid;

impl Guest for Toxoid {
    fn greet() -> String {
        "Hello, ".to_string() + &name()
    }
}

bindings::export!(Toxoid with_types_in bindings);
