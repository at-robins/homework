<template>
  <q-layout container style="min-height: 300px">
    <q-page-container>
      <q-page>
        <q-editor
          @blur="updateInstructions"
          style="min-width: 100%; height: 100%; min-height: 300px"
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
        <q-page-sticky position="bottom-right" :offset="[18, 18]">
          <q-btn
            fab
            outline
            :color="!updatingErrorMessage ? 'primary' : 'negative'"
            @click="updateInstructions"
          >
            <q-spinner v-if="isUpdatingInstructions" color="primary" />
            <q-icon
              v-else
              name="cloud_upload"
              :color="!updatingErrorMessage ? 'primary' : 'negative'"
            />
            <q-tooltip>
              <div v-if="!updatingErrorMessage">
                Änderungen werden automatisch gespeichert.
              </div>
              <div v-else>
                {{ updatingErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
        </q-page-sticky>
      </q-page>
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import axios from "axios";
import { ref, watch } from "vue";

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

let updatingInstructionsTimer: number | null = null;

watch(
  () => props.modelValue,
  (newModelValue: string) => {
    editorModel.value = newModelValue;
  }
);

function updateInstructions() {
  if (
    !isUpdatingInstructions.value &&
    (props.modelValue !== editorModel.value || !!updatingErrorMessage.value)
  ) {
    if (updatingInstructionsTimer !== null) {
      clearTimeout(updatingInstructionsTimer);
    }
    isUpdatingInstructions.value = true;
    updatingErrorMessage.value = "";
    const formData = JSON.stringify(editorModel.value);
    const config = {
      headers: {
        "content-type": "application/json",
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
        updatingInstructionsTimer = setTimeout(
          () => (isUpdatingInstructions.value = false),
          1000
        );
      });
  }
}
</script>
<style scoped lang="scss"></style>
