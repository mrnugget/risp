use std::collections::HashMap;

use crate::object::Object;

pub struct Environment {
    entries: HashMap<String, Object>,
}

impl Environment {
    fn new() -> Environment {
        let env = Environment {
            entries: HashMap::new(),
        };

        // let predefined_functions = &[
        //     ("+", Object::Builtin(object::sum)),
        //     ("*", Object::Builtin(object::multiply)),
        // ];

        // for item in predefined_functions.iter() {
        //     let (name, func) = item;
        //     env.define(name.to_string(), func).unwrap();
        // }

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

    fn get(&self, key: &String) -> &Object {
        match self.entries.get(key) {
            Some(val) => val.clone(),
            None => &Object::Nil,
        }
    }
}

// pub fn apply(proc: &Object, args: Object) -> Object {
//     if let Object::Builtin(func) = proc {
//         return func(args);
//     }
//
//     Object::Nil
// }

pub fn eval<'a>(exp: &'a Object, env: &'a Environment) -> &'a Object {
    match exp {
        Object::Nil | Object::Integer(_) | Object::Builtin(_) => exp,
        Object::Symbol(name) => env.get(name),
        Object::List(ref elems) => &elems[0],
        // Object::List(ref elems) => {
        //     let proc = eval(&elems[0], env);
        //     &apply(proc, &elems[1..elems.len()-1])
        // }
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
            let exp = objects.first().unwrap();

            let env = Environment::new();
            let result = eval(exp.clone(), &env);
            assert_eq!(result, $expected);
        }};
    }

    #[test]
    fn test_self_evaluating() {
        assert_eval!("1", &Object::Integer(1));
    }

    // #[test]
    // fn test_eval_builtins() {
    //     assert_eval!("(+ 1 2 3)", &Object::Integer(6));
    //     assert_eval!("(+ 1 2 3 4 5 6)", &Object::Integer(21));
    //     assert_eval!("(* 2 2 2 2)", &Object::Integer(16));
    // }
}
