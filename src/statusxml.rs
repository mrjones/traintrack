extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate tendril;
extern crate xml5ever;

use anyhow::Context;

use crate::feedproxy_api;
use crate::webclient_api;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SubwayStatusXml {
    service_delivery: ServiceDeliveryNode
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct ServiceDeliveryNode {
    response_timestamp: String,

    #[serde(rename="SituationExchangeDelivery")]
    situation: SituationExchangeNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    direction_ref: Option<i32>,
}

struct XmlStripper<'a> {
    accum: std::cell::RefCell<&'a mut String>,
}

impl<'a> XmlStripper<'a> {
    fn new(accum: &'a mut String) -> XmlStripper<'a> {
        return XmlStripper{accum: std::cell::RefCell::new(accum)};
    }
}

impl <'a> xml5ever::tokenizer::TokenSink for XmlStripper<'a>  {
    fn process_token(&self, token: xml5ever::tokenizer::Token) {
        match token {
            xml5ever::tokenizer::CharacterTokens(b) => {
                self.accum.borrow_mut().push_str(&b.to_string());
                // TODO(mrjones): Only insert space for <div>, <br> etc.
                self.accum.borrow_mut().push(' ');
            },
            _ => { },
        }
    }
}

fn strip_xml(input: &str) -> String {
    let mut result = String::new();
    {
        let stripper = XmlStripper::new(&mut result);
        let mut tok = xml5ever::tokenizer::XmlTokenizer::new(
            stripper,
            xml5ever::tokenizer::XmlTokenizerOpts {
                profile: true,
                ..Default::default()
            },
        );
        let input = tendril::StrTendril::from(input);
        let mut input_buffer = xml5ever::buffer_queue::BufferQueue::default();
        input_buffer.push_back(input.try_reinterpret().unwrap());
        tok.feed(&mut input_buffer);
        tok.end();
    }

    return result;
}

pub fn parse(xml: &[u8]) -> anyhow::Result<feedproxy_api::SubwayStatus> {
    let parsed: SubwayStatusXml = serde_xml_rs::from_reader(xml)
        .context("during serde_xml_rs::from_reader")?;

    let mut result = feedproxy_api::SubwayStatus::default();

    for xml_sit in &parsed.service_delivery.situation.situations.elements {
        let long_desc = strip_xml(&xml_sit.long_description);

        let mut proto_sit = webclient_api::SubwayStatusMessage::default();
        proto_sit.summary = Some(xml_sit.summary.clone());
        if long_desc.replace(" ", "") != xml_sit.summary.replace(" ", "") {
            proto_sit.long_description = Some(long_desc);
        }
        proto_sit.planned = Some(xml_sit.planned);
        proto_sit.reason_name = Some(xml_sit.reason_name.clone());
        proto_sit.priority = Some(xml_sit.message_priority);
        proto_sit.id = Some(xml_sit.situation_number.clone());  // Take a hash maybe?
        match chrono::DateTime::parse_from_rfc3339(&xml_sit.creation_time) {
            Ok(date) => proto_sit.publish_timestamp = Some(date.timestamp()),
            _ => {},
        };
        for xml_line in &xml_sit.affects.journeys.journeys {
            let mut proto_line = webclient_api::AffectedLineStatus::default();
            proto_line.line = Some(xml_line.line_ref.clone().replace("MTA NYCT_", ""));
            proto_line.set_direction(
                match xml_line.direction_ref {
                    Some(0) => webclient_api::Direction::Uptown,
                    _ => webclient_api::Direction::Downtown,
                });
            proto_sit.affected_line.push(proto_line);
        }
        result.status.push(proto_sit);
    }

    return Ok(result);
}
