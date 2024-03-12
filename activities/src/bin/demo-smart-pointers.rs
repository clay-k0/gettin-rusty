#![allow(warnings)]

use std::rc::Rc;

#[derive(Debug)]
enum VehicleType {
    TwoDoor,
    FourDoor,
}

#[derive(Debug)]
struct Vehicle {
    year: i32,
    color: String,
    make: String,
    model: String,
    vin: String,
    doors: VehicleType,
}

#[derive(Debug)]
struct Door {
    // Create an Rc vehicle type since multiple doors can belong to a single car
    vehicle: Rc<Vehicle>,
}

fn main() {
    // Create a reference counted pointer (Rc)
    let my_car = Rc::new(Vehicle {
        year: 2023,
        color: "White".to_owned(),
        make: "Toyota".to_owned(),
        model: "GR86".to_owned(),
        vin: "D3829SI".to_owned(),
        doors: VehicleType::TwoDoor,
    });

    let left_door = Door {
        // Make a new reference
        vehicle: Rc::clone(&my_car),
    };

    let right_door = Door {
        // Make another new reference
        vehicle: Rc::clone(&my_car),
    };

    drop(my_car);

    // Even after dropping the first owner of my_car, it is still owned
    // by the left_door and right_door since we cloned the car. As long as
    // one owner is still around, the data still exists:
    println!("Left Door Belongs to {:?}", left_door.vehicle);
    println!("Right Door Belongs to {:?}", right_door.vehicle);
}
