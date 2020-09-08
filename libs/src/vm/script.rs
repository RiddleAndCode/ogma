//! Holds the execution Flow of the virtual machine

use super::context::Context;
use super::func::{Callable, Func};
use super::trap::Trap;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// A list of Functions
#[derive(Default)]
pub struct Script<'a> {
    funcs: Vec<Func<'a>>,
}

/// The current state of the script instance
#[derive(Default)]
pub struct InstanceState {
    ctx: Context,
    pc: usize,
}

/// A executable reference to a Script with an internal state
pub struct Instance<'s, 'a> {
    script: &'s Script<'a>,
    state: InstanceState,
}

impl<'a> Script<'a> {
    /// Create a new empty Script
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an instance of a Script
    #[inline]
    pub fn instance(&self) -> Instance {
        Instance {
            script: self,
            state: InstanceState::default(),
        }
    }

    /// Add a new function to the end of the Script
    #[inline]
    pub fn push(&mut self, func: impl Callable + 'static) {
        self.funcs.push(Box::new(func));
    }
}

impl<'s, 'a> Instance<'s, 'a> {
    /// Step one function down the script
    #[inline]
    pub fn step(&mut self) -> Result<(), Trap> {
        self.cur_func()
            .ok_or(Trap::ScriptOutOfBounds)?
            .call(self.ctx_mut())?;
        self.state.step();
        Ok(())
    }

    /// Step through to the end of the script
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

    /// Reset to the intitial state
    #[inline]
    pub fn reset(&mut self) {
        self.state.reset()
    }

    /// Get the instance context
    #[inline]
    pub fn ctx(&self) -> &Context {
        self.state.ctx()
    }

    /// Get a mutable reference to the context
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
    use crate::vm::func::tests::Add;

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
