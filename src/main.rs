use std::fmt;
use std::io;
use std::io::prelude::*;
use std::iter::Peekable;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum Object {
    Nil,
    Pair { car: Rc<Object>, cdr: Rc<Object> },
    Integer(i64),
}

impl Object {
    fn is_nil(&self) -> bool {
        match self {
            Object::Nil => true,
            _ => false,
        }
    }

    fn is_pair(&self) -> bool {
        match self {
            Object::Pair {
                car: _car,
                cdr: _cdr,
            } => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            Object::Integer(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Nil => write!(f, "<nil>"),
            Object::Pair { car, cdr } => write!(f, "({} . {})", car, cdr),
            Object::Integer(num) => write!(f, "{}", num),
        }
    }
}

fn cons(car: Rc<Object>, cdr: Rc<Object>) -> Rc<Object> {
    Rc::new(Object::Pair {
        car: car.clone(),
        cdr: cdr.clone(),
    })
}

fn car(pair: Rc<Object>) -> Rc<Object> {
    match pair.deref() {
        Object::Pair { car, cdr: _cdr } => car.clone(),
        _ => Rc::new(Object::Nil),
    }
}

fn cdr(pair: Rc<Object>) -> Rc<Object> {
    match pair.deref() {
        Object::Pair { car: _car, cdr } => cdr.clone(),
        _ => Rc::new(Object::Nil),
    }
}

fn read_number<T: Iterator<Item = char>>(
    c: char,
    iter: &mut Peekable<T>,
) -> Result<i64, std::num::ParseIntError> {
    let mut number = c.to_string().parse::<i64>()?;

    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<i64>()) {
        number = number * 10 + digit;
        iter.next();
    }

    Ok(number)
}

fn read_list<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<Rc<Object>, String> {
    let mut list = Rc::new(Object::Nil);

    while let Some(c) = iter.next() {
        match c {
            '0'...'9' => match read_number(c, iter) {
                Ok(integer) => {
                    list = cons(Rc::new(Object::Integer(integer)), list);
                }
                Err(e) => {
                    return Err(format!("parsing number failed: {}", e));
                }
            },
            '(' => match read_list(iter) {
                Ok(sub_list) => {
                    list = cons(sub_list, list);
                }
                Err(e) => {
                    return Err(format!("parsing list failed: {}", e));
                }
            },
            ')' => {
                break;
            }
            ' ' => {
                continue;
            }
            _ => {
                return Err(format!("unexpected character: {:?}", c));
            }
        }
    }

    Ok(list)
}

fn read(code: &str) -> Result<Vec<Rc<Object>>, String> {
    let mut lexer = code.chars().peekable();
    let mut objects = Vec::new();

    while let Some(c) = lexer.next() {
        match c {
            '0'...'9' => match read_number(c, &mut lexer) {
                Ok(integer) => {
                    objects.push(Rc::new(Object::Integer(integer)));
                }
                Err(e) => {
                    return Err(format!("parsing number failed: {}", e));
                }
            },
            '(' => match read_list(&mut lexer) {
                Ok(list) => {
                    objects.push(list);
                }
                Err(e) => {
                    return Err(format!("parsing list failed: {}", e));
                }
            },
            ' ' | '\n' => {
                continue;
            }
            _ => {
                return Err(format!("unexpected character: {:?}", c));
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
        if let Object::Pair { car, cdr } = pair.deref() {
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
        println!("objects={:?}", objects);
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
