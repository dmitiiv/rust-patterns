trait Visitor {
    fn visit_element_a(&self, element: &mut ElementA);
    fn visit_element_b(&self, element: &mut ElementB);
}

struct ConcreteVisitor {}

impl Visitor for ConcreteVisitor {
    fn visit_element_a(&self, element: &mut ElementA) {
        todo!()
    }

    fn visit_element_b(&self, element: &mut ElementB) {
        todo!()
    }
}

trait Element {
    fn accept(&mut self, visitor: &dyn Visitor);
}

struct ElementA {}

impl Element for ElementA {
    fn accept(&mut self, visitor: &dyn Visitor) {
        visitor.visit_element_a(self);
    }
}
struct ElementB {}

impl Element for ElementB {
    fn accept(&mut self, visitor: &dyn Visitor) {
        visitor.visit_element_b(self);
    }
}

pub fn oop() {
    let visitor = ConcreteVisitor {};

    let list: Vec<Box<dyn Element>> = vec![Box::new(ElementA {}), Box::new(ElementB {})];

    for mut elem in list {
        elem.accept(&visitor);
    }
}
