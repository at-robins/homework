<template>
  <q-input
    outlined
    bottom-slots
    v-model="title"
    label="Rezepttitel"
    counter
    maxlength="1000"
    :readonly="isUploadingRecipe"
    :error="!!uploadErrorMessage"
    :error-message="uploadErrorMessage"
    @keydown.enter="uploadRecipe"
  >
    <template v-slot:before>
      <q-icon name="local_dining" color="primary" />
    </template>

    <template v-slot:append>
      <q-icon
        v-if="!!title"
        name="close"
        @click="title = ''"
        class="cursor-pointer"
      />
    </template>

    <template v-slot:hint> Erstellen Sie ein neues Rezept. </template>

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
        "content-type": "application/json",
      },
    };
    axios
      .post("/api/recipes", formData, config)
      .then((response) => {
        return router.push({ name: "recipe", params: { id: response.data } });
      })
      .catch((error) => {
        console.log(error);
        uploadErrorMessage.value = error;
      })
      .finally(() => {
        title.value = "";
        isUploadingRecipe.value = false;
      });
  } else {
    uploadErrorMessage.value =
      "Bitte tragen Sie einen Titel für das zu erstellende Rezept ein.";
  }
}
</script>
<style scoped lang="scss"></style>
