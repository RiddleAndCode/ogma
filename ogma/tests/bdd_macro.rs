#[macro_use]
extern crate ogma;

use ogma::bdd;
use ogma::module::{Module as ModuleTrait, ModuleList, ModuleType};
use ogma::object_query::Query;
use ogma::vm::{Context, Trap};

#[given(Add, "the addition of q`input` and d`b` henceforth q`out`")]
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

#[given(Sub, "the difference of q`input` and d`b` henceforth q`out`")]
fn sub<'a>(
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
    let c = a - b;
    ctx.set_global::<_, i32>(out, c);
    Ok(())
}

#[when(Equals, "q`left` is equal to q`right`")]
fn equals<'a>(
    ctx: &mut Context,
    left: &Vec<Query<'a>>,
    right: &Vec<Query<'a>>,
) -> Result<(), Trap> {
    let left = left.iter().next().unwrap().as_key().unwrap();
    let right = right.iter().next().unwrap().as_key().unwrap();
    let a = ctx
        .get_global::<_, i32>(left)?
        .ok_or_else(|| Trap::MissingGlobal(left.to_string()))?;
    let b = ctx
        .get_global::<_, i32>(right)?
        .ok_or_else(|| Trap::MissingGlobal(right.to_string()))?;
    if a != b {
        Err(Trap::runtime("left not equal to right"))
    } else {
        Ok(())
    }
}

#[then(Noop, "do nothing")]
fn noop(_: &mut Context) -> Result<(), Trap> {
    Ok(())
}

type Module<'a> = mod_type!(Add<'a>, Sub<'a>, Equals<'a>, Noop);

fn module<'a>() -> ModuleList<'a, bdd::Step> {
    mod_list!(bdd::Step => Add, Sub, Equals, Noop)
}

#[test]
fn test_given_add() {
    let mut ctx = bdd::Step::new();
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
}

#[test]
fn test_given_add_extra_fail() {
    let mut ctx = bdd::Step::new();
    assert!(Module::compile(
        &mut ctx,
        r#"Given the addition of the input and 4 henceforth the output extra"#,
    )
    .is_err());
}

#[test]
fn test_bdd() {
    let mut ctx = bdd::Step::new();
    let script = Module::compile(
        &mut ctx,
        r#"
        Given the addition of the input and 4 henceforth the left
        And the difference of the input and -4 henceforth the right
        When the left is equal to the right
        Then do nothing
        "#,
    )
    .unwrap();
    let mut instance = script.instance();
    instance.ctx_mut().set_global::<_, i32>("input", 3);
    instance.exec().unwrap();
    let left = instance.ctx().get_global::<_, i32>("left").unwrap();
    assert_eq!(left, Some(&7));
    let right = instance.ctx().get_global::<_, i32>("right").unwrap();
    assert_eq!(right, Some(&7));
}

#[test]
fn test_mod_list() {
    let mut ctx = bdd::Step::new();
    let script = module()
        .compile(
            &mut ctx,
            r#"
        Given the addition of the input and 4 henceforth the left
        And the difference of the input and -4 henceforth the right
        When the left is equal to the right
        Then do nothing
        "#,
        )
        .unwrap();
    let mut instance = script.instance();
    instance.ctx_mut().set_global::<_, i32>("input", 3);
    instance.exec().unwrap();
    let left = instance.ctx().get_global::<_, i32>("left").unwrap();
    assert_eq!(left, Some(&7));
    let right = instance.ctx().get_global::<_, i32>("right").unwrap();
    assert_eq!(right, Some(&7));
}
