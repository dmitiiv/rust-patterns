mod base;

mod composition;

// https://habr.com/ru/articles/741458/

fn main() {
    base::run();
    composition::run();
    println!("Hello, world!");
}
