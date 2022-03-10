<script setup lang="ts">
import Toolbar from "@/components/Toolbar.vue";
import RecipeDetails from "../components/recipe/RecipeDetails.vue";
defineProps({
  id: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <header>
    <Toolbar />
  </header>
  <main>
    <RecipeDetails :id="id" class="q-ma-md" />
  </main>
</template>
