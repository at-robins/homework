<script setup lang="ts">
import RecipeList from "../components/RecipeList.vue";
import RecipeCreation from "../components/recipe/RecipeCreation.vue";
</script>

<template>
  <main>
    <RecipeCreation class="q-ma-md" />
    <RecipeList />
  </main>
</template>
