use crate::evaluator::Object;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type BuiltinFn = fn(Vec<Object>) -> Object;

lazy_static! {
    pub static ref BUILTIN_FUNCTIONS: HashMap<&'static str, BuiltinFn> =
        HashMap::from([("len", len as BuiltinFn)]);
}

fn len(args: Vec<Object>) -> Object {
    let first_obj = args
        .first()
        .expect("len function must be provided a string argument!");

    match first_obj {
        Object::String(str) => Object::Integer(str.len() as i32),
        _ => panic!("len function must be provided a string argument!"),
    }
}
