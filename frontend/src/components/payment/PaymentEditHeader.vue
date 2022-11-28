<template>
  <div class="row no-wrap">
    <q-item-label v-if="!editTargetMode" class="text-h3 shrink q-ma-sm">{{
      targetModel
    }}</q-item-label>
    <q-input
      v-else
      ref="targetInputRef"
      v-model="targetModel"
      @blur="updateTarget"
      @keydown.enter="updateTarget"
      :readonly="isUpdatingTarget"
      class="text-h3 col-grow q-ma-sm"
    />
    <q-btn
      v-if="!editTargetMode"
      round
      icon="edit"
      class="self-start"
      :color="!updateTargetErrorMessage ? 'primary' : 'negative'"
      outline
      size="sm"
      @click="editTargetField"
    >
      <q-tooltip>
        <div v-if="!updateTargetErrorMessage">Rezepttitel bearbeiten</div>
        <div v-else>
          {{ updateTargetErrorMessage }}
        </div>
      </q-tooltip>
    </q-btn>
    <q-btn v-else round class="self-start" color="primary" outline size="sm">
      <q-spinner v-if="isUpdatingTarget" color="primary" />
      <q-icon v-else name="done" color="primary" />
    </q-btn>
  </div>
</template>

<script setup lang="ts">
import type { Payment } from "@/scripts/types";
import axios from "axios";
import { nextTick, ref, watch, type Ref } from "vue";

const props = defineProps({
  payment: { type: Object as () => Payment, required: true },
});

const isUpdatingTarget = ref(false);
const updateTargetErrorMessage = ref("");
const editTargetMode = ref(false);
const targetModel = ref(props.payment.target);
const referenceTargetRef: Ref<HTMLInputElement | null> = ref(null);

function updateTarget() {
  isUpdatingTarget.value = true;
  updateTargetErrorMessage.value = "";
  const formData = JSON.stringify(targetModel.value);
  const config = {
    headers: {
      "Content-Type": "application/json",
    },
  };
  axios
    .post(
      "/api/payment/" + props.payment.id + "/string/target",
      formData,
      config
    )
    .catch((error) => {
      updateTargetErrorMessage.value = error;
    })
    .finally(() => {
      isUpdatingTarget.value = false;
      editTargetMode.value = false;
    });
}

function editTargetField() {
  editTargetMode.value = true;
  nextTick(() => {
    if (referenceTargetRef.value) {
      referenceTargetRef.value.focus();
    }
  });
}
</script>
<style scoped lang="scss"></style>
