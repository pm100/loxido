use std::{
    any::Any,
    ffi::c_void,
    fmt::{self, Debug},
    mem,
};

use crate::gc::{Gc, GcRef, GcTrace};

pub struct ExternalFunction {
    pub funcdef: dyncall::FuncDef,
    pub defstr: GcRef<String>,
}

impl ExternalFunction {
    pub fn new(funcdef: dyncall::FuncDef, defstr: GcRef<String>) -> Self {
        ExternalFunction { funcdef, defstr }
    }
}

#[derive(Clone, Debug)]
pub enum ExternalData {
    Pointer(*mut c_void),
}

impl ExternalData {
    pub fn pointer_value(&self) -> Option<*mut c_void> {
        match self {
            Self::Pointer(ptr) => Some(*ptr),
        }
    }
}

impl GcTrace for ExternalFunction {
    fn format(&self, f: &mut fmt::Formatter, gc: &Gc) -> fmt::Result {
        let name = gc.deref(self.defstr);
        write!(f, "<external function {}>", name)
    }
    fn size(&self) -> usize {
        mem::size_of::<ExternalFunction>()
    }
    fn trace(&self, gc: &mut Gc) {
        gc.mark_object(self.defstr);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl GcTrace for ExternalData {
    fn format(&self, f: &mut fmt::Formatter, _gc: &Gc) -> fmt::Result {
        match self {
            Self::Pointer(ptr) => {
                if ptr.is_null() {
                    write!(f, "ptr(null)")
                } else {
                    write!(f, "ptr({ptr:p})")
                }
            }
        }
    }
    fn size(&self) -> usize {
        mem::size_of::<ExternalData>()
    }
    fn trace(&self, _gc: &mut Gc) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for ExternalFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<external fn>")
    }
}

impl std::fmt::Display for ExternalFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<external fn>")
    }
}
