mod front_of_house;

mod back_of_house{
    pub struct Breakfast{
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str)-> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}



pub fn eat_at_restaurant(){
    
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist;

    //Relative path
    front_of_house::hosting::seat_at_table;

    //Struct
    let mut meal = back_of_house::Breakfast::summer("Rye");

}