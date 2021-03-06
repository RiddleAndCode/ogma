//! The callable code units of a script in the Virtual Machine

use super::context::Context;
use super::trap::Trap;
use alloc::boxed::Box;

/// A Callable Type
pub type Func<'a> = Box<dyn Callable + 'a>;

/// Given a Context can call itself and modify the Context
pub trait Callable {
    fn call(&self, ctx: &mut Context) -> Result<(), Trap>;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use alloc::string::ToString;

    pub struct Add(pub &'static str, pub &'static str);

    impl Callable for Add {
        fn call(&self, ctx: &mut Context) -> Result<(), Trap> {
            let a = *ctx
                .get_global::<_, i32>(self.0)?
                .ok_or_else(|| Trap::MissingGlobal(self.0.to_string()))?;
            let b = *ctx
                .get_global::<_, i32>(self.1)?
                .ok_or_else(|| Trap::MissingGlobal(self.1.to_string()))?;
            ctx.set_global::<_, i32>("c", a + b);
            Ok(())
        }
    }

    #[test]
    fn add() {
        let mut ctx = Context::new();
        ctx.set_global::<_, i32>("a", 1);
        ctx.set_global::<_, i32>("b", 2);
        Add("a", "b").call(&mut ctx).unwrap();
        assert_eq!(ctx.get_global::<_, i32>("c").unwrap(), Some(&3));
    }
}
