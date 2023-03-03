<script setup lang="ts">
defineProps({
  model: null
})
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
          <button type="button" @click="$emit('add-directory', 'new directory', model.id)">d+</button>
          <button type="button" @click="$emit('add-note', 'new note', model.id)">n+</button>
          <button type="button" @click="$emit('add-project', 'new project', model.id)">p+</button>
        </div>
      </div>
      <ul v-for="child in model.children" :key="child.id">
        <TreeItem :model="child" @add-directory="(name, id) => $emit('add-directory', name, id)"
          @add-note="(name, id) => $emit('add-note', name, id)"
          @add-project="(name, id) => $emit('add-project', name, id)" @load-content="(id) => $emit('load-content', id)" />
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
