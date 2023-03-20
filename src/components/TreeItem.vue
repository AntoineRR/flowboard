<script setup lang="ts">
defineProps({ model: null })
</script>

<template>
  <li>
    <div v-if="model">
      <div class="content">
        <button type="button" v-if="model.node_type !== 'Directory'" @click="$emit('load-content', model.id)">
          {{ model.name }}
        </button>
        <p v-else>
          {{ model.name }}
        </p>
        <div v-if="model.node_type === 'Directory'">
          <button type="button" @click="$emit('add-element', 'Directory', 'new directory', model.id)">d+</button>
          <button type="button" @click="$emit('add-element', 'Note', 'new note', model.id)">n+</button>
          <button type="button" @click="$emit('add-element', 'Project', 'new project', model.id)">p+</button>
          <button type="button" @click="$emit('delete-element', model.id, null)">D</button>
        </div>
        <div v-else>
          <button type="button" @click="$emit('delete-element', model.id, null)">D</button>
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

button {
  background-color: transparent;
  border: none;
  cursor: pointer;
  padding: 0;
}
</style>
