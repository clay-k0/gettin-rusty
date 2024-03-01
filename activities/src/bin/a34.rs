// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state
use chrono::Utc;

#[derive(Debug, Clone, Copy)]
struct LuggageID(usize);
struct Luggage(LuggageID);

struct CheckIn(LuggageID);
struct OnLoading(LuggageID);
struct OffLoading(LuggageID);
struct AwaitingPickup(LuggageID);

struct EndCustody(LuggageID);

impl Luggage {
    fn new(id: LuggageID) -> Self {
        println!("\n{:?}", id);
        println!("-------------------");

        Luggage(id)
    }

    fn check_in(self) -> CheckIn {
        println!("> Checking In at {}", Utc::now());
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn on_load(self) -> OnLoading {
        println!("> Loading Onto Plane");
        OnLoading(self.0)
    }
}

impl OnLoading {
    fn off_load(self) -> OffLoading {
        println!("> Unloading From Plane");
        OffLoading(self.0)
    }
}

impl OffLoading {
    fn await_pickup(self) -> AwaitingPickup {
        println!("> Awaiting Pickup");
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pickup(self) -> (Luggage, EndCustody) {
        println!("> Getting Picked Up By Passenger at {}", Utc::now());
        (Luggage(self.0), EndCustody(self.0))
    }
}

fn main() {
    let id1 = LuggageID(49538);
    let luggage1 = Luggage::new(id1);

    luggage1
        .check_in()
        .on_load()
        .off_load()
        .await_pickup()
        .pickup();

    let id2 = LuggageID(49538);
    let luggage2 = Luggage::new(id2);

    luggage2.check_in().on_load().off_load().await_pickup();
}
