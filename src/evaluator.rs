use std::cell::RefCell;
use std::rc::Rc;

use crate::object::Environment;
use crate::object::Function;
use crate::object::Object;

pub fn apply(proc: &Object, args: &[Object], env: Rc<RefCell<Environment>>) -> Object {
    if let Object::Callable(func) = proc {
        let Function::Native(builtin) = func;
        return builtin(args, env);
    }

    Object::Error(String::from("cannot call non-function"))
}

pub fn eval(exp: Object, env: Rc<RefCell<Environment>>) -> Object {
    match exp {
        Object::Nil | Object::Integer(_) | Object::Callable(_) | Object::Error(_) => exp,

        Object::Symbol(name) => {
            let val = env.borrow().get(&name);
            val
        }
        Object::List(elems) => {
            let mut iter = elems.into_iter();
            let proc = eval(iter.next().unwrap(), env.clone());
            let args = iter.collect::<Vec<Object>>();
            apply(&proc, &args, env.clone())
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
            let env = Environment::new();

            let objects = reader::read($input).unwrap();

            let mut result = Object::Nil;
            for exp in objects.into_iter() {
                result = eval(exp, env.clone())
            }

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
    fn test_eval_builtin_cons() {
        assert_eval!(
            "(cons 1 2)",
            Object::List(vec![Object::Integer(1), Object::Integer(2)])
        );
    }

    #[test]
    fn test_eval_applying_non_callable() {
        assert_eval!(
            "(1)",
            Object::Error(String::from("cannot call non-function"))
        );
    }

    #[test]
    fn test_definitions() {
        assert_eval!(
            "(define foobar 15)
foobar",
            Object::Integer(15)
        );

        assert_eval!(
            "(define foobar (+ 1 2 3 4))
        foobar",
            Object::Integer(10)
        );
    }
}
