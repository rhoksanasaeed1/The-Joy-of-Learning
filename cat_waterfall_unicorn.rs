fn main() {
    // Introduction to Rust Programming
    println!("Hello World!");

    // Variables and Types
    let mut age: i32 = 25;
    println!("My age is: {}", age);

    // Control Flows
    if age > 18 {
        println!("You are an adult!");
    } else {
        println!("You are still a minor!");
    }

    // Loops
    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i += 1;
    }

    // Functions
    fn say_hello () {
        println!("Hello!");
    }
    say_hello();

    // Structs
    struct Person {
        name:String,
        age:i32,
    }

    let person1 = Person {
        name: String::from("John"),
        age: 30
    };

    println!("Name: {} and Age: {}", person1.name, person1.age);

    // Traits
    trait Greeter {
        fn greet(&self);
    }

    impl Greeter for Person {
        fn greet(&self) {
            println!("Hello, my name is {}!", self.name);
        }
    }

    person1.greet();

    // Modules
    mod calculator {
        pub fn add(x:i32, y:i32) {
            println!("The sum is: {}", x+y);
        }
    }

    calculator::add(5, 6);

    // Generics
    fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // Lifetimes
    struct Person1<'a> {
        name: &'a str,
        age: u8
    }

    fn main1() {
        let name = "Peter";
        let age = 27;
        let peter = Person1 { name, age };

        println!("{} is {}", peter.name, peter.age);
    }

    main1();

    // Closures
    let plus_one = |x:i32| x + 1;
    println!("1 + 1 = {}", plus_one(1));

    // Futures
    use std::future::Future;

    fn main2() -> impl Future<Output = ()> {
        println!("Hello Future!");
        async {
            println!("Inside Future");
        }
    }

    tokio::runtime::Runtime::new().unwrap().block_on(main2());

    // Error Handling
    use std::error::Error;
    use std::io;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = std::fs::File::open("foo.txt")?;
        let mut s = String::new();
        std::io::BufReader::new(f).read_line(&mut s)?;
        Ok(s)
    }

    match read_username_from_file() {
        Ok(name) => println!("Name: {}", name),
        Err(e) => println!("Error: {}", e.description())
    }

    // Memory Management
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    let first = &v[0];
    v.push(6);

    println!("The first element is {}", first);
}