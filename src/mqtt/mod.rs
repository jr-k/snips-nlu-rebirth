extern crate serde_json;

use rumqtt::{MqttClient, MqttOptions, QoS, SecurityOptions};
use snips_nlu_lib::SnipsNluEngine;
use snips_nlu_ontology::{IntentParserResult, IntentClassifierResult, Slot};
use std::str;
use crate::schema::config;
use crate::schema::hermes;
use crate::schema::snips_nlu;

pub fn start(config: &config::Config, engine: &SnipsNluEngine) {

    let mqtt_options = MqttOptions::new("snips-nlu-rebirth", config.mqtt.host.clone(), config.mqtt.port.clone())
        .set_keep_alive(30)
        .set_security_opts(SecurityOptions::UsernamePassword(config.mqtt.username.clone(), config.mqtt.password.clone()));

    let _conn = match MqttClient::start(mqtt_options) {
        Ok(c) => {
            let (mut mqtt_client, notifications) = c;

            mqtt_client.subscribe("hermes/#", QoS::AtLeastOnce).unwrap();

            for notification in notifications {
                match notification {
                    rumqtt::client::Notification::Publish(packet) => {
                        let query = str::from_utf8(&packet.payload).unwrap();

                        if &packet.topic_name == "hermes/nlu/query" {
                            hermes_nlu_query(&mut mqtt_client, &engine, &query);
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
                _ => {}
            }

        }
    };
}

pub fn hermes_error_nlu(mqtt_client: &mut MqttClient, error_message: &str) {
    let nlu_error: hermes::NluError = hermes::NluError {
        message: String::from(error_message)
    };

    let result_json = serde_json::to_string(&nlu_error).unwrap();
    mqtt_client.publish("hermes/error/nlu", QoS::AtLeastOnce, false, result_json).unwrap();
}

pub fn hermes_nlu_intent_not_recognized(mqtt_client: &mut MqttClient, parsed_query: hermes::NluQuery) {
    let nlu_intent_not_recognized: hermes::NluIntentNotRecognized = hermes::NluIntentNotRecognized {
        input: parsed_query.input,
        id: parsed_query.id,
        sessionId: parsed_query.sessionId
    };
    let result_json = serde_json::to_string(&nlu_intent_not_recognized).unwrap();
    mqtt_client.publish("hermes/nlu/intentNotRecognized", QoS::AtLeastOnce, false, result_json).unwrap();
}

pub fn hermes_nlu_intent_parsed(mqtt_client: &mut MqttClient, parsed_query: hermes::NluQuery, parsed_result: IntentParserResult) {
    let nlu_intent_parsed: hermes::NluIntentParsed = hermes::NluIntentParsed {
        input: parsed_query.input,
        id: parsed_query.id,
        sessionId: parsed_query.sessionId,
        intent: None,
        slots: Vec::new()
    };
    let result_json = serde_json::to_string(&nlu_intent_parsed).unwrap();
    mqtt_client.publish("hermes/nlu/intentParsed", QoS::AtLeastOnce, false, result_json).unwrap();
}

pub fn hermes_nlu_query(mqtt_client: &mut MqttClient, engine: &SnipsNluEngine, query: &str) {
    let parsed_query: hermes::NluQuery = match serde_json::from_str(&query) {
        Ok(pq) => { pq }
        Err(e) => {
            hermes_error_nlu(mqtt_client, &e.to_string());
            return;
        }
    };

    if parsed_query.input.is_none() {
        hermes_error_nlu(mqtt_client, "No input field");
        return;
    }

    if parsed_query.sessionId.is_none() {
        hermes_error_nlu(mqtt_client, "No sessionId field");
        return;
    }

    let intents_alternatives = 0;
    let slots_alternatives = 0;
    let input = parsed_query.input.as_ref().unwrap();
    let parsed_result = engine.parse_with_alternatives(&*input, None, None, intents_alternatives, slots_alternatives).unwrap();

    if parsed_result.intent.intent_name.is_none() {
        //IntentParserResult { input: "l", intent: IntentClassifierResult { intent_name: None, confidence_score: 0.54227686 }, slots: [], alternatives: [] }
        hermes_nlu_intent_not_recognized(mqtt_client, parsed_query);
        return;
    }

    //IntentParserResult { input: "light in the garage", intent: IntentClassifierResult { intent_name: Some("turnLightOn"), confidence_score: 0.3685922 }, slots: [Slot { raw_value: "garage", value: Custom(StringValue { value: "garage" }), alternatives: [], range: 13..19, entity: "room", slot_name: "room", confidence_score: None }], alternatives: [] }
    hermes_nlu_intent_parsed(mqtt_client, parsed_query, parsed_result);
    return;
}