<template>
  <q-item-label class="text-h3">{{ recipe.title }}</q-item-label>
  <div class="row">
    <q-rating
      class="col-auto"
      v-model="ratingModel"
      max="10"
      size="3em"
      :color="!updateRatingErrorMessage ? 'yellow' : 'negative'"
      icon="star_border"
      icon-selected="star"
      :disable="!!updateRatingErrorMessage"
    />
    <q-icon
      v-if="!!updateRatingErrorMessage"
      class="col-auto q-ma-sm self-center"
      name="warning"
      color="negative"
      size="1.5em"
    >
      <q-tooltip> {{ updateRatingErrorMessage }}</q-tooltip>
    </q-icon>
  </div>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import axios from "axios";
import { ref, watch } from "vue";

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const isUpdatingRating = ref(false);
const updateRatingErrorMessage = ref("");
const ratingModel = ref(props.recipe.rating);

let updatingInstructionsTimer: number | null = null;

watch(() => ratingModel.value, updateRating);

function updateRating(newRating: number) {
  if (updatingInstructionsTimer !== null) {
    clearTimeout(updatingInstructionsTimer);
  }
  isUpdatingRating.value = true;
  updateRatingErrorMessage.value = "";
  const formData = JSON.stringify(newRating);
  const config = {
    headers: {
      "content-type": "application/json",
    },
  };
  axios
    .post("/api/recipe/" + props.recipe.id + "/rating", formData, config)
    .catch((error) => {
      updateRatingErrorMessage.value = error;
    })
    .finally(() => {
      updatingInstructionsTimer = setTimeout(
        () => (isUpdatingRating.value = false),
        1000
      );
    });
}
</script>
<style scoped lang="scss"></style>
