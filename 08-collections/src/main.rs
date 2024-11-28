use std::collections::HashMap;

fn main() {
    // Vectors
    // Vector is stored on heap

    let mut v1: Vec<i32> = Vec::new();   // 1st way to initialize a vector
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![1,2,3];     // 2nd way to initialize a vector

    let v = vec![1,2,3,4,5];

    let third = &v[2];              // This can give error if use "20", so we shpould use match and get 
    println!("The third element is {}", third);

    match v.get(2){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }


    // Strings

    let s1 = String::new();
    let s2 = "initial content";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // HashMap

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);


}
