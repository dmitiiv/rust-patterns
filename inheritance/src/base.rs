trait Animal {
    type Parent: Animal;

    fn name(text: &'static str) {
        Self::Parent::name(text);
    }
}

struct Dog {}

impl Animal for Dog {
    type Parent = Self;

    fn name(text: &'static str) {
        println!("The dog name is {text}");
    }
}

struct Cat {}

impl Animal for Cat {
    type Parent = Self;

    fn name(text: &'static str) {
        println!("The cat name is {text}");
    }
}

struct Pug {}

impl Animal for Pug {
    type Parent = Dog;
}

struct SmallPug {}

impl Animal for SmallPug {
    type Parent = Pug;
}

fn get_name<A>(text: &'static str)
where
    A: Animal,
{
    A::name(text)
}

fn get_strong_name<A>(text: &'static str)
where
    A: Animal<Parent = Dog>,
{
    A::name(text)
}

// Good for libraires to support inheritans without changing
// the Animal (root) trait
pub fn run() {
    get_name::<Cat>("Barsik");

    // get_strong_name::<SmallPug>("Mops"); // an error (38 type Parent = Pug;) should be Parent = Dog
    // get_strong_name::<Cat>("Mops"); // an error (38 type Parent = Animal;) should be Parent = Dog

    get_strong_name::<Pug>("Mops");
    get_strong_name::<Dog>("Mops");
}
