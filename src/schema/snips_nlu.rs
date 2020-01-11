use serde_derive::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SnipsNluIntent {
    pub intentName: Option<String>,
    pub confidenceScore: Option<f32>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SnipsNluSlot {
    pub rawValue: Option<String>,
    pub value: Option<SnipsNluSlotValue>,
    pub range: Option<SnipsNluSlotRange>,
    pub entity: Option<String>,
    pub slotName: Option<String>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SnipsNluSlotValue {
    pub kind: Option<String>,
    pub value: Option<String>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct SnipsNluSlotRange {
    pub start: Option<i32>,
    pub end: Option<i32>
}
