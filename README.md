Snips NLU rebirth
=================

The aim of this repository is to bring back to life the `snips-nlu` bin that wasn't fully open source.

Training
=

The snips-nlu training part is provided by this repository: https://github.com/snipsco/snips-nlu. 


> You can build from source or download and install pre-built binaries I built. These binaries are for armhf architectures like raspberrypi and are working for python3.7. Don't follow steps 1 and 2 if you want to build snips-nlu yourself.


1. Download all wheels by running following commands (MD5 and SHA256 checksums: [Prebuilt wheels README.md](wheels/README.md))

```bash
sudo apt install libatlas3-base libgfortran5

cd /home/pi
wget --content-disposition https://github.com/jr-k/snips-nlu-rebirth/blob/master/wheels/scipy-1.3.3-cp37-cp37m-linux_armv7l.whl?raw=true
wget --content-disposition https://github.com/jr-k/snips-nlu-rebirth/blob/master/wheels/scikit_learn-0.22.1-cp37-cp37m-linux_armv7l.whl?raw=true
wget --content-disposition https://github.com/jr-k/snips-nlu-rebirth/blob/master/wheels/snips_nlu_utils-0.9.1-cp37-cp37m-linux_armv7l.whl?raw=true
wget --content-disposition https://github.com/jr-k/snips-nlu-rebirth/blob/master/wheels/snips_nlu_parsers-0.4.3-cp37-cp37m-linux_armv7l.whl?raw=true
wget --content-disposition https://github.com/jr-k/snips-nlu-rebirth/blob/master/wheels/snips_nlu-0.20.2-py3-none-any.whl?raw=true
```
  
2. Install them all in this order:

```bash
sudo pip3 install scipy-1.3.3-cp37-cp37m-linux_armv7l.whl
sudo pip3 install scikit_learn-0.22.1-cp37-cp37m-linux_armv7l.whl
sudo pip3 install snips_nlu_utils-0.9.1-cp37-cp37m-linux_armv7l.whl
sudo pip3 install snips_nlu_parsers-0.4.3-cp37-cp37m-linux_armv7l.whl
sudo pip3 install snips_nlu-0.20.2-py3-none-any.whl
```

3. Thanks to snips-nlu tools you'll be able to train a model. But first we need to prepare the targeted language. **(Warning: if you installed wheels from `pi` user without `sudo`, `snips-nlu` path will be `/home/pi/.local/bin/snips-nlu`)**

```bash 
snips-nlu download en
```

4. Then train a dataset. Let's take the sample available on the snips-nlu repository.

```bash
git clone https://github.com/snipsco/snips-nlu
cd snips-nlu/
snips-nlu train sample_datasets/lights_dataset.json path_to_output_trained_engine/
```

* *Note for later: Don't forget to add `path_to_output_trained_engine` to the configuration file `snips-nlu.toml` (from this project) in the `engine_dir` variable of the `[global]` section and you're ready to parse any query trained from the `lights_dataset` model.* *

Dependencies
=

- rustup
- mqtt server/client (Mosquitto)
- clang

Get these dependencies from apt repositories by running

```bash
sudo apt install mosquitto mosquitto-clients clang
```

Build instructions
=

- We need a rust compiler so let's install rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Download repository

```bash
git clone https://github.com/jr-k/snips-nlu-rebirth && cd snips-nlu-rebirth
```
  
- Setup your configuration and edit
  
```bash
cp snips-nlu.toml.dist snips-nlu.toml && nano snips-nlu.toml
```
  
Run
=

**! You won't be able to compile this on a raspberry pi, you need more power so you'll need to cross compile using a specific toolchain, there is more information in this page: https://github.com/jr-k/snips-nlu-rebirth/blob/master/XCOMPILE.md !**

- Run `mosquitto_sub -t '#' -v` to see whats going on 

- Finally build/run project

```bash
cargo run
```

- You can trigger the NLU by sending a MQTT message

```bash
mosquitto_pub -t 'hermes/nlu/query' -m '{"input":"light in the garage", "sessionId":"42"}'
```

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
