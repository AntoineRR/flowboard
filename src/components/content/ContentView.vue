<script setup lang="ts">
import { watch, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri';
import NoteView from './NoteView.vue';

const props = defineProps({ id: Number });
defineEmits<{
  (e: 'save-board'): void
  (e: 'set-content-name', id: number, name: string): void
}>();

let nodeType: string;
let content = ref();
let editContent = ref(false);
let editTitle = ref(false);

watch(() => props.id, async (id) => {
  await invoke("get_node_type", { id }).then((data) => {
    nodeType = data as string;
  }).catch((err) => {
    console.log(err);
  });
  const get_function_name = nodeType === 'Note' ? 'get_note' : nodeType === 'Project' ? 'get_project' : 'get_directory';
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
        <div class="title">
          <h1 v-if="!editTitle" v-on:click="_ => editTitle = true">{{ content.name }}</h1>
          <input v-else v-model="content.name" v-focus v-on:focusout="_ => {
            editTitle = false; $emit('set-content-name', content.id, content.name);
          }" />
        </div>
        <div class="icons">
          <div class="icon-container" v-if="editContent">
            <button class="icon-button" v-on:click="editContent = false">
              <fa-icon icon="fa-solid fa-floppy-disk"></fa-icon>
            </button>
          </div>
          <div class="icon-container" v-else>
            <button class="icon-button" v-on:click="editContent = true">
              <fa-icon icon="fa-solid fa-pen"></fa-icon>
            </button>
          </div>
        </div>
      </div>
      <h1 v-else>~ Welcome to your flowboard ~</h1>
    </div>
    <div class="content" v-if="!!content">
      <NoteView v-if="nodeType === 'Note'" :model="content" :edit="editContent" @save-board="() => $emit('save-board')" />
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
    padding: 20px;

    div {
      display: flex;
      flex-direction: row;
      justify-content: space-between;
    }
  }

  .title {
    flex: 1;

    h1 {
      width: 100%;
      line-height: normal;
    }

    input {
      width: 100%;
      padding: 0px;
      margin: 0px;
      border: none;
      font-size: 2em;
      font-weight: bold;
      background-color: transparent;
      text-align: center;
    }
  }

  .content {
    height: 100%;
    padding: 20px;
    overflow: auto;
  }

  .icons {
    display: flex;
    justify-content: center;
    align-items: center;
  }
}
</style>