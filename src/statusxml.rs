extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use feedproxy_api;
use result;

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

pub fn parse(xml: &[u8]) -> result::TTResult<feedproxy_api::SubwayStatus> {
    let parsed: SubwayStatusXml = serde_xml_rs::from_reader(xml)?;

    let mut result = feedproxy_api::SubwayStatus::new();

    for xml_sit in &parsed.service_delivery.situation.situations.elements {
        let mut proto_sit = feedproxy_api::Situation::new();
        proto_sit.set_summary(xml_sit.summary.clone());
        proto_sit.set_planned(xml_sit.planned);
        proto_sit.set_reason_name(xml_sit.reason_name.clone());
        for xml_line in &xml_sit.affects.journeys.journeys {
            let mut proto_line = feedproxy_api::AffectedLine::new();
            proto_line.set_line(xml_line.line_ref.clone());
            proto_line.set_direction(xml_line.direction_ref);
            proto_sit.mut_affected_line().push(proto_line);
        }
        result.mut_situation().push(proto_sit);
    }

    return Ok(result);
    /*
    return Ok(feedproxy_api::SubwayStatus{
        situations: parsed.service_delivery.situation.situations.elements.iter().map(
            |e: &PtSituationElement| {
                return Situation{
                    summary: e.summary.clone(),
                    description: e.description.clone(),
                    long_description: e.long_description.clone(),
                    planned: e.planned,
                    reason_name: e.reason_name.clone(),
                    priority: e.message_priority,
                    affected_lines: e.affects.journeys.journeys.iter().map(
                        |j: &VehicleJourney| {
                            return AffectedLine{
                                line: j.line_ref.clone(),
                                direction: j.direction_ref,
                            };
                        }).collect(),
                };
        }).collect(),
    });*/
}
