extern crate serde_json;

use rumqtt::{MqttClient, MqttOptions, QoS, SecurityOptions, ReconnectOptions};
use snips_nlu_lib::SnipsNluEngine;
use snips_nlu_ontology::{IntentParserResult};
use std::str;
use std::process;
use crate::schema::config;
use crate::schema::hermes;

pub struct EngineContext {
    pub engine: SnipsNluEngine,
    pub engine_dir: String
}

impl EngineContext {

    pub fn set_engine(&mut self, new_engine: SnipsNluEngine) {
        self.engine = new_engine;
    }

    pub fn start(&mut self, config: &config::Config) {

        let mqtt_options = MqttOptions::new("snips-nlu-rebirth", config.mqtt.host.clone(), config.mqtt.port.clone())
            .set_keep_alive(30)
            .set_clean_session(false)
            .set_reconnect_opts(ReconnectOptions::Always(5))
            .set_security_opts(SecurityOptions::UsernamePassword(config.mqtt.username.clone(), config.mqtt.password.clone()));

        let _conn = match MqttClient::start(mqtt_options) {
            Ok(c) => {
                let (mut mqtt_client, notifications) = c;
                println!("Connect");

                mqtt_client.subscribe("hermes/nlu/#", QoS::AtLeastOnce).unwrap();
                mqtt_client.publish("hermes/nlu/ready", QoS::AtLeastOnce, false, env!("CARGO_PKG_VERSION")).unwrap();

                for notification in notifications {
                    match notification {
                        rumqtt::client::Notification::Publish(packet) => {
                            let query = str::from_utf8(&packet.payload).unwrap();

                            println!("Message on topic {:?}", packet.topic_name);

                            if &packet.topic_name == "hermes/nlu/query" {
                                self.hermes_nlu_query(&mut mqtt_client, &query);
                            } else if &packet.topic_name == "hermes/nlu/exit" {
                                let ret_code: i32 = match str::FromStr::from_str(query) {
                                    Ok(rc) => { rc },
                                    Err(_) => {
                                        println!("\nBad exit return code, using 0x01");
                                        1
                                    }
                                };

                                process::exit(ret_code);
                            } else if &packet.topic_name == "hermes/nlu/reload/engine" {
                                println!("\nReload engine");
                                self.set_engine(SnipsNluEngine::from_path(self.engine_dir.as_str()).expect("Can't find engine"));
                            }
                        },
                        _ => {}
                    }
                }
            }
            Err(e) => {
                match e {
                    rumqtt::error::ConnectError::MqttConnectionRefused(_mqtt_error) => {

                    },
                    rumqtt::error::ConnectError::Io(_io_error) => {

                    },
                    _ => {

                    }
                }
            }
        };
    }

    pub fn hermes_error_nlu(&self, mqtt_client: &mut MqttClient, parsed_query: Option<hermes::NluQuery>, error_message: &str) {
        let nlu_error: hermes::NluError = hermes::NluError {
            sessionId: match parsed_query {
                Some(pq) => { pq.sessionId },
                None => { None }
            },
            error: String::from(error_message),
            context: None
        };

        let result_json = serde_json::to_string(&nlu_error).unwrap();
        mqtt_client.publish("hermes/error/nlu", QoS::AtLeastOnce, false, result_json).unwrap();
    }

    pub fn hermes_nlu_intent_not_recognized(&self, mqtt_client: &mut MqttClient, parsed_query: hermes::NluQuery, error_message: String) {
        let nlu_intent_not_recognized: hermes::NluIntentNotRecognized = hermes::NluIntentNotRecognized {
            input: parsed_query.input,
            id: parsed_query.id,
            siteId: parsed_query.siteId,
            customData: parsed_query.customData,
            error: Some(error_message),
            sessionId: parsed_query.sessionId
        };
        let result_json = serde_json::to_string(&nlu_intent_not_recognized).unwrap();
        mqtt_client.publish("hermes/nlu/intentNotRecognized", QoS::AtLeastOnce, false, result_json).unwrap();
    }

    pub fn hermes_nlu_intent_parsed(&self, mqtt_client: &mut MqttClient, parsed_query: hermes::NluQuery, parsed_result: IntentParserResult) {
        let nlu_intent_parsed: hermes::NluIntentParsed = hermes::NluIntentParsed {
            input: parsed_query.input,
            id: parsed_query.id,
            siteId: parsed_query.siteId,
            customData: parsed_query.customData,
            sessionId: parsed_query.sessionId,
            intent: parsed_result.intent,
            slots: parsed_result.slots
        };
        let result_json = serde_json::to_string(&nlu_intent_parsed).unwrap();
        mqtt_client.publish("hermes/nlu/intentParsed", QoS::AtLeastOnce, false, result_json).unwrap();
    }

    pub fn hermes_nlu_query(&self, mqtt_client: &mut MqttClient, query: &str) {
        let parsed_query: hermes::NluQuery = match serde_json::from_str(&query) {
            Ok(pq) => { pq }
            Err(e) => {
                self.hermes_error_nlu(mqtt_client, None, &e.to_string());
                return;
            }
        };

        let mut intent_whitelist : Vec<&str> = Vec::new();
        let mut intent_blacklist : Vec<&str> = Vec::new();

        if parsed_query.input.is_none() {
            self.hermes_error_nlu(mqtt_client, Some(parsed_query), "No input field");
            return;
        }

        if parsed_query.sessionId.is_none() {
            self.hermes_error_nlu(mqtt_client, Some(parsed_query), "No sessionId field");
            return;
        }

        if !parsed_query.intentFilter.is_none() {
            intent_whitelist = parsed_query.intentFilter.as_ref().unwrap().iter().map(AsRef::as_ref).collect();
        } else if !parsed_query.intentWhitelist.is_none() {
            intent_whitelist = parsed_query.intentWhitelist.as_ref().unwrap().iter().map(AsRef::as_ref).collect();
        }

        if !parsed_query.intentBlacklist.is_none() {
            intent_blacklist = parsed_query.intentBlacklist.as_ref().unwrap().iter().map(AsRef::as_ref).collect();
        }

        let intents_alternatives = 0;
        let slots_alternatives = 0;
        let input = parsed_query.input.as_ref().unwrap();

        let parsed_result = self.engine.parse_with_alternatives(
            &*input,
            match intent_whitelist.is_empty() {
                true => { None },
                false => { Some(intent_whitelist) }
            },
            match intent_blacklist.is_empty() {
                true => { None },
                false => { Some(intent_blacklist) }
            },
            intents_alternatives,
            slots_alternatives
        );

        return match parsed_result {
            Ok(parsed_result_unwraped) => {
                 if parsed_result_unwraped.intent.intent_name.is_none() {
                    //IntentParserResult { input: "l", intent: IntentClassifierResult { intent_name: None, confidence_score: 0.54227686 }, slots: [], alternatives: [] }
                    self.hermes_nlu_intent_not_recognized(mqtt_client, parsed_query, String::new());
                    return;
                }

                //IntentParserResult { input: "light in the garage", intent: IntentClassifierResult { intent_name: Some("turnLightOn"), confidence_score: 0.3685922 }, slots: [Slot { raw_value: "garage", value: Custom(StringValue { value: "garage" }), alternatives: [], range: 13..19, entity: "room", slot_name: "room", confidence_score: None }], alternatives: [] }
                self.hermes_nlu_intent_parsed(mqtt_client, parsed_query, parsed_result_unwraped);
                return;
            },
            Err(e) => {
                self.hermes_nlu_intent_not_recognized(mqtt_client, parsed_query, e.to_string());
            }
        }
    }
}