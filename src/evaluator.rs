use crate::object::{EnvRef, Environment, Function, Object};

fn apply_lambda(lambda: &Function, args: &[Object]) -> Result<Object, Object> {
    if let Function::Lambda(parameters, body, lambda_env) = lambda {
        let application_env = Environment::new_child(lambda_env.clone());

        for (i, p) in parameters.iter().enumerate() {
            if let Object::Symbol(name) = p {
                let result = application_env
                    .borrow_mut()
                    .define(name.to_string(), args[i].clone());

                if result.is_err() {
                    return Err(Object::new_error(&format!(
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
    } else {
        Err(Object::new_error(&format!(
            "lambda is not a Function::Lambda: {}",
            lambda
        )))
    }
}

pub fn apply(proc: &Object, args: &[Object], env: EnvRef) -> Result<Object, Object> {
    match proc {
        Object::Callable(func) => match func {
            Function::Native(builtin) => builtin(args, env),
            Function::Lambda(_, _, _) => apply_lambda(func, args),
        },
        _ => Err(Object::new_error("cannot call non-function")),
    }
}

pub fn eval(exp: Object, env: EnvRef) -> Result<Object, Object> {
    match exp {
        Object::Nil | Object::Integer(_) | Object::Callable(_) | Object::Error(_) => Ok(exp),
        Object::Symbol(name) => Ok(env.borrow().get(&name)),
        Object::List(elems) => {
            if is_definition(&elems) {
                return make_definition(&elems, env.clone());
            }

            if is_lambda(&elems) {
                return make_lambda(&elems, env.clone());
            }

            let mut iter = elems.into_iter();
            let proc = eval(iter.next().unwrap(), env.clone())?;

            let mut args: Vec<Object> = Vec::new();
            for a in iter {
                let result = eval(a.clone(), env.clone())?;
                args.push(result)
            }

            apply(&proc, &args, env.clone())
        }
    }
}

fn is_lambda(exps: &[Object]) -> bool {
    match exps.first().and_then(|o| o.has_symbol_value("lambda")) {
        Some(b) => b,
        None => false,
    }
}

fn make_lambda(exps: &[Object], env: EnvRef) -> Result<Object, Object> {
    let args = match &exps[1] {
        Object::List(args) => args.clone(),
        _ => return Err(Object::new_error("arguments are not a list")),
    };

    let body = vec![exps[2].clone()];
    let lambda = Object::Callable(Function::Lambda(args, body, env.clone()));
    Ok(lambda)
}

fn is_definition(exps: &[Object]) -> bool {
    match exps.first().and_then(|o| o.has_symbol_value("define")) {
        Some(b) => b,
        None => false,
    }
}

fn make_definition(exps: &[Object], env: EnvRef) -> Result<Object, Object> {
    let name = match &exps[1] {
        Object::Symbol(name) => name.to_string(),
        _ => return Err(Object::new_error("argument has wrong type")),
    };

    let value = eval(exps[2].clone(), env.clone())?;

    env.borrow_mut()
        .define(name.to_string(), value)
        .and_then(|_| Ok(Object::Nil))
        .or_else(|e| Err(Object::new_error(&format!("defining failed: {}", e))))
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

            let mut result: Result<Object, Object> = Ok(Object::Nil);
            for exp in objects.into_iter() {
                result = eval(exp, env.clone())
            }

            assert_eq!(result, $expected);
        }};
    }

    #[test]
    fn test_self_evaluating() {
        assert_eval!("15", Ok(Object::Integer(15)));
    }

    #[test]
    fn test_eval_builtin_arithmetic() {
        assert_eval!("(+ 1 2 3)", Ok(Object::Integer(6)));
        assert_eval!("(- 3 2 1)", Ok(Object::Integer(0)));
        assert_eval!("(* 2 2 2)", Ok(Object::Integer(8)));
    }

    #[test]
    fn test_eval_builtin_list() {
        assert_eval!(
            "(list 1 2 3)",
            Ok(Object::List(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3)
            ]))
        );
    }

    #[test]
    fn test_eval_builtin_cons() {
        assert_eval!(
            "(cons 1 2)",
            Ok(Object::List(vec![Object::Integer(1), Object::Integer(2)]))
        );
    }

    #[test]
    fn test_eval_builtin_car() {
        assert_eval!("(car (cons 1 2))", Ok(Object::Integer(1)));
    }

    #[test]
    fn test_eval_applying_non_callable() {
        assert_eval!(
            "(1)",
            Err(Object::Error(String::from("cannot call non-function")))
        );
    }

    #[test]
    fn test_definitions() {
        assert_eval!(
            "(define foobar 15)
            foobar",
            Ok(Object::Integer(15))
        );

        assert_eval!(
            "(define foobar (+ 1 2 3 4))
            foobar",
            Ok(Object::Integer(10))
        );
    }

    #[test]
    fn test_lambdas() {
        assert_eval!("((lambda (x) (+ x 1)) 2)", Ok(Object::Integer(3)));
        assert_eval!("((lambda (a b c) (+ a b c)) 1 2 3)", Ok(Object::Integer(6)));
    }
}
