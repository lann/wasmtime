use crate::isa::reg::Reg;
use std::collections::VecDeque;

/// Value definition to be used within the shadow stack.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Val {
    /// I32 Constant.
    I32(i32),
    /// A register.
    Reg(Reg),
    /// A local slot.
    Local(u32),
    /// Offset to a memory location.
    Memory(u32),
}

impl Val {
    /// Create a new I32 constant value.
    pub fn i32(v: i32) -> Self {
        Self::I32(v)
    }

    /// Create a new Reg value.
    pub fn reg(r: Reg) -> Self {
        Self::Reg(r)
    }

    /// Create a new Local value.
    pub fn local(index: u32) -> Self {
        Self::Local(index)
    }

    /// Check whether the value is a register.
    pub fn is_reg(&self) -> bool {
        match *self {
            Self::Reg(_) => true,
            _ => false,
        }
    }

    /// Get the register representation of the value.
    ///
    /// # Panics
    /// This method will panic if the value is not a register.
    pub fn get_reg(&self) -> Reg {
        match self {
            Self::Reg(r) => *r,
            v => panic!("expected value {:?} to be a register", v),
        }
    }

    /// Get the integer representation of the value.
    ///
    /// # Panics
    /// This method will panic if the value is not an i32.
    pub fn get_i32(&self) -> i32 {
        match self {
            Self::I32(v) => *v,
            v => panic!("expected value {:?} to be i32", v),
        }
    }

    /// Check whether the value is an i32 constant.
    pub fn is_i32_const(&self) -> bool {
        match *self {
            Self::I32(_) => true,
            _ => false,
        }
    }
}

/// The shadow stack used for compilation.
#[derive(Default, Debug)]
pub(crate) struct Stack {
    inner: VecDeque<Val>,
}

impl Stack {
    /// Allocate a new stack.
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    /// Push a value to the stack.
    pub fn push(&mut self, val: Val) {
        self.inner.push_back(val);
    }

    /// Peek into the top in the stack.
    pub fn peek(&mut self) -> Option<&Val> {
        self.inner.back()
    }

    /// Pops the top element of the stack, if any.
    pub fn pop(&mut self) -> Option<Val> {
        self.inner.pop_back()
    }

    /// Pops the element at the top of the stack if it is a const;
    /// returns `None` otherwise.
    pub fn pop_i32_const(&mut self) -> Option<i32> {
        match self.peek() {
            Some(v) => v.is_i32_const().then(|| self.pop().unwrap().get_i32()),
            _ => None,
        }
    }

    /// Pops the element at the top of the stack if it is a register;
    /// returns `None` otherwise.
    pub fn pop_reg(&mut self) -> Option<Reg> {
        match self.peek() {
            Some(v) => v.is_reg().then(|| self.pop().unwrap().get_reg()),
            _ => None,
        }
    }

    /// Pops the given register if it is at the top of the stack;
    /// returns `None` otherwise.
    pub fn pop_named_reg(&mut self, reg: Reg) -> Option<Reg> {
        match self.peek() {
            Some(v) => (v.is_reg() && v.get_reg() == reg).then(|| self.pop().unwrap().get_reg()),
            _ => None,
        }
    }

    /// Get a mutable reference to the inner stack representation.
    pub fn inner_mut(&mut self) -> &mut VecDeque<Val> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::{Stack, Val};
    use crate::isa::reg::Reg;

    #[test]
    fn test_pop_i32_const() {
        let mut stack = Stack::new();
        stack.push(Val::i32(33i32));
        assert_eq!(33, stack.pop_i32_const().unwrap());

        stack.push(Val::local(10));
        assert!(stack.pop_i32_const().is_none());
    }

    #[test]
    fn test_pop_reg() {
        let mut stack = Stack::new();
        let reg = Reg::int(2usize);
        stack.push(Val::reg(reg));
        stack.push(Val::i32(4));

        assert_eq!(None, stack.pop_reg());
        let _ = stack.pop().unwrap();
        assert_eq!(reg, stack.pop_reg().unwrap());
    }

    #[test]
    fn test_pop_named_reg() {
        let mut stack = Stack::new();
        let reg = Reg::int(2usize);
        stack.push(Val::reg(reg));
        stack.push(Val::reg(Reg::int(4)));

        assert_eq!(None, stack.pop_named_reg(reg));
        let _ = stack.pop().unwrap();
        assert_eq!(reg, stack.pop_named_reg(reg).unwrap());
    }
}
