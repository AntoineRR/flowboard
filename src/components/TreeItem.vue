<script setup>
defineProps({ model: Object })
</script>

<template>
  <li>
    <div v-if="model">
      {{ model.name }}
      <ul v-for="child in model.children" :key="child.id">
        <TreeItem :model="child" />
      </ul>
    </div>
  </li>
</template>