#[macro_use]
extern crate fn_macro;

use object_query::Query;
use vm::{Context, Trap};

#[ogma_fn(Add, "Given the addition of d`b` and d`a` henceforth q`out`")]
fn add<'a>(ctx: &mut Context, a: i32, b: i32, out: Vec<Query<'a>>) -> Result<(), Trap> {
    Ok(())
}

fn main() {}
