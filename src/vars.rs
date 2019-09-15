// Variables hold primitive data or a references to data
// Variables are immutable by default
// Rust is a bloc-scoped language
/**
Primitive types are:
Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take)
u8: unsigned (means that's no negative values)
Float: f32, f64
Boolean: (bool)
Characters: (char)
Tuples
Arrays
*/
pub fn run (){
    let name = "chokri";
    let age = 34; // We can't change this
    let year = 2019; // This can be changed
    const PI: f32 = 3.14 ;
    println!("PI: {:?}", PI);
    // Assign multiple varis
    let (sport, activity) = ("Basketball", "Playing");
    // Maximum value of i32?
    println!("Max of i32 is {}", std::i32::MAX);

    // String

    let mut title = String::from("Hello");
    println!("{:?}", title);
    title.push_str(" world! ");
    println!("{:?}", title);
    title.push('\u{1F600}'); // Adding Happy face
    println!("{:?}", title);
    println!("Capacity: {}", title.capacity());
    println!("Is Empty?: {}", title.is_empty());
    println!("Contain world?: {}", title.contains("world"));
    title.replace("world", "there");
    println!("Replaced: {}", title);
    for word in title.split_whitespace() {
        println!("/{}/", word);
    }
    // Tuples
    let post: (i8, &str) = (1, "First Post");
    println!("Post: {:?}", post);
    // Arrays
    let scores: [i8; 4] = [std::i8::MAX, std::i8::MIN, 5, 9];
    println!("Scores {:?}", scores);
    println!("Mem Size: {} bytes", std::mem::size_of_val(&scores));
    // Slice an Array
    let slice: &[i8] = &scores[0..2];
    println!("Slice: {:?}", slice);

    // Vectors: resizable arrays
    let mut points: Vec<u8> = vec![1,5];
    println!("Vector: {:?}", points);
    points.push(8);
    println!("Push Vector: {:?}", points);
    println!("Pop Vector: {:?}", points.pop());
    // loop
    for x in points.iter() {
        println!("point: {}", x);
    }
    // loop mutable
    for x in points.iter_mut(){
        *x *= 2;
    }
    println!("Mutable loop: {:?}", points);
}