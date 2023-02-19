<script setup>
defineProps({ model: Object })
</script>

<template>
  <li>
    <div v-if="model">
      <div class="content">
        {{ model.name }}
        <div v-if="model.node_type === 'Directory'">
          <button type="button" @click="$emit('add-directory', 'new directory', model.id)">d+</button>
          <button type="button" @click="$emit('add-note', 'new note', model.id)">n+</button>
          <button type="button" @click="$emit('add-project', 'new project', model.id)">p+</button>
        </div>
      </div>
      <ul v-for="child in model.children" :key="child.id">
        <TreeItem :model="child" @add-directory="(name, id) => $emit('add-directory', name, id)"
          @add-note="(name, id) => $emit('add-note', name, id)"
          @add-project="(name, id) => $emit('add-project', name, id)" />
      </ul>
    </div>
  </li>
</template>

<style scoped>
.content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: antiquewhite;
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
