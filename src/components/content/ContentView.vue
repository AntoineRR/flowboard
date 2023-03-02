<script setup>
import { watch, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri';
import NoteView from './NoteView.vue';

const props = defineProps({ id: Number });

let node_type;
let content = ref();

watch(() => props.id, async (id) => {
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
  <div class="wrapper">
    <h1 v-if="!!content">{{ content.name }}</h1>
    <h1 v-else>~ Welcome to your flowboard ~</h1>
    <div v-if="!!content">
      <NoteView v-if="node_type === 'Note'" :model="content" @save-board="() => $emit('save-board')" />
    </div>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  height: 100%;
  display: flex;
  flex-direction: column;

  h1 {
    padding: 20px;
  }

  div {
    flex: 1;
  }
}
</style>