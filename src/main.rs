use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct CityTemp {
    acc_temp: f64,
    count_temp: i64,
    max_temp: f64,
    min_temp: f64,
    mean_temp: f64,
}
pub fn read_1billion_rows(path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut city_temps: HashMap<String, CityTemp> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() == 2 {
            let city = parts[0].to_string();
            if let Ok(temp) = parts[1].parse::<f64>() {

                let city_temp = city_temps.entry(city).or_insert(CityTemp {
                    acc_temp: 0.0,
                    count_temp: 0,
                    max_temp: -999f64,
                    min_temp: 999f64,
                    mean_temp: 0.0,
                });
                city_temp.acc_temp += temp;
                city_temp.count_temp += 1;
                city_temp.max_temp = city_temp.max_temp.max(temp);
                city_temp.min_temp = city_temp.min_temp.min(temp);
            }
        }
    }

    // second pass for mean
    for (city, city_temp) in city_temps.iter_mut() {
        city_temp.mean_temp = city_temp.acc_temp / city_temp.count_temp as f64;
        println!("City: {}, Min: {}, Max: {}, Mean: {}", city, city_temp.min_temp, city_temp.max_temp, city_temp.mean_temp);
    }
    Ok(())
}


fn main() {
    if let Err(e) = read_1billion_rows("measurements.txt") {
        eprintln!("Error reading file: {}", e);
    }
}
