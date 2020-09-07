use crate::context::Context;
use crate::func::{Callable, Func};
use crate::trap::Trap;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Script<'a> {
    funcs: Vec<Func<'a>>,
}

#[derive(Default)]
pub struct InstanceState {
    ctx: Context,
    pc: usize,
}

pub struct Instance<'s, 'a> {
    script: &'s Script<'a>,
    state: InstanceState,
}

impl<'a> Script<'a> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn instance(&self) -> Instance {
        Instance {
            script: self,
            state: InstanceState::default(),
        }
    }

    #[inline]
    pub fn push(&mut self, func: impl Callable + 'static) {
        self.funcs.push(Box::new(func));
    }
}

impl<'s, 'a> Instance<'s, 'a> {
    #[inline]
    pub fn step(&mut self) -> Result<(), Trap> {
        self.cur_func()
            .ok_or(Trap::ScriptOutOfBounds)?
            .call(self.ctx_mut())?;
        self.state.step();
        Ok(())
    }

    #[inline]
    pub fn exec(&mut self) -> Result<(), Trap> {
        loop {
            match self.step() {
                Ok(()) => {}
                Err(Trap::ScriptOutOfBounds) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.state.reset()
    }

    #[inline]
    pub fn ctx(&self) -> &Context {
        self.state.ctx()
    }

    #[inline]
    pub fn ctx_mut(&mut self) -> &mut Context {
        self.state.ctx_mut()
    }

    #[allow(clippy::borrowed_box)]
    #[inline]
    fn cur_func(&self) -> Option<&'s Func<'a>> {
        self.script.funcs.get(self.state.pc())
    }
}

impl InstanceState {
    #[inline]
    fn step(&mut self) {
        self.pc += 1;
    }

    #[inline]
    fn pc(&self) -> usize {
        self.pc
    }

    #[inline]
    fn reset(&mut self) {
        *self = Self::default()
    }

    #[inline]
    fn ctx(&self) -> &Context {
        &self.ctx
    }

    #[inline]
    fn ctx_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }
}

impl<'a> From<Vec<Func<'a>>> for Script<'a> {
    fn from(funcs: Vec<Func<'a>>) -> Self {
        Self { funcs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::func::tests::Add;

    #[test]
    fn exec() {
        let mut script = Script::new();
        script.push(Add("a", "b"));
        script.push(Add("c", "a"));
        script.push(Add("c", "c"));
        let mut instance = script.instance();
        instance.ctx_mut().set_global::<_, i32>("a", 1);
        instance.ctx_mut().set_global::<_, i32>("b", 1);
        instance.exec().unwrap();
        assert_eq!(instance.ctx().get_global::<_, i32>("c").unwrap(), Some(&6));
    }
}
