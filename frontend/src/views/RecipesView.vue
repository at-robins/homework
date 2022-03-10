<script setup lang="ts">
import Toolbar from "@/components/Toolbar.vue";
import RecipeList from "../components/RecipeList.vue";
import RecipeCreation from "../components/recipe/RecipeCreation.vue";
</script>

<template>
  <header>
    <Toolbar />
  </header>
  <main>
    <RecipeCreation class="q-ma-md" />
    <RecipeList />
  </main>
</template>
