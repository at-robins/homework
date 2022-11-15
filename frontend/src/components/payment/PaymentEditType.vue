<template>
  <div class="row">
    <q-select v-model="typeModel" :options="typeOptions" />
    <q-input
      v-if="!isOneOffPayment"
      v-model="distanceModel"
      type="text"
      mask="#"
      reverse-fill-mask
      placeholder="Abstand"
    />
    <q-input filled v-model="startModel">
      <template v-slot:prepend>
        <q-icon name="event" class="cursor-pointer">
          <q-popup-proxy cover transition-show="scale" transition-hide="scale">
            <q-date v-model="startModel" :mask="quasarTimeFormat">
              <div class="row items-center justify-end">
                <q-btn v-close-popup label="Bestätigen" color="primary" flat />
              </div>
            </q-date>
          </q-popup-proxy>
        </q-icon>
      </template>

      <template v-slot:append>
        <q-icon name="access_time" class="cursor-pointer">
          <q-popup-proxy cover transition-show="scale" transition-hide="scale">
            <q-time v-model="startModel" :mask="quasarTimeFormat" format24h>
              <div class="row items-center justify-end">
                <q-btn v-close-popup label="Bestätigen" color="primary" flat />
              </div>
            </q-time>
          </q-popup-proxy>
        </q-icon>
      </template>
    </q-input>
    <q-input filled v-model="endModel">
      <template v-slot:prepend>
        <q-icon name="event" class="cursor-pointer">
          <q-popup-proxy cover transition-show="scale" transition-hide="scale">
            <q-date v-model="endModel" :mask="quasarTimeFormat">
              <div class="row items-center justify-end">
                <q-btn v-close-popup label="Bestätigen" color="primary" flat />
              </div>
            </q-date>
          </q-popup-proxy>
        </q-icon>
      </template>

      <template v-slot:append>
        <q-icon name="access_time" class="cursor-pointer">
          <q-popup-proxy cover transition-show="scale" transition-hide="scale">
            <q-time v-model="endModel" :mask="quasarTimeFormat" format24h>
              <div class="row items-center justify-end">
                <q-btn v-close-popup label="Bestätigen" color="primary" flat />
              </div>
            </q-time>
          </q-popup-proxy>
        </q-icon>
      </template>
    </q-input>
  </div>
</template>

<script setup lang="ts">
import type { Payment } from "@/scripts/types";
import {
  getPaymentTypeDistance,
  getPaymentTypeEnd,
  getPaymentTypeStart,
  paymentTypeToLabel,
} from "@/scripts/utilities";
import axios from "axios";
import type { DateTime } from "luxon";
import { computed, ref } from "vue";

const props = defineProps({
  payment: { type: Object as () => Payment, required: true },
});

const luxonTimeFormat = "dd.MM.yyyy, HH:mm";
const quasarTimeFormat = "DD.MM.YYYY, HH:mm";
const isUpdatingType = ref(false);
const updateTypeErrorMessage = ref("");
const editTypeMode = ref(false);
const typeModel = ref(paymentTypeToLabel(props.payment.paymentType));
const distanceModel = ref(getPaymentTypeDistance(props.payment.paymentType));
const startModel = ref(
  formatDateTime(getPaymentTypeStart(props.payment.paymentType))
);
const endModel = ref(
  formatDateTime(getPaymentTypeEnd(props.payment.paymentType))
);
const typeOptions = [
  {
    label: "Einmalzahlung",
    value: "OneOff",
  },
  {
    label: "Täglich",
    value: "Daily",
  },
  {
    label: "Wöchentlich",
    value: "Weekly",
  },
  {
    label: "Monatlich",
    value: "Monthly",
  },
  {
    label: "Jährlich",
    value: "Annualy",
  },
];

const isOneOffPayment = computed(() => {
  return Object.keys(props.payment.paymentType).includes("OneOff");
});

function formatDateTime(time: DateTime | null): string | null {
  if (time) {
    return time.toFormat(luxonTimeFormat, {
      locale: "de",
    });
  } else {
    return null;
  }
}

function updateType() {
  isUpdatingType.value = true;
  updateTypeErrorMessage.value = "";
  const formData = JSON.stringify(typeModel.value);
  const config = {
    headers: {
      "content-type": "application/json",
    },
  };
  axios
    .post("/api/payment/" + props.payment.id + "/type", formData, config)
    .catch((error) => {
      updateTypeErrorMessage.value = error;
    })
    .finally(() => {
      isUpdatingType.value = false;
      editTypeMode.value = false;
    });
}
</script>
<style scoped lang="scss"></style>
