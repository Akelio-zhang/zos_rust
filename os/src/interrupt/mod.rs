//! 中断模块
//! 
//! 
mod timer;
mod handler;
mod context;


/// 初始化中断相关的子模块
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    println!("Mod[interrupt] initialized");
}
