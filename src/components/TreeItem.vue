<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps({ model: null })

let showAddContextMenu = ref(false);
let showOtherContextMenu = ref(false);

function toggleAddContextMenu() {
  showAddContextMenu.value = !showAddContextMenu.value;
  showOtherContextMenu.value = false;
}

function addIncluded(): Node[] {
  return Array.from(document.querySelectorAll('.add-include-' + String(props.model.id)));
}

function toggleOtherContextMenu() {
  showOtherContextMenu.value = !showOtherContextMenu.value;
  showAddContextMenu.value = false;
}

function otherIncluded(): Node[] {
  return Array.from(document.querySelectorAll('.other-include-' + String(props.model.id)));
}

</script>

<template>
  <li>
    <div v-if="model">
      <div class="content">
        <button class="invisible-button" type="button" v-if="model.node_type !== 'Directory'"
          @click="$emit('load-content', model.id)">
          {{ model.name }}
        </button>
        <p v-else>
          {{ model.name }}
        </p>
        <div class="button-container">
          <div class="context-menu-reference" v-if="model.node_type === 'Directory'">
            <button class="icon-button" :class="'add-include-' + model.id" type="button" @click="toggleAddContextMenu">
              <fa-icon icon="fa-solid fa-plus"></fa-icon>
            </button>
            <div v-if="showAddContextMenu" class="context-menu"
              v-click-outside="{ handler: toggleAddContextMenu, include: addIncluded }">
              <button class="icon-button" type="button"
                @click="$emit('add-element', 'Directory', 'new directory', model.id)">
                <fa-icon icon="fa-solid fa-folder-plus"></fa-icon>
              </button>
              <button class="icon-button" type="button" @click="$emit('add-element', 'Note', 'new note', model.id)">
                <fa-icon icon="fa-solid fa-note-sticky"></fa-icon>
              </button>
              <button class="icon-button" type="button" @click="$emit('add-element', 'Project', 'new project', model.id)">
                <fa-icon icon="fa-solid fa-lightbulb"></fa-icon>
              </button>
            </div>
          </div>
          <div class="context-menu-reference">
            <button class="icon-button" :class="'other-include-' + model.id" type="button"
              @click="toggleOtherContextMenu">
              <fa-icon icon="fa-solid fa-ellipsis"></fa-icon>
            </button>
            <div v-if="showOtherContextMenu" class="context-menu"
              v-click-outside="{ handler: toggleOtherContextMenu, include: otherIncluded }">
              <button class="icon-button" type="button" @click="$emit('delete-element', model.id, null)">
                <fa-icon icon="fa-solid fa-trash"></fa-icon>
              </button>
            </div>
          </div>
        </div>
      </div>
      <ul v-for="child in model.children" :key="child.id">
        <TreeItem :model="child" @add-element="(type, name, id) => $emit('add-element', type, name, id)"
          @load-content="(id) => $emit('load-content', id)"
          @delete-element="(id, parentId) => !!parentId ? $emit('delete-element', id, parentId) : $emit('delete-element', id, model.id)" />
      </ul>
    </div>
  </li>
</template>

<style scoped lang="scss">
.content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: antiquewhite;

  p {
    margin: 0;
    padding: 0;
  }
}

ul {
  list-style-type: none;
  padding-top: 5px;
  padding-left: 10px;
}

.button-container {
  display: flex;
}
</style>
