// Type - Layout
pub fn listing_two_one() {
    #[repr(C)]
    struct Foo {
        tiny: bool,
        normal: u32,
        small: u8,
        long: u64,
        short: u16,
    }
}
