use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

// Original CityTemp struct without mean_temp
struct CityTemp {
    acc_temp: f64,
    count_temp: i64,
    max_temp: f64,
    min_temp: f64,
}

// Final representation includes mean
struct CityTempReport {
    min_temp: f64,
    max_temp: f64,
    mean_temp: f64,
}
pub fn read_1billion_rows(path: &str) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut city_temps: HashMap<Vec<u8>, CityTemp> = HashMap::new();
    let mut line = Vec::new();

    while reader.read_until(b'\n', &mut line)? > 0 {

        let parts = line.split(|&x| x == b';').collect::<Vec<&[u8]>>();
        if parts.len() == 2 {
            let city = parts[0].to_vec();
            let temp_bytes = parts[1];
            // Rust cannot safely convert bytes to f64 and must convert to string
            // as an intermediate before using built-in string parse logic to float
            if let Ok(temp_str) = std::str::from_utf8(temp_bytes) {
                if let Ok(temp) = temp_str.trim_end().parse::<f64>() {
                    let city_temp = city_temps.entry(city).or_insert_with(|| CityTemp {
                        acc_temp: 0.0,
                        count_temp: 0,
                        // Set max_temp and min_temp to temp for the first entry
                        max_temp: temp,
                        min_temp: temp,
                    });
                    city_temp.acc_temp += temp;
                    city_temp.count_temp += 1;
                    // Either max or min may change but not both
                    // If statement may save one check for each row
                    if temp > city_temp.max_temp {
                        city_temp.max_temp = temp;
                    } else if temp < city_temp.min_temp {
                        city_temp.min_temp = temp;
                    }
                }
            }
        }
        line.clear(); // Clear the buffer for the next line
    }

    // Mean calculation and final report
    for (city_bytes, city_temp) in city_temps.iter() {
        let mean_temp = city_temp.acc_temp / city_temp.count_temp as f64;
        let report = CityTempReport {
            min_temp: city_temp.min_temp,
            max_temp: city_temp.max_temp,
            mean_temp,
        };
        let city_name = String::from_utf8_lossy(city_bytes);
        println!("City: {}, Min: {}, Max: {}, Mean: {}", city_name, report.min_temp, report.max_temp, report.mean_temp);
    }

    Ok(())
}


fn main() {
    if let Err(e) = read_1billion_rows("measurements.txt") {
        eprintln!("Error reading file: {}", e);
    }
}
