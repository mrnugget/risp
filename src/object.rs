use std::fmt;

pub type BuiltinFunction = fn(&[Object]) -> Object;

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

pub fn sum(args: &[Object]) -> Object {
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

pub fn multiply(args: &[Object]) -> Object {
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

pub fn list(args: &[Object]) -> Object {
    let items = args.to_vec();
    Object::List(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_test_args() -> Vec<Object> {
        vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)]
    }

    #[test]
    fn test_list_plus() {
        let args = new_test_args();
        let sum_result = sum(&args);
        assert_eq!(sum_result, Object::Integer(6));
    }

    #[test]
    fn test_list_multiply() {
        let args = new_test_args();
        let multiply_result = multiply(&args);
        assert_eq!(multiply_result, Object::Integer(6));
    }
}
