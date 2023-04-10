<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps({ model: null })
defineEmits<{
  (e: 'add-element', type: string, name: string, parent_id: number): void
  (e: 'delete-element', id: number, parent_id: number | null): void
  (e: 'load-content', id: number): void
}>()

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

function toggleFold() {
  document.querySelectorAll('.foldable-' + String(props.model.id)).forEach((el) => {
    el.classList.toggle('folded');
  });
}

</script>

<template>
  <li v-if="model">
    <div class="container">
      <div v-if="!!model.children?.length" class="fold-toggle" @click="toggleFold">
        <div></div>
      </div>
      <div class="item-container">
        <div class="tree-item-content">
          <div class="tree-item-background">
            <button class="invisible-button" type="button" v-if="model.node_type !== 'Directory'"
              @click="$emit('load-content', model.id)">
              {{ model.name }}
            </button>
            <p v-else>
              {{ model.name }}
            </p>
            <div class="button-container">
              <div class="context-menu-reference icon-container" v-if="model.node_type === 'Directory'">
                <button class="icon-button" :class="'add-include-' + model.id" type="button"
                  @click="toggleAddContextMenu">
                  <fa-icon icon="fa-solid fa-plus"></fa-icon>
                </button>
                <div v-if="showAddContextMenu" class="context-menu"
                  v-click-outside="{ handler: toggleAddContextMenu, include: addIncluded }">
                  <div class="icon-container">
                    <button class="icon-button" type="button"
                      @click="$emit('add-element', 'Directory', 'new directory', model.id)">
                      <fa-icon icon="fa-solid fa-folder"></fa-icon>
                    </button>
                  </div>
                  <div class="icon-container">
                    <button class="icon-button" type="button" @click="$emit('add-element', 'Note', 'new note', model.id)">
                      <fa-icon icon="fa-solid fa-note-sticky"></fa-icon>
                    </button>
                  </div>
                  <div class="icon-container">
                    <button class="icon-button" type="button"
                      @click="$emit('add-element', 'Project', 'new project', model.id)">
                      <fa-icon icon="fa-solid fa-lightbulb"></fa-icon>
                    </button>
                  </div>
                </div>
              </div>
              <div class="context-menu-reference icon-container">
                <button class="icon-button" :class="'other-include-' + model.id" type="button"
                  @click="toggleOtherContextMenu">
                  <fa-icon icon="fa-solid fa-ellipsis"></fa-icon>
                </button>
                <div v-if="showOtherContextMenu" class="context-menu"
                  v-click-outside="{ handler: toggleOtherContextMenu, include: otherIncluded }">
                  <div class="icon-container">
                    <button class="icon-button" type="button" @click="$emit('delete-element', model.id, null)">
                      <fa-icon icon="fa-solid fa-trash"></fa-icon>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div :class="'foldable-' + model.id">
          <div v-for="child in model.children" :key="child.id">
            <ul>
              <TreeItem :model="child"
                @add-element="(type: string, name: string, id: number) => $emit('add-element', type, name, id)"
                @delete-element="(id: number, parentId: number | null) => !!parentId ? $emit('delete-element', id, parentId) : $emit('delete-element', id, model.id)"
                @load-content="(id: number) => $emit('load-content', id)" />
            </ul>
          </div>
        </div>
      </div>
    </div>
  </li>
</template>

<style scoped lang="scss">
.item-container {
  width: 100%;
}

.tree-item-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: radial-gradient(300px circle at var(--x) var(--y), var(--highlight-color-one) 0, var(--highlight-color-two) 50%, transparent 100%);
  --x: 0;
  --y: 0;

  padding: 2px;
  border-radius: 10px;

  p {
    margin: 0;
    padding: 0;
  }

  .tree-item-background {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-left: 10px;
    padding-right: 10px;
    padding-top: 5px;
    padding-bottom: 5px;
    border-radius: 8px;
    background-color: var(--surface-color-one);

    button {
      width: 100%;
      text-align: left;
    }
  }
}

.container {
  display: flex;
  flex-direction: row;
  margin-top: 5px;
  margin-left: 10px;
  padding: 0px;

  .fold-toggle {
    display: flex;
    justify-content: center;
    margin-right: 5px;
    width: 8px;
    min-width: 8px;
    background-color: transparent;

    div {
      width: 4px;
      border-radius: 2px;
      background-color: var(--surface-color-one);
    }
  }

  .fold-toggle:hover div {
    width: 8px;
    border-radius: 4px;
    background-color: var(--surface-color-two);
    cursor: pointer;
  }
}

ul {
  flex-direction: column;
  list-style-type: none;
  padding: 0px;
}

.button-container {
  display: flex;
}

.folded {
  height: 0px;
  overflow: hidden;
}
</style>
