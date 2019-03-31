use std::iter::Peekable;

use crate::object::Object;

fn read_integer<T: Iterator<Item = char>>(lexer: &mut Peekable<T>) -> Result<Object, String> {
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

    Ok(Object::Integer(number))
}

fn valid_symbol_char(c: &char) -> bool {
    if *c == '(' || *c == ')' {
        return false;
    }

    c.is_ascii_alphanumeric() || c.is_ascii_punctuation()
}

fn read_symbol<T: Iterator<Item = char>>(lexer: &mut Peekable<T>) -> Result<Object, String> {
    let c = lexer.next().unwrap();
    let mut result = c.to_string();

    while let Some(c) = lexer.peek() {
        if !valid_symbol_char(c) {
            break;
        }
        let c = lexer.next().unwrap();
        result.push(c);
    }

    Ok(Object::Symbol(result))
}

fn read_list<T: Iterator<Item = char>>(lexer: &mut Peekable<T>) -> Result<Object, String> {
    let mut elems = vec![];

    lexer.next();

    while let Some(&c) = lexer.peek() {
        if c == ')' {
            lexer.next();
            break;
        }
        if c == ' ' || c == '\n' {
            lexer.next();
            continue;
        }

        let element = if c == '(' {
            read_list(lexer)?
        } else {
            read_object(lexer)?
        };

        elems.push(element);
    }

    Ok(Object::List(elems))
}
fn read_object<T: Iterator<Item = char>>(lexer: &mut Peekable<T>) -> Result<Object, String> {
    match lexer.peek() {
        Some('0'...'9') => read_integer(lexer),
        Some('(') => read_list(lexer),
        Some(c) if valid_symbol_char(c) => read_symbol(lexer),
        c => Err(format!("unexpected character: {:?}", c)),
    }
}

pub fn read(code: &str) -> Result<Vec<Object>, String> {
    let mut lexer = code.chars().peekable();
    let mut objects = Vec::new();

    while let Some(&c) = lexer.peek() {
        if c == ' ' || c == '\n' {
            lexer.next();
            continue;
        }

        let object = read_object(&mut lexer)?;
        objects.push(object);
    }

    Ok(objects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn reading_single_numbers() {
        let objects = read("5").unwrap();

        let number = objects.first().unwrap();

        match number.deref() {
            Object::Integer(int) => assert_eq!(*int, 5),
            _ => assert!(false),
        }

        let objects = read("123456789").unwrap();

        let number = objects.first().unwrap();

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

        match number.deref() {
            Object::Integer(int) => assert_eq!(*int, 5),
            _ => assert!(false),
        }
    }

    #[test]
    fn reading_lists() {
        let objects = read("(1 2 3)").unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(
            objects.first().unwrap(),
            &Object::List(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3)
            ])
        );
    }

    #[test]
    fn reading_lists_of_lists() {
        let objects = read("(1 (2 3 (4 5)))").unwrap();
        assert_eq!(objects.len(), 1);

        assert_eq!(
            objects.first().unwrap(),
            &Object::List(vec![
                Object::Integer(1),
                Object::List(vec![
                    Object::Integer(2),
                    Object::Integer(3),
                    Object::List(vec![Object::Integer(4), Object::Integer(5)]),
                ]),
            ])
        );
    }

    #[test]
    fn testing_valid_symbol_characters() {
        assert!(valid_symbol_char(&'a'));
        assert!(valid_symbol_char(&'z'));
        assert!(valid_symbol_char(&'A'));
        assert!(valid_symbol_char(&'Z'));
        assert!(valid_symbol_char(&'-'));
        assert!(valid_symbol_char(&'!'));
        assert!(valid_symbol_char(&'+'));

        assert!(!valid_symbol_char(&' '));
    }

    #[test]
    fn reading_symbols() {
        let objects = read("(list)").unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(
            objects.first().unwrap(),
            &Object::List(vec![Object::Symbol(String::from("list"))])
        );

        let objects = read("(list-one)").unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(
            objects.first().unwrap(),
            &Object::List(vec![Object::Symbol(String::from("list-one"))])
        );

        let objects = read("(+ 1 2 3)").unwrap();
        assert_eq!(objects.len(), 1);
        assert_eq!(
            objects.first().unwrap(),
            &Object::List(vec![
                Object::Symbol(String::from("+")),
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3)
            ])
        );
    }
}
