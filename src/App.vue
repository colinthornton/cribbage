<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri";

function startGame() {
  invoke("start_game");
}
</script>

<template>
  <h1>Hello, cribbage</h1>
  <button @click="startGame">Start Game</button>
</template>
