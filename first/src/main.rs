mod mars_calc;

fn main() {
    println!("Hello, world!");
    println!("{}", test(2, 3));

    let a = 2;
    let result = stack_only(a);
    dbg!(result);

    //calling mars Calc from marsCalc file
    mars_calc::main();
}

fn test(a: i32, b: i32) -> i32 {
    return a + b;
}


fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d = 5;
    let e = Box::new(7);
    return d + *e;
}

