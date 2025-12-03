<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as PIXI from 'pixi.js';
// import { Live2DModel } from 'pixi-live2d-display';
import { Live2DModel } from 'pixi-live2d-display/cubism4';

import { getCurrentWindow } from '@tauri-apps/api/window';


import { invoke } from '@tauri-apps/api/core';

// expose PIXI to window so that this plugin is able to
// reference window.PIXI.Ticker to automatically update Live2D models
declare global{
  interface Window{
    PIXI: typeof PIXI
  }
}
window.PIXI = PIXI;

const canvasRef = ref<HTMLCanvasElement | null>(null);
const prompt = ref('')
const messages = ref<string[]>([])
const need_voice = ref(false)

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

  // setInterval(loadStats, 3000);
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

  async function loadStats() {
   const statsJson: string = await invoke("get_status");
   const stats = JSON.parse(statsJson)
   console.log(stats); 
  }

  interface LlmResponse {
    text: string;
    audio: number[] | null;
  }

  const handleSend = async () => {
      const content = prompt.value;

      if(!content.trim()) return;

      prompt.value = '';

      try{
        console.log("Querying LLM...");
        const response: LlmResponse = await invoke("query_llm", {prompt: content, needVoice: need_voice.value});
        if (response.audio) {
          const uint8Array = new Uint8Array(response.audio);
          const blob = new Blob([uint8Array], {type: "audio/wav"});
          const url = URL.createObjectURL(blob);
          const audio = new Audio(url);
          audio.play();
        }
        messages.value.push(response.text);
      } catch (e) {
        console.error("LLM failed", e);
        messages.value.push("Error: query failed");
      }
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

.chat-window {
  height: 22%;
  padding: 22px 24px;
  border-radius: 8px;
  background-color: azure;
  box-shadow: 3cap;
  margin-bottom: 1.2em;
  overflow-y: scroll;
  scrollbar-width: none;
}

.llm-options {
  display: flex;
  align-items: center;
  color: aqua;
  gap: 8px;
}

.llm-options::voice {
  cursor: pointer;
}

.chat-window::-webkit-scrollbar {
  display: none;
}

canvas {
  display: block;
}

</style>

<template>
  <!-- TODO: Each chat is a bubble and automatically fold a too big chat -->
  <div class="chat-window">
    <div v-if="messages.length===0">Conversation happens here...</div>
    <div v-for="(msg, index) in messages" :key="index" class="message">
      {{ msg }}
    </div>
  </div>
  <div class="llm-options">
    <div class="voice">
      <input type="checkbox" id="needVoice" v-model="need_voice"/>
      <label for="needVoice">Voice</label>
    </div>
  </div>
  <div class="prompt-input-area">
    <input v-model="prompt" type="text" placeholder="Ciallo~(∠·ω< )⌒★" @keydown.enter="handleSend"></input>
    <button @click="handleSend">Send</button>
  </div>
  <canvas ref="canvasRef"></canvas>
</template>