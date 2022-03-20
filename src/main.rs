struct Thing {
    first : u32,
    second : u32,
}

impl Thing {
    fn new(first: u32, second: u32) -> Self { Self { first, second } }

    /// Get the thing's first.
    fn first(&self) -> u32 {
        self.first
    }
}

fn return_func() -> i32 {
    5
}

fn main() {
    let mut input = "HI";
    // std::io::stdin().read_line(&mut input)
        // .expect("Failed to read line");
    // let x: std::io::Result<&str> = Err(std::io::Error::new(std::io::ErrorKind::NotFound,"hi"));
    let x: std::io::Result<&str> = Ok("all good");
    x.expect("hi");
    println!("Hello, {}!", input);

    let num: i32 = return_func();

    println!("😻 {}", num);


    let mut t: Thing =  Thing::new(1,2);
    println!("{}", t.first);

    let _s: &'static str = "hi";
    let _s = stringify!("hi");

    let _s: String = String::from("heap alooooc");

    let s = "hi";

    println!("{}", s);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 9 {
            break counter;
        }

        // break counter;
    };

    println!("{}", result)
}

