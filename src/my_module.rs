pub mod divisible;
pub use self::divisible::is_not_divisible_by_three;

pub fn hello_from_module(num: u32) {
    if is_not_divisible_by_three(num) {
        println!("Hello");
    } else {
        println!("Goodbye");
    }
}
