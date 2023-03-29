<script setup lang="ts">
import { onUpdated } from 'vue';
import TreeItem from './TreeItem.vue'

const props = defineProps({ treeData: null })
defineEmits<{
  (e: 'add-element', type: string, name: string, parent_id: number): void,
  (e: 'delete-element', id: number, parent_id: number | null): void,
  (e: 'load-content', id: number): void
}>()

function getOffset(el: HTMLElement) {
  const rect = el.getBoundingClientRect();
  return {
    left: rect.left + window.scrollX,
    top: rect.top + window.scrollY
  };
}

function setMouseListener() {
  const elements = document.querySelectorAll('.tree-item-content') as NodeListOf<HTMLElement>;
  console.log(elements);
  console.log(props.treeData);
  let lastUpdate = new Date().getTime();
  const updatePeriod = 30; // ms
  function updateXY(e: MouseEvent) {
    if (new Date().getTime() - lastUpdate < updatePeriod) {
      return;
    }
    elements.forEach((elt) => {
      elt.style.setProperty('--x', e.clientX - getOffset(elt).left + 'px');
      elt.style.setProperty('--y', e.clientY - getOffset(elt).top + 'px');
    });
    lastUpdate = new Date().getTime();
  }
  // Set an event listener to update the x and y variables of each ul when the mouse moves
  document.removeEventListener("mousemove", updateXY);
  document.addEventListener("mousemove", updateXY);
}

onUpdated(() => {
  setMouseListener();
});

</script>

<template>
  <div v-if="!!treeData" v-for="item in treeData.children" :key="item.id">
    <ul>
      <TreeItem :model="item" @add-element="(type, name, id) => $emit('add-element', type, name, id)"
        @delete-element="(id, parentId) => !!parentId ? $emit('delete-element', id, parentId) : $emit('delete-element', id, 0)"
        @load-content="(id: number) => $emit('load-content', id)">
      </TreeItem>
    </ul>
  </div>
</template>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: row;
  margin-top: 5px;
}

ul {
  flex: 1;
  padding: 0px;
  list-style-type: none;
  margin: 0;
}
</style>
