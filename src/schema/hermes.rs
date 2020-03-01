use serde_derive::{Serialize, Deserialize};
use std::fmt;
use snips_nlu_ontology::{IntentClassifierResult, Slot};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct NluQuery {
    pub input: Option<String>,
    pub id: Option<String>,
    pub siteId: Option<String>,
    pub customData: Option<String>,
    pub intentFilter: Option<Vec<String>>,
    pub intentWhitelist: Option<Vec<String>>,
    pub intentBlacklist: Option<Vec<String>>,
    pub sessionId: Option<String>
}

impl fmt::Display for NluQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NluQuery(input:{:?}, sessionId:{:?}, id:{:?}, siteId:{:?}, customData:{:?}, intentFilter:{:?}, intentWhitelist:{:?}, intentBlacklist:{:?})",
               self.input,
               self.sessionId,
               self.id,
               self.siteId,
               self.customData,
               self.intentFilter,
               self.intentWhitelist,
               self.intentBlacklist
        )
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct NluError {
    pub sessionId: Option<String>,
    pub error: String,
    pub context: Option<String>
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct NluIntentNotRecognized {
    pub input: Option<String>,
    pub id: Option<String>,
    pub siteId: Option<String>,
    pub customData: Option<String>,
    pub error: Option<String>,
    pub sessionId: Option<String>
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct NluIntentParsed {
    pub input: Option<String>,
    pub id: Option<String>,
    pub siteId: Option<String>,
    pub customData: Option<String>,
    pub sessionId: Option<String>,
    pub intent: IntentClassifierResult,
    pub slots: Vec<Slot>,
}