use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::object;
use crate::object::Object;

pub struct Environment {
    entries: HashMap<String, Rc<Object>>,
}

impl Environment {
    fn new() -> Environment {
        let mut env = Environment {
            entries: HashMap::new(),
        };

        let predefined_functions = &[("+", Rc::new(Object::Builtin(object::sum)))];

        for item in predefined_functions.iter() {
            let (name, ref func) = *item;
            env.define(name.to_string(), func.clone()).unwrap();
        }

        env
    }

    fn define(&mut self, key: String, obj: Rc<Object>) -> Result<(), ()> {
        if self.entries.contains_key(&key) {
            // TODO: we need a real error here
            return Err(());
        } else {
            self.entries.insert(key, obj);
            Ok(())
        }
    }

    fn get(&self, key: &String) -> Rc<Object> {
        match self.entries.get(key) {
            Some(val) => val.clone(),
            None => Rc::new(Object::Nil),
        }
    }
}

pub fn apply(proc: Rc<Object>, args: Rc<Object>) -> Rc<Object> {
    if let Object::Builtin(func) = proc.deref() {
        return func(args);
    }

    Rc::new(Object::Nil)
}

pub fn eval(exp: Rc<Object>, env: &Environment) -> Rc<Object> {
    match exp.deref() {
        Object::Nil | Object::Integer(_) | Object::Builtin(_) => exp,
        Object::Symbol(name) => env.get(name),
        Object::Pair(car, cdr) => {
            let proc = eval(car.clone(), env);
            apply(proc, cdr.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use crate::reader;

    #[test]
    fn test_eval_plus() {
        let objects = reader::read("(+ 1 2 3)").unwrap();
        assert_eq!(objects.len(), 1);

        let exp = objects.first().unwrap();
        assert!(exp.is_pair());

        let env = Environment::new();
        let result = eval(exp.clone(), &env);
        assert!(result.is_integer());
        assert_eq!(*result, Object::Integer(6));
    }
}
