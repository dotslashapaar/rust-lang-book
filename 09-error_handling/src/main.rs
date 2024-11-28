use std::fs::File;

// enum Result<T,E>{
//     Ok(T),
//     Err(E),
// }

fn main() {
    // panic!("Critical error! Exiting");

    let f = File::open("hello.txt");

    let f = match f{
        Ok(file)=> file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

}


//Options
// fn get_user_id(name: &str) -> Option<u32>{
//     if database.user_exists(name){
//         return  Some(database.get_id(name));
//     }
//     None
// }


// ? Operator
// fn get_salary(db: Database, id: i32) -> Option<u32> {
//     Some(db.get_user(id)?.get_job()?.salary)
//     }
//     fn connect(db: Database) -> Result<Connection, Error> {let conn =
//     db.get_active_instance()?.connect()?;
//     Ok(conn)
//     }
    
