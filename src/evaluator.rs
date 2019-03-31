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

        let native_functions = &[
            ("+", Function::Native(object::plus)),
            ("*", Function::Native(object::multiply)),
            ("list", Function::Native(object::list)),
        ];

        for item in native_functions.into_iter() {
            let (name, ref func) = item;
            env.define(name.to_string(), Object::Callable(func.clone()))
                .unwrap();
        }

        env
    }

    fn define(&mut self, key: String, obj: Object) -> Result<(), ()> {
        self.entries.insert(key, obj);
        Ok(())
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
        let Function::Native(builtin) = func;
        return builtin(args);
    }

    Object::Error(String::from("cannot call non-function"))
}

pub fn eval<'a>(exp: Object, env: &'a Environment) -> Object {
    match exp {
        Object::Nil | Object::Integer(_) | Object::Callable(_) | Object::Error(_) => exp,

        Object::Symbol(name) => env.get(&name),
        Object::List(elems) => {
            let mut iter = elems.into_iter();
            let proc = eval(iter.next().unwrap(), env);
            let args = iter.collect::<Vec<Object>>();
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
    fn test_eval_builtin_arithmetic() {
        assert_eval!("(+ 1 2 3)", Object::Integer(6));
        assert_eval!("(+ 1 2 3 4 5 6)", Object::Integer(21));
        assert_eval!("(* 2 2 2 2)", Object::Integer(16));
    }

    #[test]
    fn test_eval_builtin_list() {
        assert_eval!(
            "(list 1 2 3)",
            Object::List(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3)
            ])
        );
    }

    #[test]
    fn test_eval_applying_non_callable() {
        assert_eval!(
            "(1)",
            Object::Error(String::from("cannot call non-function"))
        );
    }
}
