pub fn listing_one_two() {
    /**/
    let mut x;

    /*1*/
    x = 42;

    /*2*/
    let y = &x;

    /*3*/
    //x = 43; //this line dont compile, becouse the x is borrowed in 2

    /*4*/
    assert_eq!(*y, 42);
}

/// Rules around ownership, move, copy semantics and dropping
pub fn listing_one_three() {
    let x1 = 42;
    let y1 = Box::new(84);
    {
        // new scope
        let z = (x1, y1); // x1 is copied and y1 is moved
    } // z go out of the scop so is dropped

    let x2 = x1; // x1's value is copy where, so it was not moved into z in line 24

    //the y1's value was moved into z, because the Box dont have the Copy trait implemented, and z was dropped already
    // let y2 = y1;
}

/// Borrowing and Lifetimes - Shared References &T - Rust assume that shared references are immutable
pub fn listing_one_for() {
    fn cache(input: &i32, sum: &mut i32) {
        *sum = *input + *input;
        assert_eq!(*sum, 2 * *input);
    }

    let input = 32;
    let mut sum: i32 = 0;

    cache(&input, &mut sum);
}

/// Borrowing and Lifetimes - Mutable References &mut T - Rust assume that mutable reference are exclusive
pub fn listing_one_five() {
    fn no_alias(input: &i32, output: &mut i32) {
        if *input == 1 {
            *output = 2;
        }
        if *input != 1 {
            *output = 3;
        }
    }

    let input = 32;
    let mut output: i32 = 0;

    no_alias(&input, &mut output);

    assert_eq!(output, 3);
}

/// Borrowing and Lifetimes - Mutable References &mut T - Access through a mutable reference must leave a value behind
pub fn listing_one_seven() {
    fn replace_with_84(s: &mut Box<i32>) {
        let was = std::mem::take(s);

        *s = was;

        let mut r = Box::new(84);

        std::mem::swap(s, &mut r);

        assert_ne!(*r, 84);
    }

    let mut s = Box::new(42);

    replace_with_84(&mut s);
}

/// Lifetimes - Do not need to be contiguous.
pub fn listing_one_eight() {
    use rand::{thread_rng, Rng};

    fn rand() -> f64 {
        thread_rng().gen_range(-40.0..1.3e5)
    }

    let mut x = Box::new(42);

    let r = &x;

    if rand() > 0.5 {
        *x = 84;
    } else {
        println!("{}", r);
    }
}

/// Lifetimes - Can have holes.
pub fn listing_one_nine() {
    println!("Listing 1-9");
    let mut x = Box::new(42);

    let mut z = &x; // 'a

    for i in 0..100 {
        println!("value of z in for: {}", z); // 'a
        x = Box::new(i);
        z = &x; // 'a
    }

    println!("last value of z: {}", z); // 'a
    println!();
}

/// Lifetimes - A type that needs to be generic over multiple lifetimes
pub fn listing_one_tem() {
    struct StrSplit<'s, 'p> {
        delimiter: &'p str,
        document: &'s str,
    }
    impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
        type Item = &'s str;

        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    fn str_before(s: &str, c: char) -> Option<&str> {
        StrSplit {
            document: s,
            delimiter: &c.to_string(),
        }
        .next()
    }
}

/// Lifetimes - Variance
pub fn listing_one_eleven() {
    println!("Listing 1-11");
    struct MutStr<'a, 'b> {
        s: &'a mut &'b str,
    }

    let mut s = "hello";

    *MutStr { s: &mut s }.s = "world";

    println!("{}", s);
    println!()
}
