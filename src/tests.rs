#[cfg(test)]
use crate::Expression;

#[test]
fn test_1(){
    let s = Expression::from_str("1");
    assert_eq!(s.to_string(), "1");
}

#[test]
fn test_2(){
    let s = Expression::from_str("1 + 2 * 3");
    assert_eq!(s.to_string(), "(+ 1 (* 2 3))")
}

#[test]
fn test_3(){
    let s = Expression::from_str("a * 2 * b");
    assert_eq!(s.to_string(), "(* (* a 2) b)")
}

#[test]
fn test_4(){
    let s = Expression::from_str("a + b * 2 * c + a / 4");
    assert_eq!(s.to_string(), "(+ (+ a (* (* b 2) c)) (/ a 4))");
}


#[test]
fn test_5(){
    let s = Expression::from_str("2 + b * 5 - 3 / 5 + 5 -3");
    assert_eq!(s.to_string(), "(- (+ (- (+ 2 (* b 5)) (/ 3 5)) 5) 3)");
}


#[test]
fn test_6(){
    let s = Expression::from_str("(2 + b) * 5 ");
    assert_eq!(s.to_string(), "(* (+ 2 b) 5)");
}

#[test]
fn test_7(){
    let s = Expression::from_str("(((a)))");
    assert_eq!(s.to_string(), "a");
}

/* 
#[test]
fn test_8(){
    let s = Expression::from_str("a + b * 2 * (c + a) / 4");
    assert_eq!(s.to_string(), "(+ a (/ (* (* b 2) (+ c a)) 4))");
}


#[test]
fn test_9(){
    let s = Expression::from_str("a + b * c ^ 4");
    println!("{}", s.to_string());
    assert_eq!(s.to_string(), "(+ a (* b (^ c 4)))");
}

#[test]
fn test_10(){
    let s = Expression::from_str("a + 2 √ 4 * b");
    println!("{}", s.to_string());
    assert_eq!(s.to_string(), "(+ a (* (√ 2 4) b))");
}

#[test]
fn test_11(){
    let s = Expression::from_str("a + 2 √ (4 * b)");
    println!("{}", s.to_string());
    assert_eq!(s.to_string(), "(+ a (√ 2 (* 4 b)))");
}

#[test]
fn test_12(){
    let s = Expression::from_str("a ^ b ^ 2");
    println!("{}", s.to_string());
    assert_eq!(s.to_string(), "(^ a (^ b 2))");
}


#[test]
fn test_13(){
    let s = Expression::from_str("a.b.c.d");
    println!("{}", s.to_string());
    assert_eq!(s.to_string(), "(. (. (. a b) c) d)");
}*/