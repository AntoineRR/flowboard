<script setup lang="ts">
import TreeItem from './TreeItem.vue'

defineProps({ treeData: null })
</script>

<template>
  <ul>
    <div v-if="!!treeData" v-for="item in treeData.children" :key="item.id">
      <TreeItem :model="item" @add-element="(type, name, id) => $emit('add-element', type, name, id)"
        @load-content="(id: number) => $emit('load-content', id)">
      </TreeItem>
    </div>
  </ul>
</template>

<style scoped lang="scss">
ul {
  padding-inline-start: 0;

  div {
    list-style-type: none;
    padding: 0;
    margin: 0;
    margin-top: 5px;
  }
}
</style>
