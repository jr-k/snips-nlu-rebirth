Snips NLU rebirth
=================

The aim of this repository is to bring back to life the `snips-nlu` bin that wasn't fully open source.

Dependencies
=

- `apt install mosquitto mosquitto-clients clang`

Build instructions
=

- We need a rust compiler so let's install rustup:

  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

- Download repository

  `git clone https://github.com/jr-k/snips-nlu-rebirth && cd snips-nlu-rebirth`
  
- Setup your configuration and edit
  
  `cp snips-nlu.toml.dist snips-nlu.toml && nano snips-nlu.toml`
  
- Finally build/run project

  `cargo run`
  
  
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
