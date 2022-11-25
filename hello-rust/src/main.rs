fn main() {
    let fun = String::from("Rust");
    println!("Hello, {}!", fun);

    //dynamic array
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    vector.push(6);

    for i in vector.iter() {
        println!("{}", i);
    }

    let my_arr: [u64; 3] = [1, 2, 3];
    let mut sumation = 0;

    for i in my_arr.iter() {
        sumation += i;
    }

    let mut sumation_32 = 0;
    for i in &vector {
        sumation_32 += i;
    }

    println!("Sumation: {}", sumation);
    println!("Sumation: {}", sumation_32);

    let _tuple: (i32, &str, f64) = (500, "wow", 3.14);
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

    fn print_complex() {
        let c = complex(3.0, 4.0);
        //a complex number a + bi is represented as (a, b)

        let d = Complex {
            real: 3,
            imaginary: 4,
        };
        println!("Complex: {} + {}i", c.real, c.imaginary);
        println!("Complex: {} + {}i", d.real, d.imaginary);
    }

    print_complex();

    struct Foo<T> {
        bar: T,
    }

    //methods
    //implement a generic type of foo
    impl<T> Foo<T> {
        fn bar(&self) -> &T {
            //self reference
            &self.bar
        }

        fn bar_mut(&mut self) -> &mut T {
            //mutable reference
            &mut self.bar
        }

        fn into_bar(self) -> T {
            //consume self
            self.bar
        }
    }

    //instanciate Foo
    let a_foo = Foo { bar: 42 }; //nerd :D
    println!("Foo: {}", a_foo.bar());

    // Traits, like interfaces

    trait FooBar<T> {
        fn foo(self) -> Option<T>; //foo could return a type or not, is optional
    }

    impl<T> FooBar<T> for Foo<T> {
        fn foo(self) -> Option<T> {
            Some(self.bar)
        }
    }

    let another = Foo { bar: 1 };
    println!("{:?}", another.foo());

    fn fibo(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibo(n - 1) + fibo(n - 2)
        }
    }

    // function pointer type
    type FunctionPtr = fn(u32) -> u32;
    let fib : FunctionPtr = fibo;

    for i in 0..10 {
        print!("Fibo: {}", fib(i));
    }

    //pointers and memory safety

    



}

struct Complex<T> {
    real: T,
    imaginary: T,
}

fn complex(real: f64, imaginary: f64) -> Complex<f64> {
    Complex { real, imaginary }
}
