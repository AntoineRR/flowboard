<script setup>
import { onMounted, ref } from 'vue'
import TreeItem from './TreeItem.vue'
import { invoke } from "@tauri-apps/api/tauri";

let treeData = ref();
const newName = ref("");


function updateTree() {
  invoke("get_board_tree").then((data) => {
    treeData.value = data;
  }).catch((err) => {
    console.log(err);
  });
}

async function addProject() {
  await invoke("add_project", { name: newName.value, parentId: 0 });
  newName.value = "";
  updateTree();
}

onMounted(async () => {
  updateTree();
})
</script>

<template>
  <div class="card">
    <input v-model="newName" />
    <button type="button" @click="addProject()">Add a new project</button>
  </div>
  <ul>
    <TreeItem class="item" :model="treeData"></TreeItem>
  </ul>
</template>

<style>
.item {
  cursor: pointer;
  line-height: 1.5;
}

.bold {
  font-weight: bold;
}
</style>