use crate::graph::{SocialGraph, User};

mod print;
mod vars;
mod person;
mod tree;
mod graph;

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
    println!("Person chapter:");
    person::run();

    println!("Tree chapter:");
    let mut tree = tree::Tree::new();
    tree.set_value(10);

    tree.add(5);
    tree.add(15);
    tree.add(3);
    tree.add(7);
    tree.add(12);
    tree.add(18);

    println!("Searching for 7: {}", tree::Tree::search(&tree, 7)); // true
    println!("Searching for 20: {}", tree::Tree::search(&tree, 20)); // false

    println!("Tree structure: {}", tree.presentation());

    // Graph
    println!("Graph chapter:");
    let mut graph = SocialGraph::new();

    // Create some users
    let user1 = User {
        id: 1,
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        token: String::from("jwt_token_1"),
    };

    let user2 = User {
        id: 2,
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        token: String::from("jwt_token_2"),
    };

    let user3 = User {
        id: 3,
        username: String::from("charlie"),
        email: String::from("charlie@example.com"),
        token: String::from("jwt_token_3"),
    };

    graph.add_user(user1);
    graph.add_user(user2);
    graph.add_user(user3);

    graph.add_follower(1, 2); // Bob follows Alice
    graph.add_follower(1, 3); // Charlie follows Alice
    graph.add_follower(2, 3); // Charlie follows Bob

    graph.display_graph();
}

fn add(x: i32, y: i32) -> i32{
    x + y
}

#[test]
pub fn test_add(){
    assert_eq!(add(3, 4), 7);
}