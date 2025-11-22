<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as PIXI from 'pixi.js';
// import { Live2DModel } from 'pixi-live2d-display';
import { Live2DModel } from 'pixi-live2d-display/cubism4';

import { getCurrentWindow } from '@tauri-apps/api/window';
import { EndOfLineState } from 'typescript';

// expose PIXI to window so that this plugin is able to
// reference window.PIXI.Ticker to automatically update Live2D models
declare global{
  interface Window{
    PIXI: typeof PIXI
  }
}
window.PIXI = PIXI;

const canvasRef = ref<HTMLCanvasElement | null>(null);


onMounted(async () => {
  if(!canvasRef.value){
    console.error("No Canvas");
    return;
  }
  const app = new PIXI.Application({
    view: canvasRef.value,
    width: 800,
    height: 600,
    backgroundAlpha: 0,
    antialias: true,
  });

  const model = await Live2DModel.from('shizuku/runtime/shizuku.model3.json');

  app.stage.addChild(model)

  model.scale.set(0.25);
  model.anchor.set(0.3, 0.3);
  model.x = 400 - model.width/2;
  model.y = 300 - model.height/2;


  model.interactive = true;
  model.buttonMode = true;

  handleClick(model);

  console.log("Model loaded")

})

function handleClick(model: Live2DModel){
  model.on('pointerdown', (e)=>{
    // Drag the avatar
    if (e.data.button ===0){
      console.log("Pointer Down");
      getCurrentWindow().startDragging();
    }
 
    // Play random motion
    const motionGroups = model.internalModel.motionManager.motionGroups; 
    const motionNames = Object.keys(motionGroups);
    const motion = motionNames[Math.floor(Math.random()*motionNames.length)] 
    model.motion(motion);
  })
 
}

</script>

<style>
html, body, #app{
  margin: 0;
  padding: 0;
  width: 100vw;
  height: 100vh;
  background-color: transparent;
  overflow: hidden;
}

canvas {
  display: block;
}

</style>

<template>
  <canvas ref="canvasRef"></canvas>
</template>