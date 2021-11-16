mod print;
mod vars;
mod person;

fn main() {
    println!("Print chapter:");
    print::run();
    println!("Vars chapter:");
    vars::run();

    let a: i8 = 8;
    let b: i8 = 9;
    // conditions
    if a < b {
        println!("A < B");
    } else {
      // will not be executed
    }
    // While (FizzBuzz)
    let mut count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        }else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }
    // for range
    for x in 0..10 {
        println!("{} ", x);
    }
    // Functions
    println!("Adding {} and {} equal {}",5,8, add(5,8));

    // Closure
    let multiply = |x: i32, y: i32| x * y;
    println!("Multiply 5 x 9 = {}", multiply(5, 9));

    // Let's run the person run function
    person::run();

}

fn add(x: i32, y: i32) -> i32{
    x + y
}

#[test]
pub fn test_add(){
    assert_eq!(add(3, 4), 7);
}