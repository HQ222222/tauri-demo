<script setup>
import { ref, defineEmits } from "vue";
import { invoke } from "@tauri-apps/api/tauri";


const emit = defineEmits(["alertSome"])

const greetMsg = ref("");
const name = ref("");

async function greet() {
  emit("nameBg", name.value)
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="告诉我你的名字..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
