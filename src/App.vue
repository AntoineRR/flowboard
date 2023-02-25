<script setup>
import TreeView from "./components/TreeView.vue";
import { invoke } from "@tauri-apps/api/tauri";

async function saveBoard() {
  await invoke("save_board");
}
</script>

<template>
  <div class="container">
    <div class="sidebar">
      <TreeView class="tree" @save-board="saveBoard" />
    </div>
    <div class="main">
      <h1>~ Welcome to your flowboard ~</h1>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: row;
  height: 100%;
}

.sidebar {
  width: 300px;
  padding: 20px;
  border-right: 1px solid #b5b5b5;
  top: 0;
  left: 0;
  bottom: 0;
  overflow-y: auto;
}

.tree {
  text-align: left;
}

.main {
  padding: 20px;
  flex: 1;
}
</style>
