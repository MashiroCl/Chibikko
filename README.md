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


### Start
```bash
$ cd chibikko
$ npm install
$ npm run tauri dev
```