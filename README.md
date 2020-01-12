Snips NLU rebirth
=================

The aim of this repository is to bring back to life the `snips-nlu` bin that wasn't fully open source.

Dependencies
=

- rustup
- mqtt server/client (Mosquitto)
- clang

Just get these dependencies from apt repositories: `apt install mosquitto mosquitto-clients clang`

Build instructions
=

- We need a rust compiler so let's install rustup:

  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- Download repository

  `git clone https://github.com/jr-k/snips-nlu-rebirth && cd snips-nlu-rebirth`
  
- Setup your configuration and edit
  
  `cp snips-nlu.toml.dist snips-nlu.toml && nano snips-nlu.toml`
  
Training
=

- The snips-nlu training part is provided by this repository: https://github.com/snipsco/snips-nlu.

- After you cloned it you'll be able to train a model, let's choose the provided lights dataset.
`snips-nlu /path/to/snips-nlu/repository/sample_datasets/lights_dataset.json /path/to/output_trained_engine`.

- Don't forget to add `path/to/output_trained_engine` to the configuration file `snips-nlu.toml` (from this project) in the `engine_dir` variable of the `[global]` section and you're ready to parse any query trained from the `lights_dataset` model.

Run
=

- Run `mosquitto_sub -t '#' -v` to see whats going on 

- Finally build/run project

  `cargo run`

- You can trigger the NLU by sending a MQTT message

  `mosquitto_pub -t 'hermes/nlu/query' -m '{"input":"light in the garage", "sessionId":"42"}'`

> the output on topic `hermes/nlu/intentParsed` would be:

```json
{
    "input": "light in the garage",
    "id": null,
    "sessionId": "42",
    "intent": {
        "intentName": "turnLightOn",
        "confidenceScore": 0.3685922
    },
    "slots": [{
        "rawValue": "garage",
        "value": {
            "kind": "Custom",
            "value": "garage"
        },
        "alternatives": [],
        "range": {
            "start": 13,
            "end": 19
        },
        "entity": "room",
        "slotName": "room"
    }]
}
```
  
API reference
=

This project follows the hermes protocol described here: https://docs.snips.ai/reference/hermes#natural-language-understanding-nlu

API for NLU :

- `hermes/nlu/query` : ✅ 
- `hermes/nlu/partialQuery` : ❌
- `hermes/nlu/intentParsed` : ✅ 
- `hermes/nlu/slotParsed` : ❌
- `hermes/nlu/intentNotRecognized` : ✅ 
- `hermes/error/nlu` : ✅ 

Todo
=
- TLS for MQTT server
