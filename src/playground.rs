pub mod boot_camp_1 {
    use std::fmt::Debug;

    pub trait WhereToGo {
        fn travel_agent(&self);
    }

    #[derive(Debug)]
    pub struct Destinations {
        pub direction: Directions,
        pub destination: Option<Destination>,
    }

    #[derive(Debug)]
    pub enum Directions {
        East,
        West,
        South,
        North,
    }

    #[derive(Debug)]
    pub struct Destination {
        pub place: String,
        pub street: String,
        pub distance_from_start: f64,
    }

    impl WhereToGo for Destinations {
        fn travel_agent(&self) {
            if let Some(ref destination) = self.destination {
                match self.direction {
                    Directions::East => println!(
                        "Direction: {:?} | Place: {:?} | Street: {:?} |Distance: {:?} || This shit is in the eastern province",
                        self.direction,
                        destination.place,
                        destination.street,
                        destination.distance_from_start
                    ),
                    Directions::West => println!(
                        "Direction: {:?} | Place: {:?} | Street: {:?} |Distance: {:?} || This shit is in the western province",
                        self.direction,
                        destination.place,
                        destination.street,
                        destination.distance_from_start
                    ),
                    Directions::South => println!(
                        "Direction: {:?} | Place: {:?} | Street: {:?} |Distance: {:?} || This shit is in the southern province",
                        self.direction,
                        destination.place,
                        destination.street,
                        destination.distance_from_start
                    ),
                    Directions::North => println!(
                        "Direction: {:?} | Place: {:?} | Street: {:?} |Distance: {:?} || This shit is in the northern province",
                        self.direction,
                        destination.place,
                        destination.street,
                        destination.distance_from_start
                    ),
                }
            } else {
                println!("No destination set!");
            }
        }
    }
}
