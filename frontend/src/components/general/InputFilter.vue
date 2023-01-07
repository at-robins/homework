<template>
  <q-select
    v-model="selectedModel"
    :options="inputFilteredOptions"
    :label="label"
    ref="filterReference"
    hide-dropdown-icon
    clearable
    multiple
    use-input
    input-debounce="0"
    @filter="applyInputFilter"
  >
    <template v-slot:append>
      <q-icon
        :name="matSend"
        @click.stop.prevent="closeFilterSelection"
        class="q-field__focusable-action"
      />
    </template>
  </q-select>
</template>

<script setup lang="ts">
import type { QSelect } from "quasar";
import { matSend } from "@quasar/extras/material-icons";
import { ref, watch, type PropType, type Ref } from "vue";

const props = defineProps({
  options: { type: Array as () => Array<string>, required: true },
  modelValue: {
    type: null as unknown as PropType<string | Array<string> | null>,
    required: true,
  },
  label: { type: String, required: false, default: () => "" },
  multiple: { type: Boolean, required: false, default: () => false },
});

const emit = defineEmits<{
  (
    event: "update:modelValue",
    selectedOptions: string | Array<string> | null
  ): void;
}>();

const filterReference: Ref<QSelect | null> = ref(null);

const selectedModel: Ref<string | Array<string> | null> = ref(null);

const inputFilteredOptions: Ref<Array<string>> = ref(props.options);

watch(selectedModel, () => {
  emit("update:modelValue", selectedModel.value);
});

function applyInputFilter(
  val: string,
  update: (
    callback: () => void,
    referenceCallback: (qSelectReference: QSelect) => void
  ) => void
): void {
  update(
    // input filtering
    () => {
      if (!val) {
        inputFilteredOptions.value = props.options;
      } else {
        const needle = val.toLowerCase();
        inputFilteredOptions.value = props.options.filter((v) =>
          v.toLowerCase().includes(needle)
        );
      }
    },
    // autoselect
    (ref: QSelect) => {
      if (!!val && ref.options && ref.options.length > 0) {
        ref.setOptionIndex(-1);
        ref.moveOptionSelection(1, true);
      }
    }
  );
}

function closeFilterSelection() {
  if (filterReference.value) {
    filterReference.value.hidePopup();
  }
}
</script>
<style scoped lang="scss"></style>
