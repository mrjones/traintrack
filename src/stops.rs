extern crate csv;
extern crate std;

use result;

pub struct Stop {
    pub id: String,
    pub name: String,
}

pub struct Trip {
    pub route_id: String,
    pub trip_id: String,
}

pub struct TripStop {
    pub trip_id: String,
    pub stop_id: String,
}

pub struct Stops {
    stops: std::collections::HashMap<String, Stop>,
    trips: std::collections::HashMap<String, Trip>,
    trip_stops: std::collections::HashMap<String, TripStop>,
    routes: Vec<String>,
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

#[derive(Debug, RustcDecodable)]
struct TripCsvRecord {
    route_id: String,
    service_id: String,
    trip_id: String,
    trip_headsign: String,
    direction_id: i32,
    block_id: Option<String>,
    shape_id: String,
}

#[derive(Debug, RustcDecodable)]
struct StopTimeCsvRecord {
    trip_id: String,
    arrival_time: String,
    departure_time: String,
    stop_id: String,
    stop_sequence: i32,
    stop_headsign: Option<String>,
    pickup_type: i32,
    drop_off_type: i32,
    shape_dist_traveled: Option<String>
}

#[derive(Debug, RustcDecodable)]
struct RouteCsvRecord {
    route_id: String,
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: i32,
    route_url: String,
    route_color: String,
    route_text_color: Option<String>
}


impl Stops {
    pub fn iter(&self) -> std::collections::hash_map::Values<String, Stop> {
        return self.stops.values();
    }

    pub fn lookup_by_id(&self, id: &str) -> Option<&Stop> {
        return self.stops.get(id);
    }

    pub fn routes(&self) -> Vec<String> {
        return self.routes.clone();
    }

    pub fn stops_for_route(&self, route_id: &str) -> result::TTResult<Vec<&Stop>> {
        let mut trip_ids_for_route = std::collections::HashSet::new();
        for trip in self.trips.values() {
            if trip.route_id == route_id {
                trip_ids_for_route.insert(&trip.trip_id);
            }
        }

        let mut stop_ids_for_route = std::collections::HashSet::new();
        for trip_stop in self.trip_stops.values() {
            if trip_ids_for_route.contains(&trip_stop.trip_id) {
                stop_ids_for_route.insert(&trip_stop.stop_id);
            }
        }

        let mut stops = Vec::new();
        for stop_id in stop_ids_for_route {
            stops.push(self.stops.get(stop_id).ok_or(
                result::TTError::Uncategorized(
                    format!("No stop with ID: {}", stop_id)))?);
        }

        return Ok(stops);
    }

    pub fn new_from_csvs(filename: &str) -> result::TTResult<Stops> {
        info!("Parsing GTFS files");
        let gtfs_directory = std::path::PathBuf::from(filename.to_string());

        info!("Parsing routes.txt");
        let mut routes_file = gtfs_directory.clone();
        routes_file.push("routes.txt");
        let mut routes = Vec::new();
        {
            let mut reader = csv::Reader::from_file(routes_file)?;
            for record in reader.decode() {
                let record: RouteCsvRecord = record?;
                routes.push(record.route_id);
            }
        }

        info!("Parsing stops.txt");
        let mut stops_file = gtfs_directory.clone();
        stops_file.push("stops.txt");
        let mut stops = std::collections::HashMap::new();
        {
            let mut reader = csv::Reader::from_file(stops_file)?;
            for record in reader.decode() {
                let record: StopCsvRecord = record?;
                stops.insert(record.stop_id.clone(), Stop{
                    id: record.stop_id.clone(),
                    name: record.stop_name.clone(),
                });
            }
        }

        info!("Parsing trips.txt");
        let mut trips_file = gtfs_directory.clone();
        trips_file.push("trips.txt");
        let mut trips = std::collections::HashMap::new();
        {
            let mut reader = csv::Reader::from_file(trips_file)?;
            for record in reader.decode() {
                let record: TripCsvRecord = record?;
                trips.insert(record.trip_id.clone(), Trip{
                    route_id: record.route_id.clone(),
                    trip_id: record.trip_id.clone(),
                });
            }
        }

        info!("Parsing stop_times.txt");
        let mut trip_stops_file = gtfs_directory.clone();
        trip_stops_file.push("stop_times.txt");
        let mut trip_stops = std::collections::HashMap::new();
        {
            let mut reader = csv::Reader::from_file(trip_stops_file)?;
            for record in reader.decode() {
                let record: StopTimeCsvRecord = record?;
                trip_stops.insert(record.trip_id.clone(), TripStop{
                    trip_id: record.trip_id.clone(),
                    stop_id: record.stop_id.clone(),
                });
            }
        }


        info!("Done partsing GTFS files");
        return Ok(Stops{
            stops: stops,
            trips: trips,
            trip_stops: trip_stops,
            routes: routes,
        });
    }
}
