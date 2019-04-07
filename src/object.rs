use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::evaluator::eval;

pub struct Environment {
    entries: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>> {
        let mut env = Environment {
            entries: HashMap::new(),
        };

        let native_functions = &[
            ("+", Function::Native(plus)),
            ("*", Function::Native(multiply)),
            ("list", Function::Native(list)),
            ("cons", Function::Native(cons)),
            ("define", Function::Native(define)),
        ];

        for item in native_functions.into_iter() {
            let (name, ref func) = item;
            env.define(name.to_string(), Object::Callable(func.clone()))
                .unwrap();
        }

        Rc::new(RefCell::new(env))
    }

    pub fn define(&mut self, key: String, obj: Object) -> Result<(), ()> {
        self.entries.insert(key, obj);
        Ok(())
    }

    pub fn get(&self, key: &String) -> Object {
        match self.entries.get(key) {
            Some(val) => val.clone(),
            None => Object::Nil,
        }
    }
}

pub type BuiltinFunction = fn(&[Object], Rc<RefCell<Environment>>) -> Object;

pub enum Function {
    Native(BuiltinFunction),
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        self == other
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Native(_) => write!(f, "<native>"),
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Function {
        match *self {
            Function::Native(ref func) => Function::Native(*func),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Object {
    Nil,
    Integer(i64),
    Symbol(String),
    List(Vec<Object>),
    Callable(Function),
    Error(String),
}

macro_rules! err {
    ( $message:expr ) => {{
        Object::Error(String::from(format!($message)))
    }};
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "<nil>"),
            Object::Integer(num) => write!(f, "{}", num),
            Object::Symbol(sym) => write!(f, "{}", sym),
            Object::Error(sym) => write!(f, "Error({})", sym),
            Object::Callable(_) => write!(f, "<callable>"),
            Object::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if i != items.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "Object::Nil"),
            Object::Integer(num) => write!(f, "Object::Integer({})", num),
            Object::Symbol(sym) => write!(f, "Object::Symbol({})", sym),
            Object::Error(sym) => write!(f, "Object::Error({})", sym),
            Object::Callable(_) => write!(f, "Object::Callable(<callable>)"),
            Object::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if i != items.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

pub fn define(args: &[Object], env: Rc<RefCell<Environment>>) -> Object {
    let name = match &args[0] {
        Object::Symbol(name) => name.to_string(),
        _ => return err!("argument has wrong type"),
    };

    let value = eval(args[1].clone(), env.clone());

    env.borrow_mut().define(name.to_string(), value).unwrap();

    Object::Nil
}

pub fn plus(args: &[Object], _env: Rc<RefCell<Environment>>) -> Object {
    let mut sum = 0;
    for i in args.iter() {
        if let Object::Integer(val) = i {
            sum += val;
        } else {
            return Object::Nil;
        }
    }
    Object::Integer(sum)
}

pub fn minus(args: &[Object], _env: Rc<RefCell<Environment>>) -> Object {
    if args.len() < 2 {
        return err!("not enough arguments");
    }

    let mut iter = args.iter();
    let mut sum = match iter.next().unwrap() {
        Object::Integer(first) => *first,
        _ => return err!("argument has wrong type"),
    };

    for i in iter {
        if let Object::Integer(val) = i {
            sum -= val;
        } else {
            return Object::Nil;
        }
    }

    Object::Integer(sum)
}

pub fn multiply(args: &[Object], _env: Rc<RefCell<Environment>>) -> Object {
    let mut sum = 1;
    for o in args.iter() {
        if let Object::Integer(val) = o {
            sum *= val;
        } else {
            return Object::Nil;
        }
    }
    Object::Integer(sum)
}

pub fn list(args: &[Object], _env: Rc<RefCell<Environment>>) -> Object {
    let items = args.to_vec();
    Object::List(items)
}

pub fn cons(args: &[Object], _env: Rc<RefCell<Environment>>) -> Object {
    if args.len() != 2 {
        return err!("wrong number of arguments");
    }

    let items = args.to_vec();
    Object::List(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! integer_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(temp_vec.push(Object::Integer($x));)*
                temp_vec
            }
        };
    }

    #[test]
    fn test_list_plus() {
        let args = integer_vec![1, 2, 3];
        let sum = plus(&args, Environment::new());
        assert_eq!(sum, Object::Integer(6));
    }

    #[test]
    fn test_list_minus() {
        let args = integer_vec![8, 4, 2];
        let result = minus(&args, Environment::new());
        assert_eq!(result, Object::Integer(2));
    }

    #[test]
    fn test_list_multiply() {
        let args = integer_vec![1, 2, 3];
        let multiply_result = multiply(&args, Environment::new());
        assert_eq!(multiply_result, Object::Integer(6));
    }

    #[test]
    fn test_cons() {
        let args = integer_vec![1, 2];
        let cons_result = cons(&args, Environment::new());
        assert_eq!(cons_result, Object::List(integer_vec![1, 2]));

        let args = integer_vec![1, 2, 3, 4];
        let cons_result = cons(&args, Environment::new());
        assert_eq!(
            cons_result,
            Object::Error(String::from("wrong number of arguments"))
        );
    }
}
