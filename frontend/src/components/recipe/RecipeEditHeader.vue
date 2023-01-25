<template>
  <div class="row no-wrap">
    <q-item-label v-if="!editTitleMode" class="text-h3 shrink q-ma-sm">{{
      titleModel
    }}</q-item-label>
    <q-input
      v-else
      ref="titleInputRef"
      v-model="titleModel"
      @blur="updateTitle"
      @keydown.enter="updateTitle"
      :readonly="isUpdatingTitle"
      class="text-h3 col-grow q-ma-sm"
    />
    <q-btn
      v-if="!editTitleMode"
      round
      icon="edit"
      class="self-start"
      :color="!updateTitleErrorMessage ? 'primary' : 'negative'"
      outline
      size="sm"
      @click="editTitleField"
    >
      <q-tooltip>
        <div v-if="!updateTitleErrorMessage">
          {{ t("recipe_header_edit_tooltip") }}
        </div>
        <div v-else>
          {{ updateTitleErrorMessage }}
        </div>
      </q-tooltip>
    </q-btn>
    <q-btn v-else round class="self-start" color="primary" outline size="sm">
      <q-spinner v-if="isUpdatingTitle" color="primary" />
      <q-icon v-else name="done" color="primary" />
    </q-btn>
  </div>
  <div class="row no-wrap">
    <div v-if="!editReferenceMode" class="text-h5 shrink q-ma-sm">
      {{ t("recipe_header_reference_title") + " " + referenceModel }}
    </div>
    <q-input
      v-else
      ref="referenceInputRef"
      v-model="referenceModel"
      @blur="updateReference"
      @keydown.enter="updateReference"
      :readonly="isUpdatingReference"
      class="text-h5 col-grow q-ma-sm"
    />
    <q-btn
      v-if="!editReferenceMode"
      round
      icon="edit"
      class="self-start"
      :color="!updateReferenceErrorMessage ? 'primary' : 'negative'"
      outline
      size="xs"
      @click="editReferenceField"
    >
      <q-tooltip>
        <div v-if="!updateReferenceErrorMessage">Rezeptreferenz bearbeiten</div>
        <div v-else>
          {{ updateReferenceErrorMessage }}
        </div>
      </q-tooltip>
    </q-btn>
    <q-btn v-else round class="self-start" color="primary" outline size="xs">
      <q-spinner v-if="isUpdatingReference" color="primary" />
      <q-icon v-else name="done" color="primary" />
    </q-btn>
  </div>
  <div class="row">
    <q-rating
      class="col-auto q-ma-sm"
      v-model="ratingModel"
      max="10"
      size="3em"
      :color="!updateRatingErrorMessage ? 'primary' : 'negative'"
      icon="star_border"
      icon-selected="star"
      :disable="!!updateRatingErrorMessage || isUpdatingRating"
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
import { nextTick, ref, watch, type Ref } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const isUpdatingRating = ref(false);
const updateRatingErrorMessage = ref("");
const ratingModel = ref(props.recipe.rating);

const isUpdatingTitle = ref(false);
const updateTitleErrorMessage = ref("");
const editTitleMode = ref(false);
const titleModel = ref(props.recipe.title);
const titleInputRef: Ref<HTMLInputElement | null> = ref(null);

const isUpdatingReference = ref(false);
const updateReferenceErrorMessage = ref("");
const editReferenceMode = ref(false);
const referenceModel = ref(props.recipe.reference);
const referenceInputRef: Ref<HTMLInputElement | null> = ref(null);

watch(() => ratingModel.value, updateRating);

function updateRating(newRating: number) {
  isUpdatingRating.value = true;
  updateRatingErrorMessage.value = "";
  const formData = JSON.stringify(newRating);
  const config = {
    headers: {
      "Content-Type": "application/json",
    },
  };
  axios
    .post("/api/recipe/" + props.recipe.id + "/rating", formData, config)
    .catch((error) => {
      updateRatingErrorMessage.value = error;
    })
    .finally(() => {
      isUpdatingRating.value = false;
    });
}

function updateTitle() {
  isUpdatingTitle.value = true;
  updateTitleErrorMessage.value = "";
  const formData = JSON.stringify(titleModel.value);
  const config = {
    headers: {
      "Content-Type": "application/json",
    },
  };
  axios
    .post("/api/recipe/" + props.recipe.id + "/string/title", formData, config)
    .catch((error) => {
      updateTitleErrorMessage.value = error;
    })
    .finally(() => {
      isUpdatingTitle.value = false;
      editTitleMode.value = false;
    });
}

function updateReference() {
  isUpdatingReference.value = true;
  updateReferenceErrorMessage.value = "";
  const formData = JSON.stringify(referenceModel.value);
  const config = {
    headers: {
      "Content-Type": "application/json",
    },
  };
  axios
    .post(
      "/api/recipe/" + props.recipe.id + "/string/reference",
      formData,
      config
    )
    .catch((error) => {
      updateReferenceErrorMessage.value = error;
    })
    .finally(() => {
      isUpdatingReference.value = false;
      editReferenceMode.value = false;
    });
}

function editTitleField() {
  editTitleMode.value = true;
  nextTick(() => {
    if (titleInputRef.value) {
      titleInputRef.value.focus();
    }
  });
}

function editReferenceField() {
  editReferenceMode.value = true;
  nextTick(() => {
    if (referenceInputRef.value) {
      referenceInputRef.value.focus();
    }
  });
}
</script>
<style scoped lang="scss"></style>
