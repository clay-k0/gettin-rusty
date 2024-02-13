enum Ride {
    BumperCars(i32, f64),
    Rollercoaster(i32, f64),
    FerrisWheel(i32, f64),
}

impl Ride {
    fn print(&self) {
        match self {
            Ride::BumperCars(req_age, req_height) => {
                println!(
                    "Bumper Car Requirements: Age: {:?}, Height: {:?} in.",
                    req_age, req_height
                );
            }
            Ride::Rollercoaster(req_age, req_height) => {
                println!(
                    "Rollercoaster Requirements: Age: {:?}, Height: {:?} in.",
                    req_age, req_height
                );
            }
            Ride::FerrisWheel(req_age, req_height) => {
                println!(
                    "Ferris Wheel Requirements: Age: {:?}, Height: {:?} in.",
                    req_age, req_height
                );
            }
        }
    }
}

struct Visitor {
    name: String,
    age: i32,
    height: f64,
}

fn main() {
    let rides = vec![
        Ride::BumperCars(5, 54.0),
        Ride::Rollercoaster(4, 48.0),
        Ride::FerrisWheel(18, 50.0),
    ];

    for ride in &rides {
        ride.print();
    }

    let visitors = vec![
        Visitor {
            name: String::from("Clayton"),
            age: 20,
            height: 70.0,
        },
        Visitor {
            name: String::from("Daniel"),
            age: 78,
            height: 66.0,
        },
        Visitor {
            name: String::from("Sereia"),
            age: 5,
            height: 48.0,
        },
    ];

    println!();

    println!("Visitors:");
    for visitor in &visitors {
        println!(
            "Name: {:?}, Age: {:?}, Height: {:?}",
            &visitor.name, visitor.age, visitor.height
        );
    }

    for visitor in &visitors {
        println!();
        println!("Visitor: {}", visitor.name);
        for ride in &rides {
            match ride {
                Ride::BumperCars(req_age, ..) => {
                    if visitor.age >= *req_age {
                        println!("- Yes, allowed to ride the Bumper Cars :)");
                    } else {
                        println!("- No, not allowed to ride the Bumper Cars :(");
                    }
                }
                Ride::Rollercoaster(req_age, ..) => {
                    if visitor.age >= *req_age {
                        println!("- Yes, allowed to ride the Rollercoaster :)");
                    } else {
                        println!("- No, not allowed to ride the Rollercoaster :(");
                    }
                }
                Ride::FerrisWheel(req_age, ..) => {
                    if visitor.age >= *req_age {
                        println!("- Yes, allowed to ride the Ferris Wheel :)");
                    } else {
                        println!("- No, not allowed to ride the Rollercoaster :(");
                    }
                }
            }
        }
    }
}
