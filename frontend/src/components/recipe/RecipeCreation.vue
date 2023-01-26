<template>
  <q-input
    outlined
    bottom-slots
    v-model="title"
    :label="t('recipe_creation_label')"
    counter
    maxlength="1000"
    :readonly="isUploadingRecipe"
    :error="!!uploadErrorMessage"
    :error-message="uploadErrorMessage"
    @keydown.enter="uploadRecipe"
  >
    <template v-slot:before>
      <q-icon :name="matLocalDining" color="primary" />
    </template>

    <template v-slot:append>
      <q-icon
        v-if="!!title"
        name="close"
        @click="title = ''"
        class="cursor-pointer"
      />
    </template>

    <template v-slot:hint> {{ t("recipe_creation_hint") }} </template>

    <template v-slot:after>
      <q-btn
        round
        color="primary"
        icon="add"
        :disable="isUploadingRecipe || !title"
        @click="uploadRecipe"
      />
    </template>
  </q-input>
</template>

<script setup lang="ts">
import axios from "axios";
import { ref, type Ref } from "vue";
import { useRouter } from "vue-router";
import { matLocalDining } from "@quasar/extras/material-icons";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const router = useRouter();
const title: Ref<string> = ref("");
const isUploadingRecipe = ref(false);
const uploadErrorMessage = ref("");

function uploadRecipe() {
  if (title.value) {
    isUploadingRecipe.value = true;
    uploadErrorMessage.value = "";
    const formData = JSON.stringify(title.value);
    const config = {
      headers: {
        "Content-Type": "application/json",
      },
    };
    axios
      .post("/api/recipes", formData, config)
      .then((response) => {
        return router.push({ name: "recipe", params: { id: response.data } });
      })
      .catch((error) => {
        uploadErrorMessage.value = error;
      })
      .finally(() => {
        title.value = "";
        isUploadingRecipe.value = false;
      });
  } else {
    uploadErrorMessage.value = t("recipe_creation_error_message");
  }
}
</script>
<style scoped lang="scss"></style>
