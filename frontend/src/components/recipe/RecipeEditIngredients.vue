<template>
  <div class="q-pa-md q-gutter-md">
    <q-list bordered class="rounded-borders">
      <q-item-label header>Zutaten</q-item-label>
      <div
        v-for="(ingredient, index) in ingredientsWithEmptyEntity"
        :key="ingredient.id"
      >
        <recipe-edit-ingredient
          :ingredient="ingredient"
          @added-ingredient="addIngredient"
          @updated-ingredient="updateIngredient"
          @removed-ingredient="removeIngredient"
        />
        <q-separator spaced v-if="index + 1 < ingredients.length" />
      </div>
    </q-list>
  </div>
</template>

<script setup lang="ts">
import type { Ingredient } from "@/scripts/types";
import type { Recipe } from "@/scripts/types";
import { computed, ref, watch, type Ref } from "vue";
import RecipeEditIngredient from "./RecipeEditIngredient.vue";

const emit = defineEmits<{
  (event: "updatedIngredients", ingredients: Array<Ingredient>): void;
}>();

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const ingredients: Ref<Array<Ingredient>> = ref(props.recipe.ingredients);

const ingredientsWithEmptyEntity = computed(() => {
  let withEmptyEntity = [...ingredients.value];
  const emptyEntity: Ingredient = {
    id: "",
    amount: "",
    unit: "",
    text: "",
    recipeReference: null,
    recipeId: props.recipe.id,
    creationTime: new Date(),
  };
  withEmptyEntity.push(emptyEntity);
  return withEmptyEntity;
});

watch(props.recipe, (newValue) => {
  ingredients.value = newValue.ingredients;
});

function addIngredient(ingredient: Ingredient) {
  ingredients.value.push(ingredient);
  emit("updatedIngredients", ingredients.value);
}

function updateIngredient(updatedIngredient: Ingredient) {
  ingredients.value = ingredients.value.map((ingredient) => {
    if (ingredient.id === updatedIngredient.id) {
      return updatedIngredient;
    } else {
      return ingredient;
    }
  });
  emit("updatedIngredients", ingredients.value);
}

function removeIngredient(ingredientId: string) {
  ingredients.value = ingredients.value.filter(
    (ingredient) => ingredient.id !== ingredientId
  );
  emit("updatedIngredients", ingredients.value);
}
</script>
<style scoped lang="scss"></style>
