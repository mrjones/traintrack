extern crate hyper;
extern crate regex;
extern crate std;

use feedfetcher;
use stops;

pub struct TTServer {
    stops: stops::Stops,
    fetcher: feedfetcher::Fetcher,
    routing_table: Vec<(regex::Regex, fn(&TTServer, hyper::server::Request, hyper::server::Response))>,
}

impl TTServer {
    pub fn new(stops: stops::Stops, fetcher: feedfetcher::Fetcher) -> TTServer {
        let mut routes: Vec<(regex::Regex, fn(&TTServer, hyper::server::Request, hyper::server::Response))> = Vec::new();

        routes.push((regex::Regex::new("/dump_proto").unwrap(),
                     TTServer::dump_proto));
        routes.push((regex::Regex::new(".*").unwrap(),
                     TTServer::debug));

        return TTServer{
            stops: stops,
            fetcher: fetcher,
            routing_table: routes,
        }
    }

    pub fn serve(server: TTServer, port: u16) {
        println!("Serving on port {}", port);

        hyper::Server::http(
            std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::new(0,0,0,0), port))).unwrap()
                .handle(server).unwrap();
    }

    fn debug(&self, request: hyper::server::Request, response: hyper::server::Response) {
        response.send("<html><head><title>TrainTrack debug</title></head><body><a href='/dump_proto'>/dump_proto</a></body></html>".as_bytes());
    }

    fn dump_proto(&self, request: hyper::server::Request, response: hyper::server::Response) {
        response.send(format!("{:#?}", self.fetcher.fetch(true).unwrap()).as_bytes());
    }
}

impl hyper::server::Handler for TTServer {
    fn handle(&self, req: hyper::server::Request, res: hyper::server::Response) {
        // TODO(mrjones): Abstract this out into a better router?
        println!("Routing {} {} ", req.method, req.uri);

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
