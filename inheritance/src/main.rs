mod base;
mod composition;
mod composition_bound;
mod darkmagic;
// https://habr.com/ru/articles/741458/

fn main() {
    base::run();
    composition::run();
    darkmagic::run();
    composition_bound::run();
}
