//! Holds the mutable state of the Virtual Machine

use super::trap::Trap;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::any::{type_name, Any};
use hashbrown::HashMap;

/// Virtual machine context
#[derive(Default)]
pub struct Context {
    /// The global variables queryable by name
    pub globals: HashMap<String, Box<dyn Any>>,
}

impl Context {
    /// Create an empty context
    pub fn new() -> Self {
        Context::default()
    }

    /// Set a global variable
    pub fn set_global<K: ToString, V: Any>(&mut self, key: K, value: V) {
        self.globals.insert(key.to_string(), Box::new(value));
    }

    /// This does the same thing as `set_global` but attempts to return the variable which was replaced
    pub fn replace_global<K: ToString, V: Any, R: Any>(
        &mut self,
        key: K,
        value: V,
    ) -> Result<Option<Box<R>>, Trap> {
        match self.globals.insert(key.to_string(), Box::new(value)) {
            None => Ok(None),
            Some(val) => Ok(Some(
                val.downcast()
                    .map_err(|_| Trap::DowncastError(type_name::<R>()))?,
            )),
        }
    }

    /// Get a global variable
    pub fn get_global<K: AsRef<str>, V: Any>(&self, key: K) -> Result<Option<&V>, Trap> {
        match self.globals.get(key.as_ref()) {
            None => Ok(None),
            Some(val) => match val.downcast_ref::<V>() {
                None => Err(Trap::DowncastError(type_name::<V>())),
                Some(val) => Ok(Some(val)),
            },
        }
    }

    /// Get a mutable reference to a global variable
    pub fn get_global_mut<K: AsRef<str>, V: Any>(
        &mut self,
        key: K,
    ) -> Result<Option<&mut V>, Trap> {
        match self.globals.get_mut(key.as_ref()) {
            None => Ok(None),
            Some(val) => match val.downcast_mut::<V>() {
                None => Err(Trap::DowncastError(type_name::<V>())),
                Some(val) => Ok(Some(val)),
            },
        }
    }

    /// Delete a global variable
    pub fn delete_global<K: AsRef<str>>(&mut self, key: K) {
        self.globals.remove(key.as_ref());
    }

    /// Does the same thing as `delete_global` but attempts to return the removed variable
    pub fn remove_global<K: AsRef<str>, R: Any>(&mut self, key: K) -> Result<Option<Box<R>>, Trap> {
        match self.globals.remove(key.as_ref()) {
            None => Ok(None),
            Some(val) => Ok(Some(
                val.downcast()
                    .map_err(|_| Trap::DowncastError(type_name::<R>()))?,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_global() -> Result<(), Trap> {
        let mut ctx = Context::new();
        ctx.set_global::<_, u32>("hello", 1);
        assert_eq!(ctx.get_global::<_, u32>("hello")?, Some(&1));
        assert_eq!(ctx.get_global::<_, u32>("cool")?, None);
        Ok(())
    }

    #[test]
    fn get_global_mut() -> Result<(), Trap> {
        let mut ctx = Context::new();
        ctx.set_global::<_, u32>("hello", 1);
        let num = ctx.get_global_mut::<_, u32>("hello")?.unwrap();
        *num = 2;
        assert_eq!(ctx.get_global::<_, u32>("hello")?, Some(&2));
        Ok(())
    }

    #[test]
    fn delete_global() -> Result<(), Trap> {
        let mut ctx = Context::new();
        ctx.set_global::<_, u32>("hello", 1);
        ctx.delete_global("hello");
        assert_eq!(ctx.get_global::<_, u32>("hello")?, None);
        Ok(())
    }

    #[test]
    fn remove_global() -> Result<(), Trap> {
        let mut ctx = Context::new();
        ctx.set_global::<_, u32>("hello", 1);
        assert_eq!(ctx.remove_global::<_, u32>("hello")?, Some(Box::new(1)));
        assert_eq!(ctx.get_global::<_, u32>("hello")?, None);
        assert_eq!(ctx.remove_global::<_, u32>("hello")?, None);
        assert_eq!(ctx.get_global::<_, u32>("hello")?, None);
        Ok(())
    }

    #[test]
    fn replace_global() -> Result<(), Trap> {
        let mut ctx = Context::new();
        assert_eq!(ctx.replace_global::<_, u32, u32>("hello", 1)?, None);
        assert_eq!(ctx.get_global::<_, u32>("hello")?, Some(&1));
        assert_eq!(
            ctx.replace_global::<_, u32, u32>("hello", 2)?,
            Some(Box::new(1))
        );
        assert_eq!(ctx.get_global::<_, u32>("hello")?, Some(&2));
        Ok(())
    }
}
