fn main() {
    let x = 5;
    println!("Vaule of x: {}",x);

    let x = "six";
    println!("Vaule of x: {}",x);

    // tuples...

    let tup = ("Apaar", 69);
    let (_name, _marks) = tup;

    println!("Tuples => Name: {} , Marks: {}", tup.0, tup.1);

    // Arrays

    let error_codes = [200,404,500];
    let not_found = error_codes[1]; // assigning value of an array
    
    let byte = [0;8]; // different way of creating array

    // Funtion Call

    let sum = my_function(2, 3);
    println!("Sum from function call is: {}", sum);

    // Control Flow

    //Loop

    let mut counter = 0;
    let loop_result = loop{
        counter += 1;
        if counter == 10{
            break counter; // breaking out of loop and returning counter value
        }
    };
    println!("Loop value: {}", loop_result);

    // for loop

    let a = [10,20,30,40,50];

    for element in a.iter(){    // 1st way to iter 
        println!("1st way the value is: {}", element);
    }

    for number in 1..5{         // 2nd way to iter
        println!("2nd way the value is:{}",number);
    }

    // while loop is same as always


    // Commments in Rust

    // Line Comment

    /*
    
        Block Comment
    
     */


}

// Functions

fn my_function(x: i32, y:i32) -> i32{
    println!("from fn Value of x is: {}",x);
    println!("from fn Value of y is: {}",y);

    x+y // simplified way of writing return statement

}
