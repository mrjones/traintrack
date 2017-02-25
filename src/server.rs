extern crate hyper;
extern crate reroute;
extern crate std;

use feedfetcher;
use stops;

pub struct TTServer {
    stops: stops::Stops,
    fetcher: feedfetcher::Fetcher,
}

impl TTServer {
    pub fn new(stops: stops::Stops, fetcher: feedfetcher::Fetcher) -> TTServer {
        return TTServer{
            stops: stops,
            fetcher: fetcher,
        }
    }

    pub fn serve(server: TTServer, port: u16) {
        let mut router = reroute::RouterBuilder::new();

        router.get("/dump_proto", move |req, resp, captures| {
            server.dump_proto(req, resp);
        });

        println!("Serving on port {}", port);
        hyper::Server::http(
            std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::new(0,0,0,0), port))).unwrap()
                .handle(router.finalize().unwrap()).unwrap();
    }

    fn dump_proto(&self, request: hyper::server::Request, response: hyper::server::Response) {
        response.send(format!("{:?}", self.fetcher.fetch(true).unwrap()).as_bytes());
    }

}
