<script setup lang="ts">
import { watch, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import VueMarkdown from 'vue-markdown-render';

const props = defineProps({ model: null, edit: Boolean });
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
  <div class="content" v-if="edit">
    <textarea v-model="content" v-on:focusout="event => saveNote()" />
  </div>
  <div class="content" v-else>
    <vue-markdown :source="content" />
  </div>
</template>

<style scoped lang="scss">
.content {
  width: 100%;
  height: 100%;
}

textarea {
  width: 100%;
  height: 100%;
}
</style>
