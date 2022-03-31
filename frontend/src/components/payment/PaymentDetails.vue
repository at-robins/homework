<template>
  <q-card flat bordered>
    <div v-if="payment">
      <q-item>
        <q-item-section>
          <payment-edit-header :payment="payment" />
        </q-item-section>
      </q-item>

      <q-separator />

      <q-item>
        <q-item-section>
          <payment-edit-tags :payment="payment" />
        </q-item-section>
      </q-item>

      <q-separator />

      <q-card-section horizontal class="row" style="height: auto">
        <payment-edit-note :id="id" v-model="editorModel" />
      </q-card-section>

      <q-separator />

      <q-item>
        <q-item-section>
          <payment-edit-attachments
            :payment="payment"
            @updated-attachments="updateAttachments"
          />
        </q-item-section>
      </q-item>
    </div>
    <q-spinner v-if="isLoadingPayment" color="primary" />
  </q-card>
</template>

<script setup lang="ts">
import type { Attachment, Payment } from "@/scripts/types";
import axios from "axios";
import { onMounted, ref, type Ref } from "vue";
import PaymentEditNote from "./PaymentEditNote.vue";
import PaymentEditHeader from "./PaymentEditHeader.vue";
import PaymentEditTags from "./PaymentEditTags.vue";
import PaymentEditAttachments from "./PaymentEditAttachments.vue";

const props = defineProps({
  id: { type: String, required: true },
});

const payment: Ref<Payment | null> = ref(null);
const isLoadingPayment = ref(false);
const loadingErrorMessage = ref("");
const editorModel = ref("");

onMounted(() => {
  loadPayment();
});

function loadPayment() {
  isLoadingPayment.value = true;
  loadingErrorMessage.value = "";
  axios
    .get("/api/payment/" + props.id)
    .then((response) => {
      payment.value = response.data;
      if (payment.value) {
        editorModel.value = payment.value.note;
      }
    })
    .catch((error) => {
      payment.value = null;
      loadingErrorMessage.value = error;
    })
    .finally(() => {
      isLoadingPayment.value = false;
    });
}

function updateAttachments(newAttachments: Array<Attachment>) {
  if (payment.value) {
    payment.value.attachments = newAttachments;
  }
}
</script>
<style scoped lang="scss"></style>
