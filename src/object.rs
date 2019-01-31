use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

type BuiltinFunction = fn(Rc<Object>) -> Rc<Object>;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Object {
    Nil,
    Pair(Rc<Object>, Rc<Object>),
    Integer(i64),
    Symbol(String),
    Builtin(BuiltinFunction),
}

impl Object {
    pub fn is_nil(&self) -> bool {
        match self {
            Object::Nil => true,
            _ => false,
        }
    }

    pub fn is_pair(&self) -> bool {
        match self {
            Object::Pair(_, _) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Object::Integer(_) => true,
            _ => false,
        }
    }

    pub fn set_cdr(&mut self, new_cdr: Rc<Object>) {
        if let Object::Pair(_, ref mut cdr) = *self {
            *cdr = new_cdr;
        }
    }

    pub fn set_last_cdr(&mut self, new_cdr: Rc<Object>) {
        if let Object::Pair(_, ref mut cdr) = *self {
            if cdr.is_nil() {
                self.set_cdr(new_cdr);
            } else {
                Rc::get_mut(cdr).unwrap().set_last_cdr(new_cdr);
            }
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "<nil>"),
            Object::Pair(car, cdr) => write!(f, "({} . {})", car, cdr),
            Object::Integer(num) => write!(f, "{}", num),
            Object::Symbol(sym) => write!(f, "{}", sym),
            Object::Builtin(_) => write!(f, "<builtin_function>"),
        }
    }
}

pub fn cons(car: Rc<Object>, cdr: Rc<Object>) -> Rc<Object> {
    Rc::new(Object::Pair(car.clone(), cdr.clone()))
}

pub fn car(pair: Rc<Object>) -> Rc<Object> {
    match pair.deref() {
        Object::Pair(car, _) => car.clone(),
        _ => Rc::new(Object::Nil),
    }
}

pub fn cdr(pair: Rc<Object>) -> Rc<Object> {
    match pair.deref() {
        Object::Pair(_, cdr) => cdr.clone(),
        _ => Rc::new(Object::Nil),
    }
}

pub fn cadr(pair: Rc<Object>) -> Rc<Object> {
    car(cdr(pair))
}

pub fn caadr(pair: Rc<Object>) -> Rc<Object> {
    car(car(cdr(pair)))
}

pub fn sum(pair: Rc<Object>) -> Rc<Object> {
    let mut result = 0;
    match car(pair.clone()).deref() {
        Object::Integer(value) => {
            result += value;
        }
        _ => {
            return Rc::new(Object::Nil);
        }
    }

    if let Object::Integer(value) = sum(cdr(pair.clone())).deref() {
        result += value;
    }

    Rc::new(Object::Integer(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_sum() {
        let list = Rc::new(Object::Pair(
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Pair(
                Rc::new(Object::Integer(2)),
                Rc::new(Object::Pair(
                    Rc::new(Object::Integer(3)),
                    Rc::new(Object::Nil),
                )),
            )),
        ));

        let sum_result = sum(list);
        assert_eq!(*sum_result, Object::Integer(6));
    }

    #[test]
    fn test_consing_objects() {
        let four = Rc::new(Object::Integer(4));
        let five = Rc::new(Object::Integer(5));

        let pair = cons(four.clone(), five.clone());
        assert!(pair.is_pair());
        if let Object::Pair(car, cdr) = pair.deref() {
            assert_eq!(car.deref(), four.deref());
            assert_eq!(cdr.deref(), five.deref());
        }

        assert_eq!(car(pair.clone()).deref(), four.deref());
        assert_eq!(cdr(pair).deref(), five.deref());
    }

    #[test]
    fn dereferencing() {
        let pair = Rc::new(Object::Pair(
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Nil),
        ));

        assert_eq!(*car(pair.clone()), Object::Integer(1));

        let number = Rc::new(Object::Integer(4));
        assert_eq!(*number, Object::Integer(4));
    }
}
