#[derive(Debug)]
enum Shapes {
    Triangle,
    Circle,
    Square,
}

trait GetShape {
    fn get_shape(&self) -> &Shapes;
}

struct Circle {
    shape: Shapes,
    radius: f64,
}

impl GetShape for Circle {
    fn get_shape(&self) -> &Shapes {
        &self.shape
    }
}

struct Triangle {
    shape: Shapes,
    base: f64,
    height: f64,
}

impl GetShape for Triangle {
    fn get_shape(&self) -> &Shapes {
        &self.shape
    }
}

struct Square {
    shape: Shapes,
    side: f64,
}

impl GetShape for Square {
    fn get_shape(&self) -> &Shapes {
        &self.shape
    }
}

fn visitor_func(form: &dyn GetShape) {
    match form.get_shape() {
        Shapes::Circle => {}
        Shapes::Triangle => {}
        Shapes::Square => {}
    }
}

pub fn func() {
    let circle = Circle {
        shape: Shapes::Circle,
        radius: 5.0,
    };

    let triangle = Triangle {
        shape: Shapes::Triangle,
        base: 4.0,
        height: 8.0,
    };

    let square = Square {
        shape: Shapes::Square,
        side: 3.0,
    };

    let shapes: Vec<Box<dyn GetShape>> =
        vec![Box::new(circle), Box::new(triangle), Box::new(square)];

    for shape in shapes {
        visitor_func(&*shape);
    }
}
