#[macro_use]
extern crate fn_macro;

use object_query::Query;
use vm::{Context, Trap};

#[ogma_fn(#[derive(Debug)] Add, "Given the addition of q`input` and d`b` henceforth q`out`")]
fn add<'a>(
    ctx: &mut Context,
    input: &Vec<Query<'a>>,
    b: i32,
    out: &Vec<Query<'a>>,
) -> Result<(), Trap> {
    let input = input.iter().next().unwrap().as_key().unwrap();
    let out = out.iter().next().unwrap().as_key().unwrap();
    let a = ctx
        .get_global::<_, i32>(input)?
        .ok_or_else(|| Trap::MissingGlobal(input.to_string()))?;
    let c = a + b;
    ctx.set_global::<_, i32>(out, c);
    Ok(())
}

fn main() {}
