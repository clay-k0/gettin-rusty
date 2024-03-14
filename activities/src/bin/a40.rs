// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate
#![allow(warnings)]
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum VehicleType {
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum RentalCarStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct RentalCar {
    status: RentalCarStatus,
    vehicle_type: VehicleType,
    vin: String,
}

type Car = Rc<RefCell<Vec<RentalCar>>>;

#[derive(Debug)]
struct Corporate(Car);
#[derive(Debug)]
struct StoreFront(Car);

fn main() {
    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn update_status() {
            let vehicles = vec![
                RentalCar {
                    status: RentalCarStatus::Available,
                    vehicle_type: VehicleType::Car,
                    vin: "fj489d".to_owned(),
                },
                RentalCar {
                    status: RentalCarStatus::Maintenance,
                    vehicle_type: VehicleType::Truck,
                    vin: "d4h783".to_owned(),
                },
            ];
            let vehicles = Rc::new(RefCell::new(vehicles));

            let corpo = Corporate(Rc::clone(&vehicles));
            let storefront = StoreFront(Rc::clone(&vehicles));

            {
                let mut rentals = storefront.0.borrow_mut();
                if let Some(car) = rentals.get_mut(0) {
                    assert_eq!(car.status, RentalCarStatus::Available);
                    car.status = RentalCarStatus::Rented
                }
            }

            {
                let mut rentals = corpo.0.borrow_mut();
                if let Some(car) = rentals.get_mut(0) {
                    assert_eq!(car.status, RentalCarStatus::Rented);
                    car.status = RentalCarStatus::Available
                }
            }

            let rentals = storefront.0.borrow();
            if let Some(car) = rentals.get(0) {
                assert_eq!(car.status, RentalCarStatus::Available)
            }
        }
    }
}
