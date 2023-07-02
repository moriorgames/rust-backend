mod my_module;

fn main() {    
    let num = 3;
    my_module::hello_from_module(num);
    println!("world!");
}
