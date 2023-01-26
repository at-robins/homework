<template>
  <q-input
    outlined
    bottom-slots
    v-model="title"
    :label="t('payment_creation_label')"
    counter
    maxlength="1000"
    :readonly="isUploadingPayment"
    :error="!!uploadErrorMessage"
    :error-message="uploadErrorMessage"
    @keydown.enter="uploadPayment"
  >
    <template v-slot:before>
      <q-icon :name="matReceiptLong" color="primary" />
    </template>

    <template v-slot:append>
      <q-icon
        v-if="!!title"
        :name="matClose"
        @click="title = ''"
        class="cursor-pointer"
      />
    </template>

    <template v-slot:hint> {{ t("payment_creation_hint") }} </template>

    <template v-slot:after>
      <q-btn
        round
        color="primary"
        :icon="matAdd"
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
import { useI18n } from "vue-i18n";
import {
  matAdd,
  matClose,
  matReceiptLong,
} from "@quasar/extras/material-icons";

const { t } = useI18n();

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
        "Content-Type": "application/json",
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
    uploadErrorMessage.value = t("payment_creation_error_message");
  }
}
</script>
<style scoped lang="scss"></style>
