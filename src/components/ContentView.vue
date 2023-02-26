<script setup>
import { watch, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri';

const props = defineProps({ id: Number });

let content = ref();

watch(() => props.id, async (id) => {
  let node_type;
  await invoke("get_node_type", { id }).then((data) => {
    node_type = data;
  }).catch((err) => {
    console.log(err);
  });
  const get_function_name = node_type === 'Note' ? 'get_note' : node_type === 'Project' ? 'get_project' : 'get_directory';
  await invoke(get_function_name, { id }).then((data) => {
    content.value = data;
  }).catch((err) => {
    console.log(err);
  });
})
</script>

<template>
  <div>
    <h1 v-if="!!content">{{ content.name }}</h1>
    <h1 v-else>~ Welcome to your flowboard ~</h1>
    <p v-if="!!content">content for {{ id }}</p>
  </div>
</template>