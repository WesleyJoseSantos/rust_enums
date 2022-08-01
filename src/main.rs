fn main() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("tree"),
        _ => ()
    }

    if let Some(3) = some_value {
        println!("tree");
    }
}

fn func() {
    let five = Some(5);
    let siz = plus_one(five);
    let none = plus_one(None);
    
    print_value(five);
    print_value(siz);
    print_value(none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        _ => None,
    }
}

fn print_value(x: Option<i32>) {
    match x {
        Some(i) => println!("value is {i}"),
        _ => println!("value is none")
    }
}