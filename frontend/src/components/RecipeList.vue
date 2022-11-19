<template>
  <div>
    <recipe-filter
      :recipes="recipes"
      @updated-filter="filteredRecipes = $event"
    />
    <div class="q-pa-md q-gutter-md row wrap">
      <div v-for="recipe in filteredRecipes" :key="recipe.id" class="col-2">
        <recipe-card
          :recipe="recipe"
          @deleted-recipe="remove_recipe(recipe.id)"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import axios from "axios";
import { onMounted, ref, type Ref } from "vue";
import RecipeCard from "./recipe/RecipeCard.vue";
import RecipeFilter from "./recipe/RecipeFilter.vue";
const recipes: Ref<Array<Recipe>> = ref([]);
const filteredRecipes: Ref<Array<Recipe>> = ref([]);
const isLoadingRecipes = ref(false);
const loadRecipesErrorMessage = ref("");

onMounted(() => {
  loadRecipes();
});

function loadRecipes() {
  isLoadingRecipes.value = true;
  loadRecipesErrorMessage.value = "";
  axios
    .get("/api/recipes")
    .then((response) => {
      recipes.value = response.data;
      recipes.value.sort(function (a, b) {
        const nameA = a.title.toLowerCase();
        const nameB = b.title.toLowerCase();
        if (nameA < nameB) {
          return -1;
        } else if (nameA > nameB) {
          return 1;
        } else {
          return 0;
        }
      });
      filteredRecipes.value = recipes.value;
    })
    .catch((error) => {
      recipes.value = [];
      loadRecipesErrorMessage.value = error;
    })
    .finally(() => {
      isLoadingRecipes.value = false;
    });
}

function remove_recipe(recipe_id: string) {
  recipes.value = recipes.value.filter((recipe) => recipe.id !== recipe_id);
  filteredRecipes.value = filteredRecipes.value.filter(
    (recipe) => recipe.id !== recipe_id
  );
}
</script>
<style scoped lang="scss"></style>
