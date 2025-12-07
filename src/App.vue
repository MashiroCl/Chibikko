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

interface Message{
  content: string;
  isUser: boolean;
}

const canvasRef = ref<HTMLCanvasElement | null>(null);
const prompt = ref('')
const messages = ref<string[]>([])
const conversation = ref<Message[]>([])
const need_voice = ref(false)

onMounted(async () => {
  if(!canvasRef.value){
    console.error("No Canvas");
    return;
  }

  const app = new PIXI.Application({
    view: canvasRef.value,
    width: 200,
    height: 350,
    backgroundAlpha: 0,
    antialias: true,
  });

  const model = await Live2DModel.from('shizuku/runtime/shizuku.model3.json');

  app.stage.addChild(model)

  model.scale.set(0.2);
  model.anchor.set(0.3, 0.3);
  model.x = 50;
  model.y = 70;


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

      conversation.value.push({content:content, isUser:true});
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
        conversation.value.push({content:response.text, isUser:false});
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
  pointer-events: none;
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

canvas {
  display: block;
  background-color: red;
  margin: 0 auto;
  pointer-events: auto;
}

.chat-container {
  width: 100%;
  max-width: 600px;
  height: 200px;
  border: 1px solid #ccc;
  display: flex;
  flex-direction: column;
  margin: 0 auto;
  border-radius: 8px;
  background-color: #f5f5f5;
  opacity: 0.9;
  pointer-events: none;
}

.conversation-area {
  flex: 1;
  padding: 15px;
  overflow-y: scroll; 
  scrollbar-width: none;
  display: flex;
  flex-direction: column;
  gap: 10px;
  pointer-events: auto;
}

.bubble {
  padding: 10px 14px;
  border-radius: 12px;
  word-wrap: break-word;
  font-size: 14px;
  line-height: 1.4;
  position: relative;
  pointer-events: auto;
}

.user {
  align-self: flex-end;
}

.user .bubble {
  background-color: aqua;
  color: #000;
  border-bottom-right-radius: 2px;
}

.prompt-input-area {
  position: relative; 
  z-index: 10;
  display: flex;
  justify-content: center;
  margin-top: 10px;
  pointer-events: none;
}

.llm-options input,
.llm-options label {
  pointer-events: auto;
  cursor: pointer;
}

.prompt-input-area input,
.prompt-input-area button {
  pointer-events: auto;
  cursor: pointer;
}

</style>

<template>
  <div class="chat-container">
    <div class="conversation-area" ref="converstaionContainer">
      <div v-if="conversation.length===0">Ciallo~(∠·ω< )⌒★</div>
      <div v-for="(msg, index) in conversation" :key="index" class="is-user" :class="{'user':msg.isUser, 'llm':!msg.isUser}">
        <div class="bubble">
          <!-- TODO: put an cute llm icon here -->
          <div v-if="!msg.isUser">llm: </div>
          {{ msg.content }}
        </div>
      </div>
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