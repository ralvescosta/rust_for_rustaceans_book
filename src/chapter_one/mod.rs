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
