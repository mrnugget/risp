use std::collections::HashMap;

use crate::object;
use crate::object::Function;
use crate::object::Object;

pub struct Environment {
    entries: HashMap<String, Object>,
}

impl Environment {
    fn new() -> Environment {
        let mut env = Environment {
            entries: HashMap::new(),
        };

        env.define(
            String::from("+"),
            Object::Callable(Function::Native(object::sum)),
        );
        env.define(
            String::from("*"),
            Object::Callable(Function::Native(object::multiply)),
        );

        env
    }

    fn define(&mut self, key: String, obj: Object) -> Result<(), ()> {
        if self.entries.contains_key(&key) {
            // TODO: we need a real error here
            return Err(());
        } else {
            self.entries.insert(key, obj);
            Ok(())
        }
    }

    fn get(&self, key: &String) -> Object {
        match self.entries.get(key) {
            Some(val) => val.clone(),
            None => Object::Nil,
        }
    }
}

pub fn apply(proc: &Object, args: &[Object]) -> Object {
    if let Object::Callable(func) = proc {
        println!("here we are!");
        let Function::Native(builtin) = func;
        return builtin(args);
    }

    // TODO: this should be an error
    Object::Nil
}

pub fn eval<'a>(exp: Object, env: &'a Environment) -> Object {
    match exp {
        Object::Nil | Object::Integer(_) | Object::Callable(_) => exp,

        Object::Symbol(name) => env.get(&name),
        Object::List(elems) => {
            let mut iter = elems.into_iter();
            let first = iter.next().unwrap();
            let args = iter.collect::<Vec<Object>>();
            let proc = eval(first, env);
            apply(&proc, &args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use crate::reader;

    macro_rules! assert_eval {
        ( $input:expr, $expected:expr ) => {{
            let objects = reader::read($input).unwrap();
            assert_eq!(objects.len(), 1);
            let exp = objects.into_iter().next().unwrap();

            let env = Environment::new();
            let result = eval(exp, &env);
            assert_eq!(result, $expected);
        }};
    }

    #[test]
    fn test_self_evaluating() {
        assert_eval!("15", Object::Integer(15));
    }

    #[test]
    fn test_eval_builtins() {
        assert_eval!("(+ 1 2 3)", Object::Integer(6));
        assert_eval!("(+ 1 2 3 4 5 6)", Object::Integer(21));
        assert_eval!("(* 2 2 2 2)", Object::Integer(16));
    }
}
