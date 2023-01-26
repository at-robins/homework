<template>
  <div style="min-width: 100%">
    <div class="row">
      <q-item-label header>{{ t("recipe_instructions_title") }}</q-item-label>
      <q-btn
        outline
        round
        size="xs"
        class="self-start q-mt-xs"
        :color="!updatingErrorMessage ? 'primary' : 'negative'"
        @click="updateInstructionsOrChangeToEditMode"
      >
        <q-spinner v-if="isUpdatingInstructions" color="primary" />
        <q-icon
          v-else
          :name="editMode ? matCheck : matEdit"
          :color="!updatingErrorMessage ? 'primary' : 'negative'"
        />
        <q-tooltip>
          <div v-if="!updatingErrorMessage">
            <div v-show="!editMode">
              {{ t("recipe_instructions_edit_tooltip") }}
            </div>
            {{ t("recipe_instructions_save_tooltip") }}
          </div>
          <div v-else>
            {{ updatingErrorMessage }}
          </div>
        </q-tooltip>
      </q-btn>
    </div>
    <div>
      <div v-show="!editMode" v-html="editorModel" class="q-pa-md" />
      <div v-show="editMode" style="min-width: 100%">
        <q-editor
          @update:model-value="onEditorUpdate"
          :readonly="!editMode"
          style="min-width: 100%"
          v-model="editorModel"
          :dense="$q.screen.lt.md"
          :toolbar="[
            [
              {
                label: $q.lang.editor.align,
                icon: $q.iconSet.editor.align,
                fixedLabel: true,
                list: 'only-icons',
                options: ['left', 'center', 'right', 'justify'],
              },
              {
                label: $q.lang.editor.align,
                icon: $q.iconSet.editor.align,
                fixedLabel: true,
                options: ['left', 'center', 'right', 'justify'],
              },
            ],
            [
              'bold',
              'italic',
              'strike',
              'underline',
              'subscript',
              'superscript',
            ],
            ['token', 'hr', 'link', 'custom_btn'],
            [
              {
                label: $q.lang.editor.formatting,
                icon: $q.iconSet.editor.formatting,
                list: 'no-icons',
                options: ['p', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'code'],
              },
              {
                label: $q.lang.editor.fontSize,
                icon: $q.iconSet.editor.fontSize,
                fixedLabel: true,
                fixedIcon: true,
                list: 'no-icons',
                options: [
                  'size-1',
                  'size-2',
                  'size-3',
                  'size-4',
                  'size-5',
                  'size-6',
                  'size-7',
                ],
              },
              {
                label: $q.lang.editor.defaultFont,
                icon: $q.iconSet.editor.font,
                fixedIcon: true,
                list: 'no-icons',
                options: [
                  'default_font',
                  'arial',
                  'arial_black',
                  'comic_sans',
                  'courier_new',
                  'impact',
                  'lucida_grande',
                  'times_new_roman',
                  'verdana',
                ],
              },
              'removeFormat',
            ],
            ['quote', 'unordered', 'ordered', 'outdent', 'indent'],

            ['undo', 'redo'],
            ['viewsource'],
          ]"
          :fonts="{
            arial: 'Arial',
            arial_black: 'Arial Black',
            comic_sans: 'Comic Sans MS',
            courier_new: 'Courier New',
            impact: 'Impact',
            lucida_grande: 'Lucida Grande',
            times_new_roman: 'Times New Roman',
            verdana: 'Verdana',
          }"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import axios from "axios";
import { ref, watch, type Ref } from "vue";
import { matCheck, matEdit } from "@quasar/extras/material-icons";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps({
  id: { type: String, required: true },
  modelValue: { type: String, required: false, default: "" },
});
const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const isUpdatingInstructions = ref(false);
const updatingErrorMessage = ref("");
const editorModel = ref(props.modelValue);
const editMode = ref(false);
const updateTimer: Ref<number | null> = ref(null);
const UPDATE_DELAY = 5000.0;

watch(
  () => props.modelValue,
  (newModelValue: string) => {
    editorModel.value = newModelValue;
  }
);

function onEditorUpdate() {
  if (updateTimer.value) {
    clearTimeout(updateTimer.value);
  }
  updateTimer.value = setTimeout(clearTimerAndUpdateInstructions, UPDATE_DELAY);
}

function updateInstructionsOrChangeToEditMode() {
  if (editMode.value) {
    updateInstructions();
    editMode.value = false;
  } else {
    editMode.value = true;
  }
}

function clearTimerAndUpdateInstructions() {
  updateTimer.value = null;
  updateInstructions();
}

function updateInstructions() {
  if (
    !isUpdatingInstructions.value &&
    (props.modelValue !== editorModel.value || !!updatingErrorMessage.value)
  ) {
    isUpdatingInstructions.value = true;
    updatingErrorMessage.value = "";
    const formData = JSON.stringify(editorModel.value);
    const config = {
      headers: {
        "Content-Type": "application/json",
      },
    };
    axios
      .post(
        "/api/recipe/" + props.id + "/string/instructions",
        formData,
        config
      )
      .then(() => {
        emit("update:modelValue", editorModel.value);
      })
      .catch((error) => {
        updatingErrorMessage.value = error;
      })
      .finally(() => {
        isUpdatingInstructions.value = false;
      });
  }
}
</script>
<style scoped lang="scss">
.button-floating {
  position: absolute;
  right: 18px;
  bottom: 18px;
}
</style>
