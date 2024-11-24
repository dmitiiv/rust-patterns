trait Animal<Parent: Animal<Parent>> {
    fn name(text: &'static str) {
        Parent::name(text);
    }
}

struct Cat {}

impl Animal<Self> for Cat {
    fn name(text: &'static str) {
        println!("The cat name is {text}")
    }
}

struct Dog {}

impl Animal<Self> for Dog {
    fn name(text: &'static str) {
        println!("The dog name is {text}")
    }
}

struct Kokoa {}

impl Animal<Cat> for Kokoa {}
impl Animal<Dog> for Kokoa {}

fn name_cat<A>(text: &'static str)
where
    A: Animal<Cat>,
{
    A::name(text);
}

fn name_dog<A>(text: &'static str)
where
    A: Animal<Dog>,
{
    A::name(text);
}

pub fn run() {
    name_cat::<Kokoa>("Barsik");
    name_dog::<Kokoa>("Rex");

    <Kokoa as Animal<Cat>>::name("text");
}
