use geoutils::Location;
use std::env;

#[derive(Clone)]
struct City {
    code: String,
    name: String,
    lat: f32,
    lon: f32,
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    println!(" -- Alone Traveller --");

    if arguments.len() < 3 {
        panic!("You must provide a origin and a destination!");
    }

    let origin_city: City;

    if let Some(city) = city_lookup(&arguments[1]) {
        origin_city = city;
        println!("Origin city entered: {0}", origin_city.name);
    } else {
        println!(
            "No Origin or a invalid origin Has been Provided by User. -----> {}",
            &arguments[1]
        );
        std::process::exit(-1);
    }

    let destination_city: City;

    if let Some(city) = city_lookup(&arguments[2]) {
        destination_city = city;
        println!("Destination city entered: {0}", destination_city.name);
    } else {
        println!(
            "No Origin or a invalid origin Has been Provided by User. -----> {}",
            &arguments[2]
        );
        std::process::exit(-1);
    }

    let distance = calculate_distance(
        origin_city.lat,
        origin_city.lon,
        destination_city.lat,
        destination_city.lon,
    );

    println!(
        "The distance below these two points are: {:.2} KM",
        distance
    );
    std::process::exit(0);
}

fn city_lookup(code: &str) -> Option<City> {
    let cities: [City; 6] = [
        City {
            code: String::from("FLN"),
            name: String::from("Florianópolis"),
            lat: -27.5954,
            lon: -48.5480,
        },
        City {
            code: String::from("JOI"),
            name: String::from("Joinville"),
            lat: -26.3045,
            lon: -48.8486,
        },
        City {
            code: String::from("BLU"),
            name: String::from("Blumenau"),
            lat: -26.9186,
            lon: -49.0663,
        },
        City {
            code: String::from("CRI"),
            name: String::from("Criciúma"),
            lat: -28.6723,
            lon: -49.3730,
        },
        City {
            code: String::from("CHA"),
            name: String::from("Chapecó"),
            lat: -27.1007,
            lon: -52.6152,
        },
        City {
            code: String::from("LGS"),
            name: String::from("Lages"),
            lat: -27.8150,
            lon: -50.3259,
        },
    ];

    let result = cities.iter().find(|city| city.code == code);

    match result {
        None => None,
        Some(city) => Some(city.clone()),
    }
}

fn calculate_distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f64 {
    let origin = Location::new(lat1, lon1);
    let destination = Location::new(lat2, lon2);
    origin.distance_to(&destination).unwrap().meters() / 1000.0
}
