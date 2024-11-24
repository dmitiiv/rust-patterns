mod base;
mod composition;
mod darkmagic;
// https://habr.com/ru/articles/741458/

fn main() {
    base::run();
    composition::run();
    darkmagic::run();
    println!("Hello, world!");
}
