<template>
  <q-input
    outlined
    bottom-slots
    v-model="title"
    label="Zahlungsbezug"
    counter
    maxlength="1000"
    :readonly="isUploadingPayment"
    :error="!!uploadErrorMessage"
    :error-message="uploadErrorMessage"
    @keydown.enter="uploadPayment"
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

    <template v-slot:hint> Erstellen Sie ein neue Zahlung. </template>

    <template v-slot:after>
      <q-btn
        round
        color="primary"
        icon="add"
        :disable="isUploadingPayment || !title"
        @click="uploadPayment"
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
const isUploadingPayment = ref(false);
const uploadErrorMessage = ref("");

function uploadPayment() {
  if (title.value) {
    isUploadingPayment.value = true;
    uploadErrorMessage.value = "";
    const formData = JSON.stringify(title.value);
    const config = {
      headers: {
        "content-type": "application/json",
      },
    };
    axios
      .post("/api/payments", formData, config)
      .then((response) => {
        return router.push({ name: "payment", params: { id: response.data } });
      })
      .catch((error) => {
        uploadErrorMessage.value = error;
      })
      .finally(() => {
        title.value = "";
        isUploadingPayment.value = false;
      });
  } else {
    uploadErrorMessage.value =
      "Bitte tragen Sie einen Bezugstitel für die erstellende Zahlung ein.";
  }
}
</script>
<style scoped lang="scss"></style>
