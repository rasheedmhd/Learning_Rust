// THE RUST PROGRAMMING LANGUAGE BOOK
// CHAPTER 10 - GENERIC TYPES, TRAITS AND LIFETIMES

// === 10.1 - GENERIC DATA TYPES


// MONOMORPHIZATION ->  rust turns generic code into specific code during compilation
// by filling in generic types with concrete types
// this ensures that we don't have PERFORMANCE downgrades when using generics
//
//    let town_names = vec![String::from("Garu"), String::from("Bawku"), String::from("Pusiga"), String::from("Zebilla")];
//    //println!("{}", garu);
//    println!("{:?}", town_names);
//    //let garu = town_names[0];
//    println!("{:?}", town_names[0]);


// largest is generic over some type T and takes a reference to an arguement
// which is a slice of values of  same type T and returns
// a reference to the value of same type T
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// IN STRUCT DEFINITIONS
// Point<T> is generic over some type, T and its fields are of the same type
// whatever the type maybe that means the fields can't hold different data types

struct MonoPoint<T> {
    x: T,
    y: T,
}

// Generic method of MonoPoint struct that works on any instance of MonoPoint.
// regardless of its types
impl<T> MonoPoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can implement methods on a struct for only certain instance of the struct
// that have a certain concrete type
impl MonoPoint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl MonoPoint<String> {
    fn add_qmark_to_x(&self) -> &String {
        //let qmark = String::from("?");
        &self.x
        //println!("{}!", &self.x)
        // &self.x.push_str(&qmark)
        // let mut result = &self.x;
        // result.push_str(&qmark)
    }
}

// We can create structs that are generic  and can hold one type for all fields,
// or can hold different types for any arbitrary field, here, Point<T, U>
// can hold at most 2 different types.
// We say Point<T,U> is generic of some types T, U whatever types T,U maybe.
struct Point<T, U> {
    x: T,
    y: U,
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// MY SHENANIGANS
// Here I tried passing an enum and using its instances to feed to the mixup method.
// it didn't work and i learn't why.
// An enum is an enumeration, when you are creating instances of enums, each instance,
// only maps to 1 variant but with structs, instances, instantiates all the fields when
// created. You can pass in an enum to the function below but you will only be passing it
// as a variable not a struct, where the function can choose one field and leave the other

// you can pass the instance of an enum to the field of the instance of a struct before
// you can feed it into a function

//impl<X1, Y1> Point2<X1, Y1> {
//    fn mixup<X2, Y2, Un, Deux>(self, other: Point2<X2,Y2>, last: ToTestPoint<Un, Deux>) -> Point<X1,Un> {
//        Point {
//            x: self.x,
//            //y: other.y,
//            y: last,
//        }
//    }
//}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<Un, Deux>(self, second: Point2<Un, Deux>) -> Point<X1, Deux> {
        Point {
            x: self.x,
            //y: other.y,
            y: second.y,
        }
    }
}


//
//fn largest_char(list: &[char]) -> &char {
//    let mut largest = &list[0];
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}

// Generic enum that is generic over types T, U whatever they may be
#[derive(Debug)]
enum ToTestPoint<T, U> {
    X(T),
    Y(U),
}

fn main() {


    let iott_px: ToTestPoint<i128, bool> = ToTestPoint::X(600);
    let iott_py: ToTestPoint<u64, f64> = ToTestPoint::Y(11.0);

    let p1 = Point2 { x: 5, y: 10.5 };
    let p2 = Point2 { x: "Hello", y: iott_px};
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {:?}", p3.x, p3.y);

    // this won't compile because x is of type int and y is of type float
    // Rust's type inference picks the type of the first value and
    // expects the rest of the fields to be of that same type,
    // if the next fields have a different type, Rust won't compile the code
    //
    //    struct OnePoint {
    //        x: i32,
    //        y: i32,
    //    }

    //    let wont_work = OnePoint {
    //        x: 5,
    //        y: 5.0,
    //    };

    let p = MonoPoint {x: 5, y: 10};
    println!("p.x = {}", p.x());

    let both_integer = Point { x: 1, y: 3};
    let both_float = Point { x: 4.0, y: 6.0};
    let float_first = Point { x: 4.0, y: 7};
    let integer_first = Point { x: 8, y: 9.0};

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);


    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);


    let integer = MonoPoint { x: 5, y: 10 };
    let float = MonoPoint { x: 1.0, y: 4.0};

    println!("float.distance_from_origin = {}", float.distance_from_origin());

    // won't run because distance_from_origin in only implemented on
    // float instances of the Point<T> struct
    // println!("integer.distance_from_origin = {}", integer.distance_from_origin());

    let name = MonoPoint {
        x: String::from("North Pole"),
        y: String::from("South Pole"),
    };

    println!("name.x = {}", name.x());
    println!("name.add_qmark_to_x = {}", name.add_qmark_to_x());

    // this won't work because even though MonoPoint is a generic struct,
    // the add_qmark_to_x method was only implemented on its String type instances
    //println!("Integer.add_qmark_to_x = {}", integer.add_qmark_to_x());


    // PERFORMANCE OF CODE USING GENERICS

    let integer = Some(5);
    let float = Some(5.0);

    // after MONOMORPHIZATION, the code becomes
    // the compiler uses different names than what we are using here to illustrate
    enum Option_i32 {
        Some(i32),
        None,
    }
    enum Option_f64 {
        Some(f64),
        None
    }
    //    let integer = Some(5); -> let integer = Option_i32::Some(5);
    //    let float = Some(5.0); -> let float = Option::Some(5.0);
}