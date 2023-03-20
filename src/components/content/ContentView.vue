<script setup lang="ts">
import { watch, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri';
import NoteView from './NoteView.vue';

const props = defineProps({ id: Number });

let node_type: string;
let content = ref();
let edit = ref(false);

watch(() => props.id, async (id) => {
  await invoke("get_node_type", { id }).then((data) => {
    node_type = data as string;
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
    <div class="top-bar">
      <div v-if="!!content">
        <h1>{{ content.name }}</h1>
        <button class="icon-button" v-if="edit" v-on:click="edit = false">
          <fa-icon icon="fa-solid fa-floppy-disk"></fa-icon>
        </button>
        <button class="icon-button" v-else v-on:click="edit = true">
          <fa-icon icon="fa-solid fa-pen"></fa-icon>
        </button>
      </div>
      <h1 v-else>~ Welcome to your flowboard ~</h1>
    </div>
    <div v-if="!!content">
      <NoteView v-if="node_type === 'Note'" :model="content" :edit="edit" @save-board="() => $emit('save-board')" />
    </div>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  height: 100%;
  display: flex;
  flex-direction: column;

  .top-bar {
    flex: 0;

    div {
      display: flex;
      flex-direction: row;
      justify-content: space-between;
    }
  }

  h1 {
    padding: 20px;
  }

  div {
    flex: 1;
  }
}
</style>