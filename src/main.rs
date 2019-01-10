use std::fmt;
use std::io;
use std::io::prelude::*;
use std::iter::Peekable;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Object {
    Nil,
    Pair(Rc<Object>, Rc<Object>),
    Integer(i64),
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

fn read_integer<T: Iterator<Item = char>>(lexer: &mut Peekable<T>) -> Result<Rc<Object>, String> {
    let c = lexer.next().unwrap();

    let mut number = match c.to_string().parse::<i64>() {
        Ok(number) => number,
        Err(e) => {
            return Err(format!("error parsing number: {:?}", e));
        }
    };

    while let Some(Ok(digit)) = lexer.peek().map(|c| c.to_string().parse::<i64>()) {
        number = number * 10 + digit;
        lexer.next();
    }

    lexer.next();

    Ok(Rc::new(Object::Integer(number)))
}

fn read_object<T: Iterator<Item = char>>(lexer: &mut Peekable<T>) -> Result<Rc<Object>, String> {
    match lexer.peek() {
        Some('0'...'9') => read_integer(lexer),
        Some('(') => read_list(lexer),
        c => Err(format!("unexpected character: {:?}", c)),
    }
}

fn read_list<T: Iterator<Item = char>>(lexer: &mut Peekable<T>) -> Result<Rc<Object>, String> {
    let mut list = Rc::new(Object::Nil);

    lexer.next();

    while let Some(&c) = lexer.peek() {
        if c == ')' {
            break;
        }
        if c == ' ' || c == '\n' {
            continue;
        }

        let element = if c == '(' {
            read_list(lexer)?
        } else {
            read_object(lexer)?
        };

        if list.is_pair() {
            Rc::get_mut(&mut list)
                .unwrap()
                .set_last_cdr(cons(element, Rc::new(Object::Nil)));
        } else {
            list = cons(element, Rc::new(Object::Nil));
        };
    }

    Ok(list)
}

fn read(code: &str) -> Result<Vec<Rc<Object>>, String> {
    let mut lexer = code.chars().peekable();
    let mut objects = Vec::new();

    while let Some(c) = lexer.peek() {
        if *c == ' ' || *c == '\n' {
            lexer.next();
            continue;
        }

        match read_object(&mut lexer) {
            Ok(obj) => {
                objects.push(obj);
            }
            Err(e) => {
                return Err(format!("parsing number failed: {}", e));
            }
        }
    }

    Ok(objects)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn reading_single_numbers() {
        let objects = read("5").unwrap();

        let number = objects.first().unwrap();
        assert!(number.is_integer());

        match number.deref() {
            Object::Integer(int) => assert_eq!(*int, 5),
            _ => assert!(false),
        }

        let objects = read("123456789").unwrap();

        let number = objects.first().unwrap();
        assert!(number.is_integer());

        match number.deref() {
            Object::Integer(int) => assert_eq!(*int, 123456789),
            _ => assert!(false),
        }
    }

    #[test]
    fn read_multiple_numbers() {
        let objects = read("5 5 5 5").unwrap();
        assert_eq!(objects.len(), 4);

        let number = objects.first().unwrap();
        assert!(number.is_integer());

        match number.deref() {
            Object::Integer(int) => assert_eq!(*int, 5),
            _ => assert!(false),
        }
    }

    #[test]
    fn reading_lists() {
        let objects = read("(1 2 3)").unwrap();
        assert_eq!(objects.len(), 1);

        let list = objects.first().unwrap();
        assert!(list.is_pair());
        assert_eq!(*car(list.clone()), Object::Integer(1));
        assert_eq!(*car(cdr(list.clone())), Object::Integer(2));
        assert_eq!(*car(cdr(cdr(list.clone()))), Object::Integer(3));
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

fn main() -> io::Result<()> {
    const PROMPT: &'static str = "> ";

    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match read(&input) {
            Ok(objects) => {
                objects.iter().for_each(|object| println!("{}", object));
            }
            Err(e) => println!("Something went wrong: {}", e),
        };
    }
}
