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
    dbg!(&arguments);

    println!(" -- Alone Traveller --");

    if arguments.len() < 3 {
        panic!("You must provide a origin and a destination!");
    }

    let origin_city = city_lookup(&arguments[1]).unwrap();
    println!("Origin city entered: {0}", origin_city.name);

    let destination_city = city_lookup(&arguments[2]).unwrap();
    println!("Destination city entered: {0}", destination_city.name);
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

    let result = cities.iter().find(| city | city.code == code);

    match result {
        None => None,
        Some(city) => Some(city.clone())
    }
}
