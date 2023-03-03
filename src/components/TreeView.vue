<script setup lang="ts">
import { onMounted, ref } from 'vue'
import TreeItem from './TreeItem.vue'
import { invoke } from "@tauri-apps/api/tauri";

let treeData = ref();

const emit = defineEmits(['save-board', 'load-content'])

function updateTree() {
  invoke("get_board_tree").then((data) => {
    treeData.value = data;
  }).catch((err) => {
    console.log(err);
  });
}

async function addDirectory(name: string, parentId: number) {
  await invoke("add_directory", { name, parentId });
  updateTree();
  emit('save-board')
}

async function addNote(name: string, parentId: number) {
  await invoke("add_note", { name, parentId });
  updateTree();
  emit('save-board')
}

async function addProject(name: string, parentId: number) {
  await invoke("add_project", { name, parentId });
  updateTree();
  emit('save-board')
}

onMounted(async () => {
  updateTree();
})
</script>

<template>
  <ul>
    <TreeItem :model="treeData" @add-directory="addDirectory" @add-note="addNote" @add-project="addProject"
      @load-content="(id: number) => $emit('load-content', id)">
    </TreeItem>
  </ul>
</template>

<style scoped>
ul {
  list-style-type: none;
  padding: 0;
  margin: 0;
}
</style>
