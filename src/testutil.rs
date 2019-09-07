use stops;
use utils;

pub struct EmptyCookieAccessor{ }

impl utils::CookieAccessor for EmptyCookieAccessor {
    fn get_cookie(&self, _key: &str) -> Vec<String> { return vec![]; }
}

// TODO(mrjones): Decide whether filtered prod data or synthetic data is better
fn routes_csv_data_from_prod(desired_routes: Vec<&str>) -> String {
    let prod_data = include_str!("../data/routes.txt");

    return prod_data.lines().filter(|line| {
        return line.starts_with("route_id")  || // the header
            desired_routes.iter().any(|r| line.starts_with(&format!("{},", r)));
    }).collect::<Vec<&str>>().join("\n").to_string();
}

fn stations_csv_data_from_prod() -> String {
    let prod_data =  include_str!("../data/Stations.csv");
    return prod_data.to_string();
}

fn _synthetic_routes_csv_data() -> String {
    return "route_id,agency_id,route_short_name,route_long_name,route_desc,route_type,route_url,route_color,route_text_color
//1,MTA NYCT,1,Skipped route_long_name,Skipped route_desc,1,Skipped route_url,EE352E,\n
//2,MTA NYCT,2,Skipped route_long_name,Skipped route_desc,2,Skipped route_url,EE352E,\n".to_string();
}

pub fn make_stops() -> stops::Stops {
    let routes_csv_data = routes_csv_data_from_prod(vec!["1", "2"]);
    let stations_csv_data = stations_csv_data_from_prod();
    let mut routes_csv = csv::Reader::from_reader(stringreader::StringReader::new(&routes_csv_data));

    let mut trips_csv = csv::Reader::from_reader(stringreader::StringReader::new(""));
    let mut stations_csv = csv::Reader::from_reader(stringreader::StringReader::new(&stations_csv_data));

    return stops::Stops::new_from_csv_readers(
        &mut routes_csv, &mut stations_csv, &mut trips_csv)
        .expect("parsing stops data");
}
