// fn main() {
//     println!("Hello, world!");

//     let greeting: String = String::from("Hello, world 2!");
//     println!("{}", greeting);

//     let char1: Option<char> = greeting.chars().nth(1);
//     print!("char1: {}", char1.unwrap()); //it oke to have runtime error

//     //conditions
//     let is_even: bool = true;

//     if is_even {
//         println!("is even");
//     } else if !is_even {
//         println!("is odd");
//     };

//     //lops

//     for i in 0..11 {
//         print!(" {}", i);
//     }

//     let sentence: String = String:: from("my name is JOY");
//     let first_word: String = get_first_word(sentence);

//     print!("First word is:{}", first_word);
// }
// fn get_first_word(sentence: String) -> String {
//     let mut ans: String = String::from("");
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }

// struct User {
//     name: String,
//     age: u32,
//     // active: bool,

// }

// fn main(){
//     let name: String = String::from("Joy");
//     let user: User = User {
//         name:name,
//         age: 20,
//         // active: false,
//     };
//     print!("{} is {} years old", user.name, user.age);
// }

use std::fs::DirBuilder;

enum Direction {
    North,
    East,
    South,
    West,
}

fn main(){
    let my_direction: Direction = Direction::North;
    move_around(my_direction);
}
fn move_around(direction: Direction) {
    
}