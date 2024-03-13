// Import necessary modules
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type V = i32;

struct CityTempReport {
    min_temp: V,
    max_temp: V,
    mean_temp: V,
}

impl fmt::Display for CityTempReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Nice report printout from integers
        write!(
            f,
            "Min: {:.1}, Max: {:.1}, Mean: {:.1}",
            self.min_temp as f32 / 10.0,
            self.max_temp as f32 / 10.0,
            self.mean_temp as f32 / 10.0
        )
    }
}

fn parse_float_as_ints(mut s: &[u8]) -> V {
    // efficent use of integers
    // https://github.com/RagnarGrootKoerkamp/1brc/blob/1fd779a2ae175b733793ca10ec94c73b769fee5e/src/main.rs

    // positive is most common
    let is_pos = s[0] != b'-';
    if !is_pos {
        s = &s[1..];
    }

    // s = abc.d
    let (a, b, c, d) = match s {
        [c, b'.', d] => (0, 0, c - b'0', d - b'0'),
        [b, c, b'.', d] => (0, b - b'0', c - b'0', d - b'0'),
        [a, b, c, b'.', d] => (a - b'0', b - b'0', c - b'0', d - b'0'),
        [c] => (0, 0, 0, c - b'0'),
        [b, c] => (0, b - b'0', c - b'0', 0),
        [a, b, c] => (a - b'0', b - b'0', c - b'0', 0),
        _ => panic!("Unknown pattern: {:?}", std::str::from_utf8(s).unwrap()),
    };

    let v = a as V * 1000 + b as V * 100 + c as V * 10 + d as V;
    if is_pos {
        v
    } else {
        -v
    }
}

struct CityTemp {
    count: i32,
    rolling_sum: V,
    max_temp: V,
    min_temp: V,
}

impl CityTemp {
    fn new(temp: V) -> Self {
        CityTemp {
            count: 0,
            rolling_sum: 0,
            max_temp: temp, // Initial temp is both max and min at this point
            min_temp: temp,
        }
    }

    fn add(&mut self, temp: V) {
        self.rolling_sum += temp;
        self.count += 1;

        // Update max and min temperatures only if necessary
        if temp > self.max_temp {
            self.max_temp = temp;
        } else if temp < self.min_temp {
            self.min_temp = temp;
        }
    }

    fn mean(&self) -> V {
        self.rolling_sum / self.count
    }
}

// Function to process each line and update city_temps
fn process_line(line: &[u8], city_temps: &mut HashMap<Vec<u8>, CityTemp>) {
    let parts = line.split(|&x| x == b';').collect::<Vec<&[u8]>>();
    let city = parts[0].to_vec();
    let temp_bytes = parts[1].strip_suffix(&[b'\n']).unwrap_or(parts[1]);
    let temp = parse_float_as_ints(temp_bytes);
    let city_temp = city_temps
        .entry(city)
        .or_insert_with(|| CityTemp::new(temp));
    city_temp.add(temp);
}

// Function to generate final report of city_temperatures
fn generate_reports(city_temps: &HashMap<Vec<u8>, CityTemp>) {
    for (city_bytes, city_temp) in city_temps.iter() {
        let report = CityTempReport {
            min_temp: city_temp.min_temp,
            max_temp: city_temp.max_temp,
            mean_temp: city_temp.mean(),
        };
        let city_name = String::from_utf8_lossy(city_bytes);
        println!("City: {}, {}", city_name, report);
    }
}

// Main function to read 1 billion rows
pub fn read_1billion_rows(path: &str) -> Result<(), io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut city_temps: HashMap<Vec<u8>, CityTemp> = HashMap::new();
    let mut line = Vec::new();

    while reader.read_until(b'\n', &mut line)? > 0 {
        process_line(&line, &mut city_temps);
        line.clear(); // Clear the buffer for the next line
    }

    generate_reports(&city_temps);

    Ok(())
}

fn main() {
    if let Err(e) = read_1billion_rows("measurements.txt") {
        eprintln!("Error reading file: {}", e);
    }
}
