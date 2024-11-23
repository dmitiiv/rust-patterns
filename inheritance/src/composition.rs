trait BaseAnimal {
    type Parent: BaseAnimal;

    fn parent(&self) -> &Self::Parent;

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

impl BaseAnimal for Animal {
    type Parent = Self;

    fn parent(&self) -> &Self::Parent {
        self
    }

    fn name(&self, text: &'static str) {
        println!("The animal name is {text}");
    }
}

struct Cat(Animal);

impl Cat {
    fn new(prefix: &'static str) -> Self {
        Self(Animal { prefix })
        // or
        // Self(Animal::new(prefix))
    }
}

impl BaseAnimal for Cat {
    type Parent = Animal;

    fn parent(&self) -> &Self::Parent {
        &self.0
    }

    fn name(&self, text: &'static str) {
        self.parent().name(text);
    }
}

struct Winky(Cat);

impl Default for Winky {
    fn default() -> Self {
        Self(Cat::new("Winky"))
    }
}

impl BaseAnimal for Winky {
    type Parent = Cat;

    fn parent(&self) -> &Self::Parent {
        &self.0
    }
}

struct Komaru(Cat);

impl Default for Komaru {
    fn default() -> Self {
        Self(Cat::new("Komaru"))
    }
}

impl BaseAnimal for Komaru {
    type Parent = Cat;

    fn parent(&self) -> &Self::Parent {
        &self.0
    }

    fn name(&self, text: &'static str) {
        println!("The fucking cat name if {text}");
    }
}

fn unsized_sayer<A>()
where
    A: Default + BaseAnimal,
{
    A::default().name("hello1");
}

fn sized_sayer(animal: impl BaseAnimal<Parent = Cat>) {
    animal.name(animal.parent().parent().prefix);
}

pub fn run() {
    unsized_sayer::<Winky>();
    sized_sayer(Komaru::default());
}
