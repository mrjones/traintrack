extern crate chrono;
extern crate chrono_tz;
extern crate hyper;
extern crate liquid;
extern crate log;
extern crate regex;
extern crate std;
extern crate tiny_http;

use feedfetcher;
use result;
use stops;
use utils;

pub struct TTServer {
    stops: stops::Stops,
    fetcher: std::sync::Arc<feedfetcher::Fetcher>,
    routing_table: Vec<(regex::Regex, fn(&TTServer) -> result::TTResult<Vec<u8>>)>,
}

impl TTServer {
    pub fn new(stops: stops::Stops, fetcher: std::sync::Arc<feedfetcher::Fetcher>) -> TTServer {
        let mut routes: Vec<(regex::Regex, fn(&TTServer) -> result::TTResult<Vec<u8>>)> = Vec::new();

        routes.push((regex::Regex::new("^/dump_proto").unwrap(),
                     TTServer::dump_proto));
        routes.push((regex::Regex::new("^/debug").unwrap(),
                     TTServer::debug));
        routes.push((regex::Regex::new("^/fetch_now").unwrap(),
                     TTServer::fetch_now));
        routes.push((regex::Regex::new("^/station").unwrap(),
                     TTServer::station));
        routes.push((regex::Regex::new(".*").unwrap(),
                     TTServer::dashboard));

        return TTServer{
            stops: stops,
            fetcher: fetcher,
            routing_table: routes,
        }
    }

    pub fn serve(tt_server: TTServer, port: u16) {
        info!("Serving on port {}", port);

        let bind_addr = std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::new(0,0,0,0), port));

        let hyper = false;

        if hyper {
            hyper::Server::http(bind_addr).unwrap()
                .handle(tt_server).unwrap();
        } else {
            let tt_server = std::sync::Arc::new(tt_server);
            let tiny_server = std::sync::Arc::new(
                tiny_http::Server::http(bind_addr).unwrap());
            let mut handles = Vec::new();

            for i in 0..4 {
                let tiny_server = tiny_server.clone();
                let tt_server = tt_server.clone();

                let handle = std::thread::Builder::new()
                    .name(format!("TinyHTTP_{}", i))
                    .spawn(move || {
                        loop {
                            let request = tiny_server.recv().unwrap();
                            tt_server.tiny_dispatch(request);
                        }
                    }).unwrap();

                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.join();
            }
        }
    }

    fn hyper_wrap(logic: &fn(&TTServer) -> result::TTResult<Vec<u8>>,
                  tt_server: &TTServer,
                  hyper_response: hyper::server::Response) {
        match logic(tt_server) {
            Ok(response) => {
                hyper_response.send(&response).unwrap();
            },
            Err(err) => {
                hyper_response.send(format!("ERROR: {}", err).as_bytes()).unwrap();
            }
        }
    }

    fn station(&self) -> result::TTResult<Vec<u8>> {
        let mut stop_values = Vec::new();
        for ref stop in self.stops.iter() {
            let mut m = std::collections::HashMap::new();
            m.insert("name".to_string(),
                     liquid::Value::Str(stop.name.clone()));
            m.insert("id".to_string(),
                     liquid::Value::Str(stop.id.clone()));

            let v = liquid::Value::Object(m);
            stop_values.push(v);
        }

        use server::liquid::Renderable;
        let template = liquid::parse_file("./templates/stoplist.html",
                                          Default::default())?;
        let mut context = liquid::Context::new();
        context.set_val("stops", liquid::Value::Array(stop_values));

        let output = template.render(&mut context)?;
        let mut body = output.unwrap_or("No render result?".to_string());
        return Ok(body.as_bytes().to_vec());
    }

    fn dashboard(&self) -> result::TTResult<Vec<u8>> {
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
                return Ok("Fetcher has no data!".as_bytes().to_vec());
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

        use server::liquid::Renderable;

        // TODO(mrjones): don't do per-request
        let template = liquid::parse_file("./templates/dashboard.html",
                                          Default::default())?;
        let mut context = liquid::Context::new();
        context.set_val("update_timestamp", liquid::Value::Str(
            format!("{}", feed.timestamp)));

        let output = template.render(&mut context)?;
        let mut body = output.unwrap_or("No render result?".to_string());

//        let mut body = "<html><body>".to_string();
//        body.push_str(&format!("<p>Updated at {}</p>", feed.timestamp));
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

        return Ok(body.as_bytes().to_vec());
    }

    fn fetch_now(&self) -> result::TTResult<Vec<u8>> {
        self.fetcher.fetch_once()?;
        return Ok("OK".to_string().as_bytes().to_vec());
    }

    fn debug(&self) -> result::TTResult<Vec<u8>> {
        use server::liquid::Renderable;

        // TODO(mrjones): don't do per-request
        let template = liquid::parse_file("./templates/debug.html",
                                          Default::default())?;
        let mut context = liquid::Context::new();

        let output = template.render(&mut context)?;
        return Ok(output.unwrap_or("No render result?".to_string()).as_bytes().to_vec());
    }

    fn dump_proto(&self) -> result::TTResult<Vec<u8>> {
        return match self.fetcher.latest_value() {
            Some(feed) => Ok(format!(
                "Updated at: {}\n{:#?}",
                feed.timestamp,
                feed.feed).as_bytes().to_vec()),
            None => Ok("No data yet".as_bytes().to_vec()),
        }
    }

    fn tiny_dispatch(&self, request: tiny_http::Request) {
        info!("TINY Routing {} {} ", request.method(), request.url());

        // TODO: Surely this isn't the right way to do this.
        let uri_string = format!("{}", request.url());

        for &(ref pattern, ref target) in self.routing_table.iter() {
            if pattern.is_match(&uri_string) {
                match target(&self) {
                    Ok(response) => {
                        request.respond(tiny_http::Response::from_data(
                            response)).unwrap();
                    },
                    Err(err) => {
                        request.respond(tiny_http::Response::from_data(
                            format!("ERROR: {}", err).as_bytes())).unwrap();
                    }
                }
                return;
            }
        }

        panic!("No route for {}", uri_string);

    }
}

impl hyper::server::Handler for TTServer {
    fn handle(&self, req: hyper::server::Request, res: hyper::server::Response) {
        // TODO(mrjones): Abstract this out into a better router?
        info!("Hyper Routing {} {} ", req.method, req.uri);

        // TODO: Surely this isn't the right way to do this.
        let uri_string = format!("{}", req.uri);

        for &(ref pattern, ref target) in self.routing_table.iter() {
            if pattern.is_match(&uri_string) {
                TTServer::hyper_wrap(target, &self, res);
                return;
            }
        }

        panic!("No route for {} {}", req.method, req.uri);
    }
}
