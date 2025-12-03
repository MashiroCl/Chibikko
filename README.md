# Chibikko
Meet Chibikko, a tiny cuddly creature lives on your desktop!



## Build

### Prerequisite

#### Live2d Model
Before seeing the little creature, you need to download the live2d model

Free models are available on (live2d)[https://www.live2d.com/en/learn/sample/]

1. Click the `Download Pro version` for `Shizuku` to download the model
2. Unzip the model and put the unzipped folder into `chibikko/public/`


#### Live2d Cubism SDK
The live2d model relies on the (Live2D Cubism SDK)[https://www.live2d.com/sdk/download/web/]

Download the SDKs and put them under `chibikko/public/scripts/`

The file tree of the `chibikko/public/` should look like:
```bash
chibikko\public
├─scripts
|    └─live2dcubismcore.js
|    └─live2dcubismcore.js.map
|    └─live2dcubismcore.min.js
└─shizuku
```

#### Voice
To make Chibikko speak in a voice you like!

The speak part is supported by (GPT-Sovits)[https://github.com/RVC-Boss/GPT-SoVITS].

To deploy GPT-Sovits, follow the instructions below:
1. Install (miniconda)[https://www.anaconda.com/docs/getting-started/miniconda/install]
2. Install the dependencies
```shell
    conda create -n GPTSoVits python=3.10
    conda activate GPTSoVits
    # Select the option, an example already tested on RTX5060 is
    # .\install.ps1 -Device CU126 -Source ModelScope
    pwsh -F install.ps1 --Device <CU126|CU128|CPU> --Source <HF|HF-Mirror|ModelScope> [--DownloadUVR5]
```
3. Running the GPT-SoVits
```shell
    python api.py
```
4. Prepare the voice you like by putting an `example.wav` file under the path `C:\Users\{USER}\AppData\Roaming\\com.mashirocl.chibikko\` of that voice and also the `text` of that `example.wav`.
5. Prepare config file by creating a file named `app_config.json` like below:
```json
{
  "ollm_model": "gemma3:4b",
  "tts_api_endpoint": "http://127.0.0.1:9880",
  "tts_example_wav":"example.wav",
  "tts_example_wav_text": "text"
}
```

### Start
```bash
$ cd chibikko
$ npm install
$ npm run tauri dev
```