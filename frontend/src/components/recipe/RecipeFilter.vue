<template>
  <div class="row wrap q-gutter-md q-pa-md" style="justify-content: center">
    <input-filter
      v-model="nameModel"
      :options="allNames"
      label="Rezeptnamen filtern"
      class="col-xs-11 col-lg-3"
    />
    <input-filter
      v-model="tagModel"
      :options="allTags"
      label="Schlagwörter filtern"
      multiple
      class="col-xs-11 col-lg-3"
    />
    <input-filter
      v-model="ingredientModel"
      :options="allIngredients"
      label="Zutaten filtern"
      multiple
      class="col-xs-11 col-lg-3"
    />
  </div>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import { computed, ref, watch, type Ref } from "vue";
import InputFilter from "../general/InputFilter.vue";

const props = defineProps({
  recipes: { type: Array as () => Array<Recipe>, required: true },
});

const emit = defineEmits<{
  (event: "updatedFilter", recipes: Array<Recipe>): void;
}>();

const tagModel: Ref<Array<string> | null> = ref(null);
const ingredientModel: Ref<Array<string> | null> = ref(null);
const nameModel = ref("");

const allTags = computed(() => {
  const tags = props.recipes.flatMap((recipe) => recipe.tags);
  return [...new Set<string>(tags)].sort();
});

const allNames = computed(() => {
  const names = props.recipes.flatMap((recipe) => recipe.title);
  return [...new Set<string>(names)];
});

const allIngredients = computed(() => {
  const ingredients = props.recipes
    .flatMap((recipe) => recipe.ingredients)
    .map((ingredient) => ingredient.text);
  return [...new Set<string>(ingredients)].sort();
});

watch([nameModel, tagModel, ingredientModel], () => {
  emit("updatedFilter", matchingRecipes());
});

function matchingRecipes(): Recipe[] {
  let filteredRecipes = props.recipes;
  if (nameModel.value) {
    filteredRecipes = filteredRecipes.filter((recipe) =>
      recipe.title.includes(nameModel.value)
    );
  }
  const filteredTags = tagModel.value;
  if (filteredTags && filteredTags.length > 0) {
    filteredRecipes = filteredRecipes.filter((recipe) =>
      filteredTags.every((tag) => recipe.tags.includes(tag))
    );
  }
  const filteredIngredients = ingredientModel.value;
  if (filteredIngredients && filteredIngredients.length > 0) {
    filteredRecipes = filteredRecipes.filter((recipe) =>
      filteredIngredients.every((ingredient) =>
        recipe.ingredients.map((i) => i.text).includes(ingredient)
      )
    );
  }
  return filteredRecipes;
}
</script>
<style scoped lang="scss"></style>
