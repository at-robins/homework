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

      <q-card-section horizontal class="row" style="justify-content: center">
        <q-carousel
          v-model="imageSlideModel"
          animated
          arrows
          navigation
          infinite
          swipeable
          class="col-xs-12 col-lg-6"
        >
          <q-carousel-slide
            v-for="(attachment, index) in imageAttachments"
            :key="attachment.id"
            :name="index"
            class="column no-wrap"
          >
            <q-img
              class="rounded-borders col-12"
              :src="getImageAttachmentUrl(attachment)"
              fit="contain"
            />
            <div class="absolute q-pa-sm">
              <q-btn
                round
                dense
                text-color="white"
                :icon="
                  isAttachmentThumbnail(attachment)
                    ? matBookmarkAdded
                    : matBookmark
                "
                @click="setThumbnail(attachment)"
                :disable="isAttachmentThumbnail(attachment)"
                :loading="isSettingThumbnail"
              >
                <q-tooltip>
                  <div v-if="!thumbnailErrorMessage">
                    <div v-if="isAttachmentThumbnail(attachment)">
                      {{ t("recipe_details_thumbnail_tooltip_current") }}
                    </div>
                    <div v-else>
                      {{ t("recipe_details_thumbnail_tooltip_new") }}
                    </div>
                  </div>
                  <div v-else>
                    {{ thumbnailErrorMessage }}
                  </div>
                </q-tooltip>
              </q-btn>
            </div>
          </q-carousel-slide>
        </q-carousel>
      </q-card-section>

      <q-card-section horizontal class="row">
        <recipe-edit-ingredients
          :recipe="recipe"
          @updated-ingredients="updateIngredients"
          class="full-width"
        />
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
import { computed, nextTick, onMounted, ref, type Ref } from "vue";
import RecipeEditInstructions from "./RecipeEditInstructions.vue";
import RecipeEditHeader from "./RecipeEditHeader.vue";
import RecipeEditTags from "./RecipeEditTags.vue";
import RecipeEditAttachments from "./RecipeEditAttachments.vue";
import RecipeEditIngredients from "./RecipeEditIngredients.vue";
import { isImage, getImageAttachmentUrl } from "@/scripts/utilities";
import { matBookmark, matBookmarkAdded } from "@quasar/extras/material-icons";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps({
  id: { type: String, required: true },
});

const recipe: Ref<Recipe | null> = ref(null);
const isLoadingRecipe = ref(false);
const isSettingThumbnail = ref(false);
const loadingErrorMessage = ref("");
const thumbnailErrorMessage = ref("");
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

/**
 * Auomatically sets a thumbnail if images are available and no thumbnail is currently set.
 */
function autoSetThumbnail() {
  if (
    recipe.value &&
    !recipe.value.thumbnail &&
    imageAttachments.value.length > 0
  ) {
    setThumbnail(imageAttachments.value[0]);
  }
}

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
        autoSetThumbnail();
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

function setThumbnail(attachment: Attachment | null) {
  isSettingThumbnail.value = true;
  thumbnailErrorMessage.value = "";
  const formData = JSON.stringify(attachment ? attachment.id : null);
  const config = {
    headers: {
      "Content-Type": "applocation/json",
    },
  };
  axios
    .post("/api/recipe/" + props.id + "/thumbnail", formData, config)
    .then(() => {
      if (recipe.value) {
        recipe.value.thumbnail = attachment;
      }
    })
    .catch((error) => {
      thumbnailErrorMessage.value = error;
    })
    .finally(() => {
      isSettingThumbnail.value = false;
    });
}

function updateAttachments(newAttachments: Array<Attachment>) {
  if (recipe.value) {
    recipe.value.attachments = newAttachments;
    imageSlideModel.value = 0;
    nextTick(autoSetThumbnail);
  }
}

function updateIngredients(newIngredients: Array<Ingredient>) {
  if (recipe.value) {
    recipe.value.ingredients = newIngredients;
  }
}

function isAttachmentThumbnail(attachment: Attachment): boolean {
  return (
    !!recipe.value &&
    !!recipe.value.thumbnail &&
    attachment.id == recipe.value.thumbnail.id
  );
}
</script>
<style scoped lang="scss"></style>
