#[allow(unused_imports)]
use compat::*;
use requires_serialize::requires_serialized;
use type_one::TypeOne;

fn main() {
    let test = TypeOne;
    requires_serialized(test);
}
