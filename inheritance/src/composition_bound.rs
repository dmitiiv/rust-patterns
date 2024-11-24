trait BaseAnimal<Parent: BaseAnimal<Animal>> {
    fn parent(&self) -> &Parent;

    fn name(&self, text: &'static str) {
        self.parent().name(text);
    }
}

struct Animal {
    prefix: &'static str,
}

impl Animal {
    fn new(prefix: &'static str) -> Self {
        Self { prefix }
    }
}

impl BaseAnimal<Self> for Animal {
    fn parent(&self) -> &Self {
        self
    }

    fn name(&self, text: &'static str) {
        println!("The animal name is {text}");
    }
}

struct Cat(Animal);

impl Default for Cat {
    fn default() -> Self {
        Self(Animal::new("Cat"))
    }
}

impl BaseAnimal<Animal> for Cat {
    fn parent(&self) -> &Animal {
        &self.0
    }
}

struct Dog(Animal);

impl Default for Dog {
    fn default() -> Self {
        Self(Animal::new("Dog"))
    }
}

impl BaseAnimal<Animal> for Dog {
    fn parent(&self) -> &Animal {
        &self.0
    }
}

#[derive(Default)]
struct Komaru(Cat, Dog);

impl BaseAnimal<Cat> for Komaru {
    fn parent(&self) -> &Cat {
        &self.0
    }
}

impl BaseAnimal<Dog> for Komaru {
    fn parent(&self) -> &Dog {
        &self.1
    }
}

fn unsized_cat_sayer<A>()
where
    A: Default + BaseAnimal<Cat>,
{
    A::default().name("hello");
}

fn sized_cat_sayer(animal: &impl BaseAnimal<Cat>) {
    animal.name("hello");
}

pub fn run() {
    let komaru = Komaru::default();

    unsized_cat_sayer::<Komaru>();
    sized_cat_sayer(&komaru);

    BaseAnimal::<Cat>::name(&komaru, "Что я такое...");
}
