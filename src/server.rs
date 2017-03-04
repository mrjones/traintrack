extern crate chrono;
extern crate chrono_tz;
extern crate hyper;
extern crate liquid;
extern crate log;
extern crate regex;
extern crate rustful;
extern crate std;
extern crate tiny_http;

use feedfetcher;
use result;
use stops;
use utils;

struct TemplateRegistry {
    compiled_templates: std::collections::HashMap<String, std::sync::Arc<liquid::Template>>,
    compile_templates_once: bool,
    templates_dir: std::path::PathBuf,
}

impl TemplateRegistry {
    fn new<P: AsRef<std::path::Path>>(templates_dir: P,
                                      compile_templates_once: bool) -> TemplateRegistry {
        let templates;

        if compile_templates_once {
            let templates_dir = templates_dir.as_ref().to_path_buf();
            let template_files = std::fs::read_dir(&templates_dir).expect(
                format!("Couldn't not read templates dir: {:?}",
                        templates_dir.as_path()).as_str());

            templates = template_files.map(|template_file: std::io::Result<std::fs::DirEntry>| {
                let template_filename = template_file.unwrap().file_name().into_string().unwrap();
                let mut full_filename = templates_dir.clone();
                full_filename.push(&template_filename);
                return (template_filename,
                        std::sync::Arc::new(
                            TemplateRegistry::must_parse(full_filename.as_path())));
            }).collect();
        } else {
            templates = std::collections::HashMap::new();
        }

        TemplateRegistry {
            compiled_templates: templates,
            compile_templates_once: compile_templates_once,
            templates_dir: templates_dir.as_ref().to_path_buf(),
        }
    }

    fn must_parse<P: AsRef<std::path::Path>>(filename: P) -> liquid::Template {
        return TemplateRegistry::parse(&filename)
            .expect(format!("Could not parse {:?}", filename.as_ref()).as_str());
    }

    fn parse<P: AsRef<std::path::Path>>(filename: P) -> result::TTResult<liquid::Template> {
        info!("Parsing template file: {:?}", filename.as_ref());
        return Ok(liquid::parse_file(&filename, Default::default())?);
    }

    fn get(&self, filename: &str) -> result::TTResult<std::sync::Arc<liquid::Template>> {
        if self.compile_templates_once {
            match self.compiled_templates.get(filename) {
                Some(template) => {
                    return Ok(template.clone());
                },
                None => {
                    return Err(result::TTError::Uncategorized(
                        format!("Unknown template: {}", filename)));
                }
            }
        } else {
            let mut full_filename = self.templates_dir.clone();
            full_filename.push(filename);
            return TemplateRegistry::parse(full_filename.as_path()).map(|tmpl| {
                return std::sync::Arc::new(tmpl);
            });
        }
    }
}

pub struct TTContext {
    stops: stops::Stops,
    fetcher: std::sync::Arc<feedfetcher::Fetcher>,
    templates: TemplateRegistry,
}

impl TTContext {
    pub fn new<P: AsRef<std::path::Path>>(stops: stops::Stops,
                                          fetcher: std::sync::Arc<feedfetcher::Fetcher>,
                                          templates_dir: P,
                                          compile_templates_once: bool,
    ) -> TTContext {

        return TTContext{
            stops: stops,
            fetcher: fetcher,
            templates: TemplateRegistry::new(
                templates_dir, compile_templates_once),
        }
    }

    fn render(&self, template_name: &str, mut context: &mut liquid::Context) -> result::TTResult<Vec<u8>> {
        use server::liquid::Renderable;
        let template = self.templates.get(template_name)?;
        let output = template.render(&mut context)?;
        let body = output.unwrap_or("No render result?".to_string());
        return Ok(body.as_bytes().to_vec());
    }

    fn fetch_now(&self) -> result::TTResult<Vec<u8>> {
        self.fetcher.fetch_once()?;
        return Ok("OK".to_string().as_bytes().to_vec());
    }
}

fn list_stations(ctx: rustful::Context) -> result::TTResult<Vec<u8>> {
    let tt_context = ctx.global.get::<TTContext>().unwrap();

    let mut stop_values = Vec::new();
    for ref stop in tt_context.stops.iter() {
        let mut m = std::collections::HashMap::new();
        m.insert("name".to_string(),
                 liquid::Value::Str(stop.name.clone()));
        m.insert("id".to_string(),
                 liquid::Value::Str(stop.id.clone()));

        let v = liquid::Value::Object(m);
        stop_values.push(v);
    }

    let mut context = liquid::Context::new();
    context.set_val("stops", liquid::Value::Array(stop_values));

    return tt_context.render("stoplist.html", &mut context);
}

fn dashboard(ctx: rustful::Context) -> result::TTResult<Vec<u8>> {
    let tt_context = ctx.global.get::<TTContext>().unwrap();

    let feed;
    match tt_context.fetcher.latest_value() {
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

    let mut context = liquid::Context::new();
    context.set_val("update_timestamp", liquid::Value::Str(
        format!("{}", feed.timestamp)));

    let mut body = String::from_utf8(tt_context.render("dashboard.html", &mut context).unwrap()).unwrap();

    //        let mut body = "<html><body>".to_string();
//        body.push_str(&format!("<p>Updated at {}</p>", feed.timestamp));
    for item in items {
        body.push_str(&format!(
            "<h2>{} : {}</h2><ul>",
            item.line,
            tt_context.stops.lookup_by_id(&item.stop_id).unwrap().name));
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

fn debug(ctx: rustful::Context) -> result::TTResult<Vec<u8>> {
    let tt_context = ctx.global.get::<TTContext>().unwrap();
    let mut context = liquid::Context::new();
    return Ok(tt_context.render("debug.html", &mut context)?);
}

fn dump_proto(ctx: rustful::Context) -> result::TTResult<Vec<u8>> {
    let tt_context = ctx.global.get::<TTContext>().unwrap();
    return match tt_context.fetcher.latest_value() {
        Some(feed) => Ok(format!(
            "Updated at: {}\n{:#?}",
            feed.timestamp,
            feed.feed).as_bytes().to_vec()),
        None => Ok("No data yet".as_bytes().to_vec()),
    }
}

struct StandardPage{
    execute: fn(rustful::Context) -> result::TTResult<Vec<u8>>
}

impl rustful::Handler for StandardPage {
    fn handle_request(&self, context: rustful::Context, response: rustful::Response) {
        match (self.execute)(context) {
            Ok(body) => response.send(body),
            Err(err) => response.send(format!("ERROR: {}", err)),
        }
    }
}

pub fn serve(context: TTContext, port: u16) {
    let global: rustful::server::Global = Box::new(context).into();
    assert!(!global.get::<TTContext>().is_none());

    let server_result = rustful::Server {
        host: port.into(),
        global: global,
        handlers: insert_routes!{
            rustful::TreeRouter::new() => {
                Get: StandardPage{execute: dashboard},
                "/debug" => Get: StandardPage{execute: debug},
                "/dump_proto" => Get: StandardPage{execute: dump_proto},
                "/stations" => Get: StandardPage{execute: list_stations},
            }
        },
        ..rustful::Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(err) => error!("could not start server: {}", err)
    }
}
