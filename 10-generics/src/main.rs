use core::num;

struct Point<T,U>{
    x: T,
    y: U,
}

impl<T,U>Point<T,U>{
    fn mixup<V,W>(self,other: Point<V,W>) -> Point<T,W> {
        Point{
            x:self.x,
            y: other.y,
        }
    }
}

fn main() {
    let v_num = vec![10,30,20,33,53,2];
    let largest_num = get_largest(v_num);

    let char_list = vec!['y','s','a','f','g'];
    let largest_char = get_largest(char_list);

}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {

    let mut largest = list[0];
    for number in list{
        if number > largest {
            largest = number;
        }
    }
    largest

}

//other use case of Generics

enum Option<T>{
    Some(T),
    None,
}

enum Result<T,E> {
    Ok(T),
    Err(E),
}
