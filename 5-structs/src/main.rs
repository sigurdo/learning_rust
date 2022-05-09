#[derive(Debug)]
struct Car {
    odo: f64,              // km
    fuel_consumption: f64, // L/km
    fuel_level: f64,       // L
    fuel_capacity: f64,    // L
    wheel_count: u32,
    seat_count: u32,
}

#[derive(Debug)]
struct Route {
    from: String,
    to: String,
    distance: f64, // km
}

impl Route {
    fn new(from: String, to: String, distance: f64) -> Route {
        Route { from, to, distance }
    }
    fn drive(&self, car: &mut Car) {
        // Driving route with car `car`
        car.odo += self.distance;
        car.fuel_level -= car.fuel_consumption * self.distance;
    }
}

impl Car {
    fn new_sportscar() -> Car {
        Car {
            odo: 0.,
            fuel_consumption: 0.02,
            fuel_level: 0.,
            fuel_capacity: 25.,
            wheel_count: 4,
            seat_count: 2,
        }
    }
    fn new_station_wagon() -> Car {
        Car {
            odo: 0.,
            fuel_consumption: 0.01,
            fuel_level: 0.,
            fuel_capacity: 40.,
            wheel_count: 4,
            seat_count: 5,
        }
    }
    fn refill(&mut self) {
        self.fuel_level = self.fuel_capacity;
    }
}

fn main() {
    let mut garage = [
        Car::new_sportscar(),
        Car::new_sportscar(),
        Car::new_station_wagon(),
    ];

    dbg!(garage.len());
    dbg!(&garage);

    for car in &mut garage {
        car.refill();
    }

    dbg!(&garage);

    let oslo_trondheim = Route::new(String::from("Oslo"), String::from("Trondheim"), 550.);

    oslo_trondheim.drive(&mut garage[0]);
    oslo_trondheim.drive(&mut garage[2]);

    dbg!(&garage);

    println!("Hello, world!");
}
