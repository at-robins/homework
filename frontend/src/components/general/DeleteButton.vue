<template>
  <q-btn
    @mouseover="isHovering = true"
    @mouseout="endHovering"
    @mousedown.left="startClicking"
    @mouseup.left="endClicking"
    :size="size"
    flat
    dense
    round
    icon="delete"
    :color="!error ? colour : 'negative'"
    :loading="loading || clickTimer !== null"
    :percentage="percentage"
  >
    <template v-if="!loading" v-slot:loading>
      <q-icon name="delete" />
    </template>
    <q-tooltip>
      <div v-if="!error">
        {{ tooltip }}
      </div>
      <div v-else>
        {{ error }}
      </div>
    </q-tooltip>
  </q-btn>
</template>

<script setup lang="ts">
import { ref, type Ref } from "vue";

const emit = defineEmits<{
  (event: "deletionConfirmed"): void;
}>();

const props = defineProps({
  size: { type: String, required: false, default: () => "" },
  colour: { type: String, required: false, default: () => "grey" },
  tooltip: { type: String, required: false, default: () => "" },
  error: { type: String, required: false, default: () => "" },
  loading: { type: Boolean, required: false, default: () => false },
});

const isHovering = ref(false);
const isClicking = ref(false);
const clickTimer: Ref<number | null> = ref(null);
const startClickedMilliseconds: Ref<number | null> = ref(null);
const percentage = ref(0.0);
const MAX_CLICK_TIME = 1000.0;
const UPDATE_INTERVALL = 100.0;
const DELETE_DELAY = 500.0;

function endHovering() {
  if (clickTimer.value !== null) {
    clearTimeout(clickTimer.value);
    clickTimer.value = null;
  }
  isHovering.value = false;
  isClicking.value = false;
  startClickedMilliseconds.value = null;
  percentage.value = 0.0;
}

function endClicking() {
  if (clickTimer.value !== null) {
    clearTimeout(clickTimer.value);
    clickTimer.value = null;
  }
  isClicking.value = false;
  startClickedMilliseconds.value = null;
  percentage.value = 0.0;
}

function startClicking() {
  if (
    isHovering.value &&
    !isClicking.value &&
    clickTimer.value === null &&
    !props.loading
  ) {
    startClickedMilliseconds.value = performance.now();
    isClicking.value = true;
    percentage.value = 0.0;
    clickTimer.value = setTimeout(updateClicking, UPDATE_INTERVALL);
  }
}

function updateClicking() {
  if (
    isHovering.value &&
    isClicking.value &&
    startClickedMilliseconds.value != null &&
    !props.loading
  ) {
    const millisecondsClicked =
      performance.now() - startClickedMilliseconds.value;
    if (millisecondsClicked >= MAX_CLICK_TIME) {
      percentage.value = 100.0;
      clickTimer.value = setTimeout(confirmDeletion, DELETE_DELAY);
    } else {
      percentage.value = 100.0 * (millisecondsClicked / MAX_CLICK_TIME);
      clickTimer.value = setTimeout(updateClicking, UPDATE_INTERVALL);
    }
  }
}

function confirmDeletion() {
  clickTimer.value = null;
  if (
    isHovering.value &&
    isClicking.value &&
    startClickedMilliseconds.value != null &&
    !props.loading
  ) {
    emit("deletionConfirmed");
  }
}
</script>
<style scoped lang="scss"></style>
