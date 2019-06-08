extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate tendril;
extern crate xml5ever;

use feedproxy_api;
use result;
use webclient_api;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SubwayStatusXml {
    service_delivery: ServiceDeliveryNode
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ServiceDeliveryNode {
    response_timestamp: String,

    #[serde(rename="SituationExchangeDelivery")]
    situation: SituationExchangeNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SituationExchangeNode {
    response_timestamp: String,
    status: String,
    situations: SituationNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SituationNode {
    #[serde(rename="PtSituationElement")]
    elements: Vec<PtSituationElement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PtSituationElement {
    creation_time: String,
    situation_number: String,
    summary: String,
    description: String,
    long_description: String,
    planned: bool,
    reason_name: String,
    message_priority: i32,
    affects: AffectsNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AffectsNode {
    #[serde(rename="VehicleJourneys")]
    journeys: VehicleJourneyList,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct VehicleJourneyList {
    #[serde(rename="AffectedVehicleJourney")]
    journeys: Vec<VehicleJourney>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct VehicleJourney {
    line_ref: String,
    direction_ref: i32,
}

struct XmlStripper<'a> {
    accum: &'a mut String,
}

impl<'a> XmlStripper<'a> {
    fn new(accum: &'a mut String) -> XmlStripper {
        return XmlStripper{accum: accum};
    }
}

impl <'a> xml5ever::tokenizer::TokenSink for XmlStripper<'a>  {
    fn process_token(&mut self, token: xml5ever::tokenizer::Token) {
        match token {
            xml5ever::tokenizer::CharacterTokens(b) => {
                self.accum.push_str(&b.to_string());
                // TODO(mrjones): Only insert space for <div>, <br> etc.
                self.accum.push(' ');
            },
            _ => { },
        }
    }
}

fn strip_xml(input: &str) -> String {
    println!("STRIPPING: {}", input);
    let mut result = String::new();
    {
        let mut stripper = XmlStripper::new(&mut result);
        let mut tok = xml5ever::tokenizer::XmlTokenizer::new(
            stripper,
            xml5ever::tokenizer::XmlTokenizerOpts {
                profile: true,
                ..Default::default()
            },
        );
        let input = tendril::StrTendril::from(input);
        let mut input_buffer = xml5ever::buffer_queue::BufferQueue::new();
        input_buffer.push_back(input.try_reinterpret().unwrap());
        tok.feed(&mut input_buffer);
        tok.end();
    }

    println!("Result: {}", result);
    return result;
}

pub fn parse(xml: &[u8]) -> result::TTResult<feedproxy_api::SubwayStatus> {
    let parsed: SubwayStatusXml = serde_xml_rs::from_reader(xml)?;

    let mut result = feedproxy_api::SubwayStatus::new();

    for xml_sit in &parsed.service_delivery.situation.situations.elements {
        let mut proto_sit = webclient_api::SubwayStatusMessage::new();
        proto_sit.set_summary(xml_sit.summary.clone());
        proto_sit.set_long_description(strip_xml(&xml_sit.long_description));
//        proto_sit.set_long_description(xml_sit.long_description.clone());
        proto_sit.set_planned(xml_sit.planned);
        proto_sit.set_reason_name(xml_sit.reason_name.clone());
        proto_sit.set_priority(xml_sit.message_priority);
        match chrono::DateTime::parse_from_rfc3339(&xml_sit.creation_time) {
            Ok(date) => proto_sit.set_publish_timestamp(date.timestamp()),
            _ => {},
        };
        for xml_line in &xml_sit.affects.journeys.journeys {
            let mut proto_line = webclient_api::AffectedLineStatus::new();
            proto_line.set_line(xml_line.line_ref.clone().replace("MTA NYCT_", ""));
            proto_line.set_direction(
                match xml_line.direction_ref {
                    0 => webclient_api::Direction::UPTOWN,
                    _ => webclient_api::Direction::DOWNTOWN,
                });
            proto_sit.mut_affected_line().push(proto_line);
        }
        result.mut_status().push(proto_sit);
    }

    return Ok(result);
}
