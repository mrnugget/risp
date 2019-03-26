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

#[derive(PartialEq)]
pub enum Object {
    Nil,
    Integer(i64),
    Symbol(String),
    List(Vec<Object>),
    Callable(Function),
}

impl Object {
    pub fn is_nil(&self) -> bool {
        match self {
            Object::Nil => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Object::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Object::List(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "<nil>"),
            Object::Integer(num) => write!(f, "{}", num),
            Object::Symbol(sym) => write!(f, "{}", sym),
            Object::Callable(_) => write!(f, "<callable>"),
            Object::List(items) => {
                write!(f, "(")?;
                for i in items.iter() {
                    write!(f, "{}", i)?;
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
            Object::Callable(_) => write!(f, "<callable>"),
            Object::List(items) => {
                write!(f, "(")?;
                for i in items.iter() {
                    write!(f, "{}", i)?;
                }
                write!(f, ")")
            }
        }
    }
}

// pub fn cons(car: Rc<Object>, cdr: Rc<Object>) -> Rc<Object> {
//     Rc::new(Object::Pair(car.clone(), cdr.clone()))
// }
//
// pub fn car(pair: Rc<Object>) -> Rc<Object> {
//     match pair.deref() {
//         Object::Pair(car, _) => car.clone(),
//         _ => Rc::new(Object::Nil),
//     }
// }
//
// pub fn cdr(pair: Rc<Object>) -> Rc<Object> {
//     match pair.deref() {
//         Object::Pair(_, cdr) => cdr.clone(),
//         _ => Rc::new(Object::Nil),
//     }
// }
//
// pub fn cadr(pair: Rc<Object>) -> Rc<Object> {
//     car(cdr(pair))
// }
//
// pub fn caadr(pair: Rc<Object>) -> Rc<Object> {
//     car(car(cdr(pair)))
// }

// TODO: This can merged with `multiply` into a generic `fold_integers`
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

// TODO: This can merged with `sum` into a generic `fold_integers`
pub fn multiply(args: &[Object]) -> Object {
    let mut sum = 0;
    for (i, o) in args.iter().enumerate() {
        if let Object::Integer(val) = o {
            if i == 0 {
                sum = *val;
            } else {
                sum *= val;
            }
        } else {
            return Object::Nil;
        }
    }
    Object::Integer(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_test_args() -> Vec<Object> {
        vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)]
    }

    #[test]
    fn test_list_sum() {
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

    // #[test]
    // fn test_consing_objects() {
    //     let four = Rc::new(Object::Integer(4));
    //     let five = Rc::new(Object::Integer(5));
    //
    //     let pair = cons(four.clone(), five.clone());
    //     assert!(pair.is_pair());
    //     if let Object::Pair(car, cdr) = pair.deref() {
    //         assert_eq!(car.deref(), four.deref());
    //         assert_eq!(cdr.deref(), five.deref());
    //     }
    //
    //     assert_eq!(car(pair.clone()).deref(), four.deref());
    //     assert_eq!(cdr(pair).deref(), five.deref());
    // }

    // #[test]
    // fn dereferencing() {
    //     let pair = Rc::new(Object::Pair(
    //         Rc::new(Object::Integer(1)),
    //         Rc::new(Object::Nil),
    //     ));
    //
    //     assert_eq!(*car(pair.clone()), Object::Integer(1));
    //
    //     let number = Rc::new(Object::Integer(4));
    //     assert_eq!(*number, Object::Integer(4));
    // }
}
