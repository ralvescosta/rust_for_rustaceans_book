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
