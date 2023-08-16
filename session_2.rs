fn add(a: &mut i32, b: &mut i32) {
    *a += *b;
}

fn subtract(a: &mut i32, b: &mut i32) {
    *a -= *b;
}

fn multiply(a: &mut i32, b: &mut i32) {
    *a *= *b;
}

fn divide(a: &mut i32, b: &mut i32) {
    *a /= *b;
}

fn main() {
    let mut n1 = 20;
    let mut n2 = 5;

    println!("Numbers before calculation:");
    println!("Number 1: {}", n1);
    println!("Number 2: {}", n2);

    add(&mut n1, &mut n2);
    println!("Addition: {}",n1);

    subtract(&mut n1, &mut n2);
    println!("Subtraction: {}",n1);

    multiply(&mut n1, &mut n2);
    println!("Multiplication: {}",n1);

    divide(&mut n1, &mut n2);
    println!("Division: {}",n1);
}