fn main() {
    let fun = String::from("Rust");
    println!("Hello, {}!", fun);

    //dynamic array
    let mut vector: Vec<i32> = vec![1,2,3,4,5];
    vector.push(6);

    for i in vector.iter() {
        println!("{}", i);
    }

    let my_arr: [u64; 3] = [1,2,3];
    let mut sumation = 0;

    for i in my_arr.iter() {
        sumation += i;
    }

    let mut sumation_32 = 0;
    for i in &vector{
        sumation_32 += i;
    }

    println!("Sumation: {}", sumation);
    println!("Sumation: {}", sumation_32);

    let _tuple: (i32, &str, f64) = (500,"wow", 3.14);
    //iterate a tuple
    let (x, y, z) = _tuple;
    println!("{} {} {}", x, y, z);

    // Types
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 0 };
    println!("Point: {} {}", p.x, p.y);

    //tuple struct
    struct Point2(i32, i32);
    let p2 = Point2(0, 0);
    println!("Point2: {} {}", p2.0, p2.1);

    //enum
    enum Color {
        Red,
        Green,
        Blue,
    }

    fn daltonic(color: Color) {
        match color {
            Color::Red => println!("Green"),
            Color::Green => println!("Red"),
            Color::Blue => println!("Blue"),
        }
    }

    daltonic(Color::Red);

    //generics (T)
    struct Complex<T> {
        real: T,
        imaginary: T,
    }


    fn complex(real: f64, imaginary: f64) -> Complex<f64> {
        Complex { real, imaginary }
    }

    let c = complex(3.0, 4.0);
    //a complex number a + bi is represented as (a, b)
    
    let d = Complex { real: 3, imaginary: 4 };
    println!("Complex: {} + {}i", c.real, c.imaginary);
    println!("Complex: {} + {}i", d.real, d.imaginary);


}