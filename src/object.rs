use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    entries: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>> {
        let mut env = Environment {
            parent: None,
            entries: HashMap::new(),
        };

        let native_functions = &[
            ("+", Function::Native(plus)),
            ("*", Function::Native(multiply)),
            ("list", Function::Native(list)),
            ("cons", Function::Native(cons)),
            ("car", Function::Native(car)),
        ];

        for item in native_functions.into_iter() {
            let (name, ref func) = item;
            env.define(name.to_string(), Object::Callable(func.clone()))
                .unwrap();
        }

        Rc::new(RefCell::new(env))
    }

    pub fn new_child(parent: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let env = Environment {
            parent: Some(parent),
            entries: HashMap::new(),
        };

        Rc::new(RefCell::new(env))
    }

    pub fn define(&mut self, key: String, obj: Object) -> Result<(), ()> {
        self.entries.insert(key, obj);
        Ok(())
    }

    pub fn get(&self, key: &String) -> Object {
        match self.entries.get(key) {
            Some(val) => val.clone(),
            None => match self.parent {
                Some(ref parent) => parent.borrow().get(key),
                None => Object::Nil,
            },
        }
    }
}

pub type BuiltinFunction = fn(&[Object], Rc<RefCell<Environment>>) -> Object;

pub enum Function {
    Native(BuiltinFunction),
    Lambda(Vec<Object>, Vec<Object>, Rc<RefCell<Environment>>),
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
            Function::Lambda(_, _, _) => write!(f, "<lambda>"),
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Function {
        match *self {
            Function::Native(ref func) => Function::Native(*func),
            Function::Lambda(ref parameters, ref body, ref env) => {
                Function::Lambda(parameters.clone(), body.clone(), env.clone())
            }
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

pub fn car(args: &[Object], _env: Rc<RefCell<Environment>>) -> Object {
    if args.len() != 1 {
        return err!("wrong number of arguments");
    }

    let items = match &args[0] {
        Object::List(items) => items,
        _ => return err!("argument has wrong type"),
    };

    if items.is_empty() {
        return err!("empty list");
    }

    items[0].clone()
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

    #[test]
    fn test_car() {
        let args = vec![Object::List(integer_vec![1, 2])];
        let car_result = car(&args, Environment::new());
        assert_eq!(car_result, Object::Integer(1));

        let args = vec![Object::List(Vec::new())];
        let car_result = car(&args, Environment::new());
        assert_eq!(car_result, Object::Error(String::from("empty list")));

        let args = vec![Object::Integer(1)];
        let car_result = car(&args, Environment::new());
        assert_eq!(
            car_result,
            Object::Error(String::from("argument has wrong type"))
        );
    }

    #[test]
    fn test_environment_get() {
        let env = Environment::new();

        let name = "six".to_string();
        env.borrow_mut().define(name.clone(), Object::Integer(6));
        assert_eq!(env.borrow_mut().get(&name), Object::Integer(6));

        let name = "doesnotexist".to_string();
        assert_eq!(env.borrow_mut().get(&name), Object::Nil);
    }

    #[test]
    fn test_environment_get_with_parent() {
        let parent = Environment::new();

        let only_in_parent = "inparent".to_string();
        parent
            .borrow_mut()
            .define(only_in_parent.clone(), Object::Integer(6));
        assert_eq!(parent.borrow_mut().get(&only_in_parent), Object::Integer(6));

        let child = Environment::new_child(parent.clone());
        assert_eq!(child.borrow_mut().get(&only_in_parent), Object::Integer(6));

        let only_in_child = "inchild".to_string();
        child
            .borrow_mut()
            .define(only_in_child.clone(), Object::Integer(99));
        assert_eq!(child.borrow_mut().get(&only_in_child), Object::Integer(99));
        assert_eq!(parent.borrow_mut().get(&only_in_child), Object::Nil);
    }
}
