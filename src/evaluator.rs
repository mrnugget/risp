use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::object::Object;
use crate::object;

type Environment = HashMap<Rc<Object>, Rc<Object>>;

pub fn apply(proc: Rc<Object>, args: Rc<Object>, _env: Environment) -> Rc<Object> {
    if let Object::Symbol(s) = proc.deref() {
        if s != "+" {
            Rc::new(Object::Nil);
        }

        return object::sum(args);
    }

    Rc::new(Object::Nil)
}

pub fn eval(exp: Rc<Object>, env: Environment) -> Rc<Object> {
    match exp.deref() {
        Object::Nil => exp,
        Object::Integer(_) => exp,
        Object::Symbol(_) => exp,
        Object::Pair(car, cdr) => apply(car.clone(), cdr.clone(), env),
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

        let env = HashMap::new();
        let result = eval(exp.clone(), env);
        assert!(result.is_integer());
        assert_eq!(*result, Object::Integer(6));
    }
}
