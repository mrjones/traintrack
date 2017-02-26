extern crate chrono;
extern crate chrono_tz;
extern crate hyper;
extern crate log;
extern crate regex;
extern crate std;

use feedfetcher;
use stops;
use utils;

pub struct TTServer {
    stops: stops::Stops,
    fetcher: std::sync::Arc<feedfetcher::Fetcher>,
    routing_table: Vec<(regex::Regex, fn(&TTServer, hyper::server::Request, hyper::server::Response))>,
}

impl TTServer {
    pub fn new(stops: stops::Stops, fetcher: std::sync::Arc<feedfetcher::Fetcher>) -> TTServer {
        let mut routes: Vec<(regex::Regex, fn(&TTServer, hyper::server::Request, hyper::server::Response))> = Vec::new();

        routes.push((regex::Regex::new("^/dump_proto").unwrap(),
                     TTServer::dump_proto));
        routes.push((regex::Regex::new("^/debug").unwrap(),
                     TTServer::debug));
        routes.push((regex::Regex::new(".*").unwrap(),
                     TTServer::dashboard));

        return TTServer{
            stops: stops,
            fetcher: fetcher,
            routing_table: routes,
        }
    }

    pub fn serve(server: TTServer, port: u16) {
        info!("Serving on port {}", port);

        hyper::Server::http(
            std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::new(0,0,0,0), port))).unwrap()
                .handle(server).unwrap();
    }

    fn dashboard(&self, _: hyper::server::Request, response: hyper::server::Response) {
        let feed;
        /*
        match self.fetcher.fetch_once() {
            Ok(f) => feed = f,
            Err(err) => {
                response.send(format!("Fetcher error: {}", err).as_bytes()).unwrap();
                return;
            },
        }
         */
        match self.fetcher.latest_value() {
            Some(f) => feed = f,
            None => {
                response.send("Fetcher has no data!".as_bytes()).unwrap();
                return;
            }
        }

        struct Item {
            line: String,
            stop_id: String,
            trains: Vec<(utils::Direction, chrono::datetime::DateTime<chrono::UTC>)>,
        }
        let mut items = Vec::new();

        let pois = vec![
            ("R", "R20"), ("N", "R20"), ("Q", "R20"),
            ("R", "R32"),
            ("R", "R30"), ("N", "R30"), ("Q", "R30"),
        ];
        for (route, stop) in pois {
            let trains = utils::upcoming_trains(route, stop, &feed.feed);
            items.push(Item{
                line: route.to_string(),
                stop_id: stop.to_string(),
                trains: trains,
            });
        }

        let tz = chrono_tz::America::New_York;

        let mut body = "<html><body>".to_string();
        body.push_str(&format!("<p>Updated at {}</p>", feed.timestamp));
        for item in items {
            body.push_str(&format!(
                "<h2>{} : {}</h2><ul>",
                item.line,
                self.stops.lookup_by_id(&item.stop_id).unwrap().name));
            let lis: Vec<String> = item.trains.iter().map(|&(ref direction, ref time)| {
                return format!("<li>{:?} {}</li>",
                               direction, time.with_timezone(&tz))
            }).collect();
            for li in lis {
                body.push_str(&li);
            }

            body.push_str("</ul>");
        }
        body.push_str("</body></html>");

        response.send(body.as_bytes()).unwrap();
    }

    fn debug(&self, _: hyper::server::Request, response: hyper::server::Response) {
        response.send("<html><head><title>TrainTrack debug</title></head><body><a href='/dump_proto'>/dump_proto</a></body></html>".as_bytes()).unwrap()
    }

    fn dump_proto(&self, _: hyper::server::Request, response: hyper::server::Response) {
        match self.fetcher.latest_value() {
            Some(feed) => response.send(format!(
                "Updated at: {}\n{:#?}",
                feed.timestamp,
                feed.feed).as_bytes()).unwrap(),
            None => response.send("No data yet".as_bytes()).unwrap(),
        }
    }
}

impl hyper::server::Handler for TTServer {
    fn handle(&self, req: hyper::server::Request, res: hyper::server::Response) {
        // TODO(mrjones): Abstract this out into a better router?
        info!("Routing {} {} ", req.method, req.uri);

        // TODO: Surely this isn't the right way to do this.
        let uri_string = format!("{}", req.uri);

        for &(ref pattern, ref target) in self.routing_table.iter() {
            if pattern.is_match(&uri_string) {
                target(self, req, res);
                return;
            }
        }

        panic!("No route for {} {}", req.method, req.uri);
    }
}
