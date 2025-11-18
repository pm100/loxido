use dyncall::ArgVal;

use crate::{
    chunk::Value,
    error::LoxError,
    gc::{Gc, GcRef, GcTrace},
    vm::Vm,
};

// fn exfun(vm: &mut Vm, left: usize) -> Result<Value, LoxError> {
//     let args = &vm.stack[left..];
//     // Example external function that adds two numbers using dyncall
//     if args.len() != 1 {
//         let err = vm
//             .runtime_error("exfun expects exactly 1 arguments")
//             .unwrap_err();
//         return Err(err);
//     }

//     if let Value::String(n) = args[0] {
//         let s = vm.gc.deref(n).clone();
//         let funcdef = vm
//             .dyncaller
//             .borrow_mut()
//             .define_function_by_str(&s)
//             .map_err(|e| {
//                 let msg = format!("Failed to define external function: {}", e);
//                 vm.runtime_error(&msg).unwrap_err()
//             })?;
//         let external = ExternalFunction::new(funcdef, n);

//         let gc = vm.define_external(&s, external);
//         return Ok(Value::ExternalFunction(gc));
//     } else {
//         let x = vm
//             .runtime_error("exfun expects exactly 1 string argument")
//             .unwrap_err();
//         return Err(x);
//     }
// }
use std::{
    any::Any,
    cell::RefCell,
    ffi::os_str::Display,
    fmt::{self, Debug},
    mem,
};
pub struct ExternalFunction {
    pub funcdef: dyncall::FuncDef,
    pub defstr: GcRef<String>,
}

impl ExternalFunction {
    pub fn new(funcdef: dyncall::FuncDef, defstr: GcRef<String>) -> Self {
        ExternalFunction { funcdef, defstr }
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

impl GcTrace for ArgVal {
    fn format(&self, f: &mut fmt::Formatter, _gc: &Gc) -> fmt::Result {
        write!(f, "<dyncall::ArgVal> {:?}", self)
    }
    fn size(&self) -> usize {
        mem::size_of::<ArgVal>()
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
