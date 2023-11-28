mod playground;
use playground::boot_camp_1::{Destination, Destinations, Directions, WhereToGo};

fn main() {
    let east_destination = Destinations {
        direction: Directions::East,
        destination: Some(Destination {
            place: "Bourgas".to_string(),
            street: "Slavyanska".to_string(),
            distance_from_start: 370.0,
        }),
    };
    let west_destination = Destinations {
        direction: Directions::West,
        destination: Some(Destination {
            place: "Blagoevgrad".to_string(),
            street: "Kurska".to_string(),
            distance_from_start: 150.0,
        }),
    };
    let south_destination = Destinations {
        direction: Directions::South,
        destination: Some(Destination {
            place: "Haskovo".to_string(),
            street: "Istambulska".to_string(),
            distance_from_start: 370.0,
        }),
    };
    let north_destination = Destinations {
        direction: Directions::North,
        destination: Some(Destination {
            place: "Ruse".to_string(),
            street: "Dunavska".to_string(),
            distance_from_start: 220.0,
        }),
    };

    east_destination.travel_agent();
    west_destination.travel_agent();
    south_destination.travel_agent();
    north_destination.travel_agent();
}
