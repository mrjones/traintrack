extern crate csv;
extern crate std;

use result;

pub struct Stop {
    pub id: String,
    pub name: String,
}

pub struct Stops {
    stops: std::collections::HashMap<String, Stop>,
}

#[derive(Debug, RustcDecodable)]
struct StopCsvRecord {
    stop_id: String,
    stop_code: Option<String>,
    stop_name: String,
    stop_desc: Option<String>,
    stop_lat: f32,
    stop_lon: f32,
    zone_id: Option<String>,
    stop_url: Option<String>,
    location_type: Option<i32>,
    parent_station: String,
}

impl Stops {
    pub fn iter(&self) -> std::collections::hash_map::Values<String, Stop> {
        return self.stops.values();
    }

    pub fn lookup_by_id(&self, id: &str) -> Option<&Stop> {
        return self.stops.get(id);
    }

    pub fn new_from_csv(filename: &str) -> result::TTResult<Stops> {
        let mut reader = csv::Reader::from_file(filename)?;
        let mut stops = std::collections::HashMap::new();
        for record in reader.decode() {
            let record: StopCsvRecord = record?;
            stops.insert(record.stop_id.clone(), Stop{
                id: record.stop_id.clone(),
                name: record.stop_name.clone(),
            });
        }

        return Ok(Stops{
            stops: stops,
        });
    }
}
