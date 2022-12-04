<template>
  <div>
    <recipe-filter :recipes="recipes" @updated-filter="onUpdateFilter" />
    <q-infinite-scroll @load="onScrollLoad" :offset="500">
      <div class="q-pa-md q-gutter-md row wrap" style="justify-content: center">
        <div
          v-for="recipe in loadedRecipes"
          :key="recipe.id"
          class="col-xs-11 col-sm-3 col-lg-2"
        >
          <recipe-card
            :recipe="recipe"
            @deleted-recipe="remove_recipe(recipe.id)"
          />
        </div>
      </div>
    </q-infinite-scroll>
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
const loadedRecipes: Ref<Array<Recipe>> = ref([]);
const isLoadingRecipes = ref(false);
const loadRecipesErrorMessage = ref("");
const INITIALLY_LOADED_RECIPES = 10;
const RECIPE_LOAD_BUFFER = 5;

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
      initialiseLoadedRecipes();
    })
    .catch((error) => {
      recipes.value = [];
      loadRecipesErrorMessage.value = error;
    })
    .finally(() => {
      isLoadingRecipes.value = false;
    });
}

function initialiseLoadedRecipes() {
  if (filteredRecipes.value.length >= INITIALLY_LOADED_RECIPES) {
    loadedRecipes.value = filteredRecipes.value.slice(
      0,
      INITIALLY_LOADED_RECIPES
    );
  } else {
    loadedRecipes.value = filteredRecipes.value;
  }
}

function onScrollLoad(_index: number, done: () => void) {
  if (
    filteredRecipes.value.length > 0 &&
    filteredRecipes.value.length > loadedRecipes.value.length
  ) {
    if (
      filteredRecipes.value.length >
      loadedRecipes.value.length + RECIPE_LOAD_BUFFER
    ) {
      loadedRecipes.value = filteredRecipes.value.slice(
        0,
        loadedRecipes.value.length + RECIPE_LOAD_BUFFER
      );
    } else {
      loadedRecipes.value = filteredRecipes.value;
    }
  }
  done();
}

function onUpdateFilter(updatedFilteredRecipes: Recipe[]) {
  filteredRecipes.value = updatedFilteredRecipes;
  initialiseLoadedRecipes();
}

function remove_recipe(recipe_id: string) {
  recipes.value = recipes.value.filter((recipe) => recipe.id !== recipe_id);
  filteredRecipes.value = filteredRecipes.value.filter(
    (recipe) => recipe.id !== recipe_id
  );
}
</script>
<style scoped lang="scss"></style>
