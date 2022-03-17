<script setup lang="ts">
import RecipeDetails from "../components/recipe/RecipeDetails.vue";
defineProps({
  id: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <main>
    <RecipeDetails :id="id" class="q-ma-md" />
  </main>
</template>
