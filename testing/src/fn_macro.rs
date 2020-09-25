use crate::error::Fallible;
use alloc::string::ToString;
use alloc::vec::Vec;
use ogma::module::ModuleType;
use ogma::object_query::Query;
use ogma::vm::{Context, Trap};

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

type Module<'a> = mod_type!(Add<'a>);

#[cfg_attr(feature = "std", test)]
#[cfg_attr(not(feature = "std"), test_case)]
fn test_add() -> Fallible<()> {
    let mut ctx = ();
    let script = Module::compile(
        &mut ctx,
        r#"Given the addition of the input and 4 henceforth the output"#,
    )
    .unwrap();
    let mut instance = script.instance();
    instance.ctx_mut().set_global::<_, i32>("input", 3);
    instance.exec().unwrap();
    let out = instance.ctx().get_global::<_, i32>("output").unwrap();
    assert_eq!(out, Some(&7));
    Ok(())
}
