mod heap;
mod config;


pub fn init() {
    heap::init();
    println!("Mod[memory] initialized");
}
