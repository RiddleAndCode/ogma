# Ogma

Ogma (named after the [this guy](https://en.wikipedia.org/wiki/Ogma)) is a library
to create Natural Language DSLs. Specifically, the library provides convenience macros
for wrapping a function with implementations to parse parameters from English.

## Examples

```rust
#[given(Add, "the addition of q`input` and d`constant` henceforth q`out`")]
fn add<'a>(
    ctx: &mut Context,
    input: &Vec<Query<'a>>,
    constant: i32,
    out: &Vec<Query<'a>>,
) -> Result<(), Trap> {
    // get global variable from `ctx` using `input`, add `constant` to it
    // and save to `ctx` via `out`
    Ok(())
}
```

which you can then use in a Script

```rust
   let mut ctx = bdd::Step::new();
   let script = Module::compile(
       &mut ctx,
       r#"
       Given the addition of the input and 2 henceforth the left
       And the product of the input and 2 henceforth the right
       When the left is equal to the right
       Then do nothing
       "#,
   )
   .unwrap();
   let mut instance = script.instance();
   instance.ctx_mut().set_global::<_, i32>("input", 2);
   assert!(instance.exec().is_ok());
```

License: MIT OR Apache-2.0
