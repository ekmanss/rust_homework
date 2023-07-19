use class_one::layer_one;

pub mod class_one;
pub mod util;
pub mod class_three;

fn main() {
    println!("layer_one::print_a2Z::run() ->");
    layer_one::print_a2Z::run();
    println!("layer_one::layer_two::print_A2z::run() ->");
    layer_one::layer_two::print_A2z::run();
}
