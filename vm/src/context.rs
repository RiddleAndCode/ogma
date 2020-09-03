use crate::trap::Trap;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::any::{type_name, Any};
use hashbrown::HashMap;

#[derive(Default)]
pub struct Context {
    pub globals: HashMap<String, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Context::default()
    }

    pub fn set_global<K: ToString, V: Any>(&mut self, key: K, value: V) {
        self.globals.insert(key.to_string(), Box::new(value));
    }

    pub fn get_global<K: AsRef<str>, V: Any>(&self, key: K) -> Result<Option<&V>, Trap> {
        match self.globals.get(key.as_ref()) {
            None => Ok(None),
            Some(val) => match val.downcast_ref::<V>() {
                None => Err(Trap::DowncastError(type_name::<V>())),
                Some(val) => Ok(Some(val)),
            },
        }
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn globals() {
        let mut ctx = Context::new();
        ctx.set_global::<_, u32>("hello", 1);
        assert_eq!(ctx.get_global::<_, u32>("hello").unwrap(), Some(&1))
    }
}
