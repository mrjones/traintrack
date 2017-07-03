extern crate csv;
extern crate std;

use result;

#[derive(Debug, Clone)]
pub struct Stop {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
}

#[derive(Clone)]
pub struct Route {
    pub id: String,
    pub color: String,
}

pub struct Stops {
    stops: std::collections::HashMap<String, Stop>,
//    trip_ids_by_route: std::collections::HashMap<String, Vec<String>>,
//    stop_ids_by_trip: std::collections::HashMap<String, Vec<String>>,
    stops_by_route: std::collections::HashMap<String, Vec<Stop>>,
    routes: Vec<Route>,
}

// For Stations.csv
#[derive(Debug, RustcDecodable)]
struct StationCsvRecord {
    mta_station_id: String,
    complex_id: String,
    gtfs_stop_id: String,
    division: String,
    line: String,
    name: String,
    borough: String,
    daytime_routes: String,
    structure: String,
    latitude: f32,
    longitude: f32,
}

// For Stops.txt
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
    parent_station: Option<String>,
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

    pub fn lines(&self) -> Vec<Route> {
        return self.routes.clone();
    }

    pub fn stops_for_route(&self, route_id: &str) -> result::TTResult<&Vec<Stop>> {
        return self.stops_by_route.get(route_id).ok_or(
            result::quick_err(&format!("No stops for route: {}", route_id)));
    }

    pub fn compute_stops_for_route(
        trip_ids_by_route: &std::collections::HashMap<String, Vec<String>>,
        stop_ids_by_trip: &std::collections::HashMap<String, Vec<String>>,
        stops: &std::collections::HashMap<String, Stop>,
        route_id: &str) -> result::TTResult<Vec<Stop>> {
        let mut stop_ids_for_route = std::collections::HashSet::new();
        for trip_id in trip_ids_by_route.get(route_id).unwrap_or(&vec![]).iter() {
            for stop_id in stop_ids_by_trip.get(trip_id).ok_or(
                result::quick_err("No stops for trip"))?.iter() {
                stop_ids_for_route.insert(stop_id);
            }
        }

        let mut parent_stop_ids = std::collections::HashSet::new();
        for stop_id in stop_ids_for_route {
            let stop = stops.get(stop_id).ok_or(
                result::quick_err(
                    format!("No stop with ID: {}", stop_id).as_ref()))?;
            match stop.parent_id {
                None => { parent_stop_ids.insert(stop_id) },
                Some(ref parent_id) => { parent_stop_ids.insert(parent_id) },
            };
        }


        let mut stops_for_this_route: Vec<Stop> = Vec::new();
        for stop_id in parent_stop_ids {
            let stop = stops.get(stop_id).ok_or(
                result::quick_err(
                    format!("No stop with ID: {}", stop_id).as_ref()))?;
            stops_for_this_route.push(stop.to_owned());
        }

        return Ok(stops_for_this_route);
    }

    pub fn new_from_csvs(filename: &str) -> result::TTResult<Stops> {
        #[derive(Eq, PartialEq)]
        enum StopsLogic { GTFS, MTA };
        let logic = StopsLogic::MTA;

        let gtfs_directory = std::path::PathBuf::from(filename.to_string());
        info!("Parsing routes.txt");
        let mut routes_file = gtfs_directory.clone();
        routes_file.push("routes.txt");
        let mut routes = Vec::new();
        {
            let mut reader = csv::Reader::from_file(routes_file)?;
            for record in reader.decode() {
                let record: RouteCsvRecord = record?;
                routes.push(Route{
                    id: record.route_id,
                    color: record.route_color,
                });
            }
        }

        if logic == StopsLogic::MTA {
            info!("Parsing MTA's Stations.csv");
            let mut routes_file = gtfs_directory.clone();
            routes_file.push("Stations.csv");
            let mut stops = std::collections::HashMap::new();
            let mut stops_by_route: std::collections::HashMap<String, Vec<Stop>> = std::collections::HashMap::new();
            {
                let mut reader = csv::Reader::from_file(routes_file)?;
                for record in reader.decode() {
                    let record: StationCsvRecord = record?;
                    let stop = Stop{
                        id: record.gtfs_stop_id.clone(),
                        parent_id: None,
                        name: record.name,
                    };

                    stops.insert(
                        record.gtfs_stop_id.clone(), stop.clone());

                    for route in record.daytime_routes.split(" ") {
                        if stops_by_route.contains_key(route) {
                            stops_by_route.get_mut(route).unwrap().push(
                                stop.clone());
                        } else {
                            stops_by_route.insert(
                                route.to_string(),
                                vec![stop.clone()]);
                        }
                    }
                }
            }

            return Ok(Stops{
                stops: stops,
//                trip_ids_by_route: std::collections::HashMap::new(),
//                stop_ids_by_trip: std::collections::HashMap::new(),
                stops_by_route: stops_by_route,
                routes: routes,
            });
        } else if logic == StopsLogic::GTFS {
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
                        parent_id: record.parent_station.clone(),
                        name: record.stop_name.clone(),
                    });
                }
            }

            info!("Parsing trips.txt");
            let mut trips_file = gtfs_directory.clone();
            trips_file.push("trips.txt");
            let mut trip_ids_by_route: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
            {
                let mut reader = csv::Reader::from_file(trips_file)?;
                for record in reader.decode() {
                    let record: TripCsvRecord = record?;
                    if trip_ids_by_route.contains_key(&record.route_id) {
                        trip_ids_by_route.get_mut(&record.route_id).unwrap().push(
                            record.trip_id);
                    } else {
                        trip_ids_by_route.insert(
                            record.route_id, vec![record.trip_id]);
                    }
                }
            }

            info!("Parsing stop_times.txt");
            let mut trip_stops_file = gtfs_directory.clone();
            trip_stops_file.push("stop_times.txt");
            let mut stop_ids_by_trip: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
            {
                let mut reader = csv::Reader::from_file(trip_stops_file)?;
                for record in reader.decode() {
                    let record: StopTimeCsvRecord = record?;
                    if stop_ids_by_trip.contains_key(&record.trip_id) {
                        stop_ids_by_trip.get_mut(&record.trip_id).unwrap().push(
                            record.stop_id);
                    } else {
                        stop_ids_by_trip.insert(
                            record.trip_id, vec![record.stop_id]);
                    }
                }
            }

            info!("Computing stops per route");
            let mut stops_by_route = std::collections::HashMap::new();
            for route in &routes {
                stops_by_route.insert(
                    route.id.to_string(), Stops::compute_stops_for_route(
                        &trip_ids_by_route,
                        &stop_ids_by_trip,
                        &stops,
                        route.id.as_ref())?);
            }

            info!("Done partsing GTFS files");
            return Ok(Stops{
                stops: stops,
//                trip_ids_by_route: trip_ids_by_route,
//                stop_ids_by_trip: stop_ids_by_trip,
                stops_by_route: stops_by_route,
                routes: routes,
            });
        } else {
            return Err(result::quick_err("Uknown stops logic"));
        }
    }
}
