<template>
  <div class="card">
    <input v-model="newName" />
    <button type="button" @click="addProject()">Add a new project</button>
  </div>

  <p>{{ projectNames }}</p>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const newName = ref("");
const projectNames = ref("");

async function addProject() {
  await invoke("add_project", { name: newName.value, parentId: 0 });
  newName.value = "";
  projectNames.value = await invoke("get_project_names", { parentId: 0 });
}
</script>