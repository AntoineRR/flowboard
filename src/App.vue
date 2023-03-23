<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import TreeView from "./components/TreeView.vue";
import ContentView from "./components/content/ContentView.vue";

let contentId = ref()
let treeData = ref();

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

async function deleteElement(id: number, parentId: number) {
  await invoke("delete_node", { id, parentId, recursive: false });
  updateTree();
  saveBoard();
}

function changeContentId(id: number) {
  contentId.value = id;
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
          <button class="icon-button" type="button" @click="addElement('Directory', 'new directory', 0)">
            <fa-icon icon="fa-solid fa-folder-plus"></fa-icon>
          </button>
          <button class="icon-button" type="button" @click="addElement('Note', 'new note', 0)">
            <fa-icon icon="fa-solid fa-note-sticky"></fa-icon>
          </button>
          <button class="icon-button" type="button" @click="addElement('Project', 'new project', 0)">
            <fa-icon icon="fa-solid fa-lightbulb"></fa-icon>
          </button>
        </div>
      </div>
      <TreeView class="tree" :tree-data="treeData" @add-element="addElement" @save-board="saveBoard"
        @delete-element="deleteElement" @load-content="changeContentId" />
    </div>
    <div class="main">
      <ContentView :id="contentId" @save-board="saveBoard" />
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
  border-right: 1px solid #b5b5b5;
  top: 0;
  left: 0;
  bottom: 0;
  overflow-y: auto;

  .title {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    margin-bottom: 20px;

    div button {
      padding: 0px;
    }
  }
}

.tree {
  text-align: left;
}

.main {
  padding: 20px;
  flex: 1;
}
</style>
