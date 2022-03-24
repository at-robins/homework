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

      <q-card-section horizontal class="row">
        <q-card-section class="col-4">
          <q-carousel
            v-model="imageSlideModel"
            animated
            arrows
            navigation
            infinite
            swipeable
          >
            <q-carousel-slide
              v-for="(attachment, index) in imageAttachments"
              :key="attachment.id"
              :name="index"
              :img-src="'/api/attachment/' + attachment.id"
            />
          </q-carousel>
        </q-card-section>

        <q-separator vertical />

        <q-card-section class="col-8">
          <recipe-edit-ingredients
            :recipe="recipe"
            @updated-ingredients="updateIngredients"
          />
        </q-card-section>
      </q-card-section>

      <q-separator />

      <q-card-section horizontal class="row" style="height: auto">
        <recipe-edit-instructions :id="id" v-model="editorModel" />
      </q-card-section>

      <q-separator />

      <q-item>
        <q-item-section>
          <recipe-edit-attachments
            :recipe="recipe"
            @updated-attachments="updateAttachments"
          />
        </q-item-section>
      </q-item>
    </div>
    <q-spinner v-if="isLoadingRecipe" color="primary" />
  </q-card>
</template>

<script setup lang="ts">
import type { Attachment, Ingredient, Recipe } from "@/scripts/types";
import axios from "axios";
import { computed, onMounted, ref, type Ref } from "vue";
import RecipeEditInstructions from "./RecipeEditInstructions.vue";
import RecipeEditHeader from "./RecipeEditHeader.vue";
import RecipeEditTags from "./RecipeEditTags.vue";
import RecipeEditAttachments from "./RecipeEditAttachments.vue";
import RecipeEditIngredients from "./RecipeEditIngredients.vue";
import { isImage } from "@/scripts/utilities";

const props = defineProps({
  id: { type: String, required: true },
});

const recipe: Ref<Recipe | null> = ref(null);
const isLoadingRecipe = ref(false);
const loadingErrorMessage = ref("");
const ratingModel = ref(0);
const editorModel = ref("");
const imageSlideModel = ref(0);

onMounted(() => {
  loadRecipe();
});

const imageAttachments = computed(() => {
  if (!recipe.value) {
    return [];
  } else {
    return recipe.value.attachments.filter((attachment) =>
      isImage(attachment.name)
    );
  }
});

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

function updateAttachments(newAttachments: Array<Attachment>) {
  if (recipe.value) {
    recipe.value.attachments = newAttachments;
  }
}

function updateIngredients(newIngredients: Array<Ingredient>) {
  if (recipe.value) {
    recipe.value.ingredients = newIngredients;
  }
}
</script>
<style scoped lang="scss"></style>
