<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import TreeView from "./components/TreeView.vue";
import ContentView from "./components/content/ContentView.vue";

let contentId = ref()
let treeData = ref();

defineEmits<{
  (e: 'add-element', type: string, name: string, parent_id: number): void,
  (e: 'delete-element', id: number, parent_id: number | null): void,
  (e: 'load-content', id: number): void,
  (e: 'save-board'): void
}>()

async function saveBoard() {
  await invoke("save_board");
}

async function addElement(type: string, name: string, parentId: number) {
  if (type === "Directory") {
    await invoke("add_directory", { name, parentId });
  } else if (type === "Note") {
    await invoke("add_note", { name, parentId });
  } else if (type === "Project") {
    await invoke("add_project", { name, parentId });
  }
  updateTree();
  saveBoard();
}

async function deleteElement(id: number, parentId: number | null) {
  await invoke("delete_node", { id, parentId, recursive: false });
  updateTree();
  saveBoard();
}

function changeContentId(id: number) {
  contentId.value = id;
}

async function setContentName(name: string) {
  await invoke("set_node_name", { id: contentId.value, name });
  updateTree();
  saveBoard();
}

function updateTree() {
  invoke("get_board_tree").then((data) => {
    treeData.value = data;
  }).catch((err) => {
    console.log(err);
  });
}

onMounted(async () => {
  updateTree();
})

</script>

<template>
  <div class="container">
    <div class="sidebar">
      <div class="title">
        <h1>Flowboard</h1>
        <div>
          <div class="icon-container">
            <button class="icon-button" type="button" @click="addElement('Directory', 'new directory', 0)">
              <fa-icon icon="fa-solid fa-folder"></fa-icon>
            </button>
          </div>
          <div class="icon-container">
            <button class="icon-button" type="button" @click="addElement('Note', 'new note', 0)">
              <fa-icon icon="fa-solid fa-note-sticky"></fa-icon>
            </button>
          </div>
          <div class="icon-container">
            <button class="icon-button" type="button" @click="addElement('Project', 'new project', 0)">
              <fa-icon icon="fa-solid fa-lightbulb"></fa-icon>
            </button>
          </div>
        </div>
      </div>
      <div class="tree">
        <TreeView :tree-data="treeData" @add-element="addElement" @delete-element="deleteElement"
          @load-content="changeContentId" />
      </div>
    </div>
    <div class="main">
      <ContentView :id="contentId" @save-board="saveBoard" @set-content-name="setContentName" />
    </div>
  </div>
</template>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: row;
  height: 100%;
}

.sidebar {
  width: 300px;
  padding: 20px;
  border-right: 1px solid var(--overlay-color);
  top: 0;
  left: 0;
  bottom: 0;
  overflow-y: auto;

  background-color: var(--background-color-two);

  .title {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    margin-bottom: 20px;

    div {
      display: flex;
      flex-direction: row;
    }
  }
}

.tree {
  text-align: left;
}

.main {
  flex: 1;
}
</style>
