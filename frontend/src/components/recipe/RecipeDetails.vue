<template>
  <q-card flat bordered>
    <div v-if="recipe">
      <q-item>
        <q-item-section>
          <recipe-edit-header :recipe="recipe" />
        </q-item-section>
      </q-item>

      <q-separator />

      <q-item>
        <q-item-section>
          <recipe-edit-tags :recipe="recipe" />
        </q-item-section>
      </q-item>

      <q-separator />

      <q-card-section horizontal>
        <q-card-section> abc </q-card-section>

        <q-separator vertical />

        <q-card-section class="col-4">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        </q-card-section>
      </q-card-section>

      <q-separator />

      <q-card-section horizontal>
        <recipe-edit-instructions :id="id" v-model="editorModel" />
      </q-card-section>
    </div>
    <q-spinner v-if="isLoadingRecipe" color="primary" />
  </q-card>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import axios from "axios";
import { ref, type Ref } from "vue";
import RecipeEditInstructions from "./RecipeEditInstructions.vue";
import RecipeEditHeader from "./RecipeEditHeader.vue";
import RecipeEditTags from "./RecipeEditTags.vue";

const props = defineProps({
  id: { type: String, required: true },
});

const recipe: Ref<Recipe | null> = ref(null);
const isLoadingRecipe = ref(false);
const loadingErrorMessage = ref("");
const ratingModel = ref(0);
const editorModel = ref("");

loadRecipe();

function loadRecipe() {
  isLoadingRecipe.value = true;
  loadingErrorMessage.value = "";
  axios
    .get("/api/recipe/" + props.id)
    .then((response) => {
      recipe.value = response.data;
      if (recipe.value) {
        ratingModel.value = recipe.value.rating;
        editorModel.value = recipe.value.instructions;
      }
    })
    .catch((error) => {
      recipe.value = null;
      loadingErrorMessage.value = error;
    })
    .finally(() => {
      isLoadingRecipe.value = false;
    });
}

function updateRating(newValue: number) {
  if (recipe.value && recipe.value.rating !== newValue) {
    const formData = JSON.stringify(newValue);
    const config = {
      headers: {
        "content-type": "application/json",
      },
    };
    axios
      .post("/api/recipe/" + props.id + "/rating", formData, config)
      .then(() => {
        if (recipe.value) {
          recipe.value.rating = newValue;
        }
      })
      .catch((error) => {
        loadingErrorMessage.value = error;
      });
  }
}
</script>
<style scoped lang="scss"></style>
