<script setup>
import { watch, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const props = defineProps({ model: Object });
const emit = defineEmits(['save-board'])

let content = ref("");

onMounted(() => {
  content.value = props.model.content;
})

async function saveNote() {
  await invoke("set_note_content", { id: props.model.id, content: content.value });
  emit('save-board');
}

watch(() => props.model, async (model) => {
  content.value = model.content;
})
</script>

<template>
  <textarea v-model="content" v-on:focusout="event => saveNote()" />
</template>

<style scoped lang="scss">
textarea {
  width: 100%;
  height: 100%;
}
</style>
