use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("./measurements.txt").unwrap();
    let f = BufReader::new(f);

    // stations[station] -> (min, sum, max, count)
    let mut stations: BTreeMap<String, (f32, f32, f32, usize)> = BTreeMap::new();

    for line in f.lines() {
        let raw = line.unwrap();
        if let Some((station, data)) = raw.trim().split_once(';') {
            stations
                .entry(station.to_string())
                .or_insert((f32::MAX, 0.0, f32::MIN, 0));
            if let Some(entry) = stations.get_mut(station) {
                let data = match data.parse::<f32>() {
                    Ok(num) => num,
                    Err(e) => {
                        eprintln!("{}: {:?}", e, raw);
                        panic!()
                    }
                };
                entry.3 += 1;
                entry.1 += data;
                entry.0 = entry.0.min(data);
                entry.2 = entry.2.max(data);
            }
        }
    }

    let mut entries = stations.iter().peekable();
    print!("{{");
    while let Some((station, (min, sum, max, count))) = entries.next() {
        print!(
            "{}={:.1}/{:.1}/{:.1}",
            station,
            min,
            (sum / *count as f32),
            max
        );
        if entries.peek().is_some() {
            print!(", ")
        }
    }
    print!("}}")
}
