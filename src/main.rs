fn main() {
    let number_list = vec![2, 27, 15, 5, 10, 12];
    let char_list = vec!['y', 'm', 'b', 'z', 'u'];

    //let result = largest(&number_list);
    //println!("Largest number is {}", result);

    //let result = largest(&char_list);
    //println!("Largest char is {}", result);

    let integer_point = Point{x: 3, y: 7};
    let float_point = Point{x: 4.0, y: 5.7};
    dbg!(&integer_point);
    dbg!(&float_point);

    dbg!(integer_point.x());

    // dbg!(integer_point.distance_from_origin());  // not defined for integers
    dbg!(float_point.distance_from_origin());
}

fn largest<T>(list: &[T]) {//-> T {
    //let mut largest_v = list[0];

    /*for item in list {
        if item > largest_v {
            largest_v = item;
        }
    }*/

    //largest_v
    //list[0]
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {  // generic type T in the impl
    fn x(&self) -> &T {
        &self.x
    }
}

// Can define methods specifically for some concrete types
impl Point<f32> {  // impl is not generic, specific for f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
