use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn read_file_to_points(path: &str) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut city_temps: HashMap<String, Vec<f64>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() == 2 {
            let city = parts[0].to_string();
            if let Ok(temp) = parts[1].parse::<f64>() {
                city_temps.entry(city).or_insert_with(Vec::new).push(temp);
            }
        }
    }

    for (city, temps) in city_temps {
        let (min, max, mean) = calculate_stats(&temps);
        println!("City: {}, Min: {}, Max: {}, Mean: {}", city, min, max, mean);
    }
    Ok(())
}


fn calculate_stats(temps: &[f64]) -> (f64, f64, f64) {
    let sum: f64 = temps.iter().sum();
    let count = temps.len() as f64;
    let min = temps.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = sum / count;
    (min, max, mean)
}


fn main() {
    if let Err(e) = read_file_to_points("measurements.txt") {
        eprintln!("Error reading file: {}", e);
    }
}
