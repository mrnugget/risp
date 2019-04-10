use crate::object::EnvRef;
use crate::object::Environment;
use crate::object::Function;
use crate::object::Object;

pub fn apply(proc: &Object, args: &[Object], env: EnvRef) -> Object {
    if let Object::Callable(func) = proc {
        return match func {
            Function::Native(builtin) => builtin(args, env),
            Function::Lambda(parameters, body, lambda_env) => {
                let application_env = Environment::new_child(lambda_env.clone());
                for (i, p) in parameters.iter().enumerate() {
                    if let Object::Symbol(name) = p {
                        let result = application_env
                            .borrow_mut()
                            .define(name.to_string(), args[i].clone());

                        if let Err(_) = result {
                            return Object::Error(String::from(format!(
                                "failed to define {} in env",
                                name
                            )));
                        }
                    }
                }

                body.iter()
                    .map(|e| eval(e.clone(), application_env.clone()))
                    .last()
                    .unwrap()
            }
        };
    }

    Object::Error(String::from("cannot call non-function"))
}

pub fn eval(exp: Object, env: EnvRef) -> Object {
    match exp {
        Object::Nil | Object::Integer(_) | Object::Callable(_) | Object::Error(_) => exp,
        Object::Symbol(name) => env.borrow().get(&name),
        Object::List(elems) => {
            if is_definition(&elems) {
                return make_definition(&elems, env.clone());
            }

            if is_lambda(&elems) {
                return make_lambda(&elems, env.clone());
            }

            let mut iter = elems.into_iter();
            let proc = eval(iter.next().unwrap(), env.clone());
            let args = iter
                .map(|a| eval(a.clone(), env.clone()))
                .collect::<Vec<Object>>();
            apply(&proc, &args, env.clone())
        }
    }
}

fn is_lambda(exps: &[Object]) -> bool {
    match exps.first() {
        Some(&Object::Symbol(ref name)) => name == "lambda",
        _ => false,
    }
}

fn make_lambda(exps: &[Object], env: EnvRef) -> Object {
    let args = match &exps[1] {
        Object::List(args) => args.clone(),
        _ => return Object::Error(String::from("arguments are not a list")),
    };

    let body = vec![exps[2].clone()];

    Object::Callable(Function::Lambda(args, body, env.clone()))
}

fn is_definition(exps: &[Object]) -> bool {
    match exps.first() {
        Some(&Object::Symbol(ref name)) => name == "define",
        _ => false,
    }
}

fn make_definition(exps: &[Object], env: EnvRef) -> Object {
    let name = match &exps[1] {
        Object::Symbol(name) => name.to_string(),
        _ => return Object::Error(String::from("argument has wrong type")),
    };

    let value = eval(exps[2].clone(), env.clone());

    env.borrow_mut().define(name.to_string(), value).unwrap();

    Object::Nil
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
        assert_eval!("(- 3 2 1)", Object::Integer(0));
        assert_eval!("(* 2 2 2)", Object::Integer(8));
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
    fn test_eval_builtin_car() {
        assert_eval!("(car (cons 1 2))", Object::Integer(1));
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

    #[test]
    fn test_lambdas() {
        assert_eval!("((lambda (x) (+ x 1)) 2)", Object::Integer(3));
        assert_eval!("((lambda (a b c) (+ a b c)) 1 2 3)", Object::Integer(6));
    }
}
