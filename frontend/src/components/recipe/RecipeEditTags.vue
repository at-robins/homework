<template>
  <div class="row wrap">
    <div v-for="tag in tags" :key="tag">
      <q-chip
        class="col-auto"
        removable
        color="primary"
        text-color="white"
        :label="tag"
        :title="tag"
        @remove="removeTag(tag)"
      />
    </div>
    <div>
      <q-chip
        class="col-auto"
        removable
        icon-remove="add"
        :color="!updateTagsErrorMessage ? 'primary' : 'negative'"
        text-color="white"
        @remove="addTag"
      >
        <q-input
          v-model="addTagModel"
          borderless
          input-style="color: white;"
          :placeholder="
            !updateTagsErrorMessage
              ? 'Stichwort zuordnen'
              : updateTagsErrorMessage
          "
          @keydown.enter="addTag"
        />
        <q-tooltip>
          <div v-if="!updateTagsErrorMessage">
            Hier können Sie dem Rezept eine weiteres Stichwort zuordnen.
          </div>
          <div v-else>
            {{ updateTagsErrorMessage }}
          </div>
        </q-tooltip>
      </q-chip>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import axios from "axios";
import { ref } from "vue";

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const isUpdatingTags = ref(false);
const updateTagsErrorMessage = ref("");
const tags = ref(props.recipe.tags);
const addTagModel = ref("");

function addTag() {
  if (addTagModel.value && !tags.value.includes(addTagModel.value.trim())) {
    isUpdatingTags.value = true;
    updateTagsErrorMessage.value = "";
    const formData = JSON.stringify(addTagModel.value.trim());
    const config = {
      headers: {
        "Content-Type": "application/json",
      },
    };
    axios
      .post("/api/recipe/" + props.recipe.id + "/tags", formData, config)
      .then(() => {
        tags.value.push(addTagModel.value.trim());
      })
      .catch((error) => {
        updateTagsErrorMessage.value = error;
      })
      .finally(() => {
        isUpdatingTags.value = false;
        addTagModel.value = "";
      });
  } else {
    updateTagsErrorMessage.value =
      "Bitte benutzen Sie ein noch nicht vergebenes, nicht leeres Stichwort.";
    addTagModel.value = "";
  }
}

function removeTag(tag_name: string) {
  axios
    .delete("/api/recipe/" + props.recipe.id + "/tag/" + tag_name)
    .then(() => {
      tags.value = tags.value.filter((tag) => tag !== tag_name);
    })
    .catch((error) => {
      updateTagsErrorMessage.value = error;
      addTagModel.value = "";
    });
}
</script>
<style scoped lang="scss"></style>
