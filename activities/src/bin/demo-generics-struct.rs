struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

struct ConveyorBelt<T: Convey> {
    items: Vec<T>,
}

impl<T: Convey> ConveyorBelt<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            width: 3.0,
            height: 1.0,
            depth: 5.0,
            weight: 1.5,
            part_number: "a384bxn".to_owned(),
        }
    }
}

impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}
fn main() {
    let mut belt = ConveyorBelt { items: vec![] };

    belt.add(CarPart::default());
}
