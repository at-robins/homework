<template>
  <div class="q-pa-xs q-gutter-xs">
    <q-list class="rounded-borders">
      <q-item-label header>Zutaten</q-item-label>
      <div
        v-for="(ingredient, index) in ingredientsWithEmptyEntity"
        :key="ingredient.id"
      >
        <recipe-edit-ingredient
          :ingredient="ingredient"
          :available-recipe-references="availableRecipeReferences"
          :ordering="index"
          :is-udating-ordering="isUpdatingOrdering"
          :update-ordering-error-message="updateOrderingErrorMessage"
          @added-ingredient="addIngredient"
          @updated-ingredient="updateIngredient"
          @removed-ingredient="removeIngredient"
          @move-ingredient-up="moveIngredient($event, -1)"
          @move-ingredient-down="moveIngredient($event, 1)"
        />
        <q-separator
          spaced
          v-if="index + 1 < ingredientsWithEmptyEntity.length"
        />
      </div>
    </q-list>
  </div>
</template>

<script setup lang="ts">
import type { Ingredient, RecipeReferences } from "@/scripts/types";
import type { Recipe } from "@/scripts/types";
import axios, { type AxiosResponse } from "axios";
import { computed, onMounted, ref, type Ref } from "vue";
import RecipeEditIngredient from "./RecipeEditIngredient.vue";

const emit = defineEmits<{
  (event: "updatedIngredients", ingredients: Array<Ingredient>): void;
}>();

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const availableRecipeReferences: Ref<RecipeReferences> = ref({
  error: "",
  references: [],
});

const ingredientsSorted = computed(() => {
  return [...props.recipe.ingredients].sort((a, b) => a.ordering - b.ordering);
});

const ingredientsWithEmptyEntity = computed(() => {
  const withEmptyEntity = [...ingredientsSorted.value];
  const emptyEntity: Ingredient = {
    id: "",
    amount: "",
    unit: "",
    text: "",
    recipeReference: null,
    recipeId: props.recipe.id,
    creationTime: new Date().toISOString(),
    ordering: props.recipe.ingredients.length,
  };
  withEmptyEntity.push(emptyEntity);
  return withEmptyEntity;
});
const isUpdatingOrdering = ref(false);
const updateOrderingErrorMessage = ref("");

onMounted(() => {
  loadRecipeReferences();
});

function addIngredient(ingredient: Ingredient) {
  const extendedIngredients = [...ingredientsSorted.value];
  extendedIngredients.push(ingredient);
  emit("updatedIngredients", extendedIngredients);
}

function updateIngredient(updatedIngredient: Ingredient) {
  const updatedIngredients = ingredientsSorted.value.map((ingredient) => {
    if (ingredient.id === updatedIngredient.id) {
      return updatedIngredient;
    } else {
      return ingredient;
    }
  });
  emit("updatedIngredients", updatedIngredients);
}

function removeIngredient(ingredientId: string) {
  const cleanedIngredients = ingredientsSorted.value.filter(
    (ingredient) => ingredient.id !== ingredientId
  );
  updateIngredientOrder(cleanedIngredients.map((value) => value.id));
  emit("updatedIngredients", cleanedIngredients);
}

function moveIngredient(ingredientId: string, shiftIndexBy: number) {
  const originalIndex = ingredientsSorted.value.findIndex(
    (value) => value.id === ingredientId
  );
  const newIndex = originalIndex + shiftIndexBy;
  if (newIndex >= 0 && newIndex < props.recipe.ingredients.length) {
    const shiftedIngredients = [...ingredientsSorted.value];
    const shiftedElement = shiftedIngredients.splice(originalIndex, 1)[0];
    shiftedIngredients.splice(newIndex, 0, shiftedElement);
    shiftedIngredients.forEach((value, index) => {
      value.ordering = index;
    });
    isUpdatingOrdering.value = true;
    updateOrderingErrorMessage.value = "";
    updateIngredientOrder(shiftedIngredients.map((value) => value.id))
      .then(() => {
        emit("updatedIngredients", shiftedIngredients);
      })
      .catch((error) => {
        updateOrderingErrorMessage.value = String(error);
      })
      .finally(() => {
        isUpdatingOrdering.value = false;
      });
  }
}

function updateIngredientOrder(
  orderedIngredientIds: string[]
): Promise<AxiosResponse<unknown, unknown>> {
  const formData = JSON.stringify(orderedIngredientIds);
  const config = {
    headers: {
      "Content-Type": "application/json",
    },
  };
  return axios.post(
    "/api/recipe/" + props.recipe.id + "/ingredients/ordering",
    formData,
    config
  );
}

function loadRecipeReferences() {
  axios
    .get("/api/recipes")
    .then((response) => {
      availableRecipeReferences.value = {
        error: "",
        references: response.data.map((recipe: Recipe) => {
          return { label: recipe.title, value: recipe.id };
        }),
      };
    })
    .catch((error) => {
      availableRecipeReferences.value = {
        error: error,
        references: [],
      };
    });
}
</script>
<style scoped lang="scss"></style>
