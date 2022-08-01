fn f3() {
    let some_number = Some(5);
    let some_strung = Some("a string");
    let absent_number: Option<i32> = None;

    f2();
}

fn f2() {
    let x = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap_or(0);

    println!("Result = {sum}");
}