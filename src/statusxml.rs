extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use result;

// TODO(mrjones): Replace this with an API proto?
pub struct SubwayStatus {
    pub situations: Vec<Situation>,
}

pub struct Situation {
    pub summary: String,
    pub description: String,
    pub long_description: String,
    pub planned: bool,
    pub reason_name: String,
    pub priority: i32,
    pub affected_lines: Vec<AffectedLine>,
}


pub struct AffectedLine {
    pub line: String,
    pub direction: i32,
}

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

pub fn parse(xml: &[u8]) -> result::TTResult<SubwayStatus> {
    let parsed: SubwayStatusXml = serde_xml_rs::from_reader(xml)?;

    return Ok(SubwayStatus{
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
    });
}
