<template>
  <div class="q-pa-md q-gutter-md">
    <q-file
      color="teal"
      filled
      v-model="fileModel"
      :label="t('recipe_attachment_upload_label')"
      :loading="isUploadingAttachment"
      :readonly="isUploadingAttachment"
      :error="!!uploadErrorMessage"
      :error-message="uploadErrorMessage"
      max-files="1"
      @update:model-value="uploadAttachment"
    >
      <template v-slot:prepend>
        <q-icon name="cloud_upload" />
      </template>
    </q-file>
    <q-list bordered class="rounded-borders" style="max-width: 600px">
      <q-item-label header>{{ t("recipe_attachment_title") }}</q-item-label>

      <div
        v-for="(attachment, index) in recipe.attachments"
        :key="attachment.id"
      >
        <q-item>
          <q-item-section avatar top>
            <q-icon name="account_tree" color="black" size="34px" />
          </q-item-section>

          <q-item-section top>
            <q-item-label class="q-mt-sm"
              ><a :href="'/api/attachment/' + attachment.id">{{
                attachment.name
              }}</a></q-item-label
            >
          </q-item-section>

          <q-item-section top side>
            <div class="text-grey-8 q-gutter-xs">
              <delete-button
                class="gt-xs"
                size="12px"
                :tooltip="t('recipe_attachment_deletion_tooltip')"
                color="grey"
                :loading="isDeltingAttachment.includes(attachment.id)"
                :error="deletionErrorMessages.get(attachment.id)"
                @deletion-confirmed="deleteAttachment(attachment.id)"
              />
            </div>
          </q-item-section>
        </q-item>

        <q-separator spaced v-if="index + 1 < recipe.attachments.length" />
      </div>
      <q-item v-if="!!loadAttachmentsErrorMessage">
        <q-item-section avatar top>
          <q-icon name="warning" color="negative" />
        </q-item-section>
        <q-item-section top>
          <q-item-label class="q-mt-sm">{{
            loadAttachmentsErrorMessage
          }}</q-item-label>
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script setup lang="ts">
import type { Attachment } from "@/scripts/types";
import type { Recipe } from "@/scripts/types";
import axios from "axios";
import { ref, type Ref } from "vue";
import DeleteButton from "../general/DeleteButton.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const emit = defineEmits<{
  (event: "updatedAttachments", attachments: Array<Attachment>): void;
}>();

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const fileModel: Ref<File | null> = ref(null);
const isUploadingAttachment = ref(false);
const isDeltingAttachment: Ref<Array<string>> = ref([]);
const deletionErrorMessages: Ref<Map<string, string>> = ref(new Map());
const loadAttachmentsErrorMessage = ref("");
const uploadErrorMessage = ref("");

function uploadAttachment(value: File | null) {
  if (value) {
    isUploadingAttachment.value = true;
    uploadErrorMessage.value = "";
    const formData = new FormData();
    formData.append("file", value);
    let generatedAttachmentId: string;
    const config = {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    };
    axios
      .post("/api/attachments", formData, config)
      .then((response) => {
        generatedAttachmentId = response.data;
        const formDataReference = JSON.stringify(generatedAttachmentId);
        const configReference = {
          headers: {
            "Content-Type": "applocation/json",
          },
        };
        return axios.post(
          "/api/recipe/" + props.recipe.id + "/attachments",
          formDataReference,
          configReference
        );
      })
      .then(() => {
        const attachments = props.recipe.attachments.slice();
        attachments.push({
          id: generatedAttachmentId,
          name: value.name,
          creationTime: new Date(),
        });
        emit("updatedAttachments", attachments);
      })
      .catch((error) => {
        uploadErrorMessage.value = error;
      })
      .finally(() => {
        fileModel.value = null;
        isUploadingAttachment.value = false;
      });
  }
}

function deleteAttachment(id: string) {
  isDeltingAttachment.value.push(id);
  axios
    .delete("/api/attachment/" + id)
    .then(() => {
      const attachments = props.recipe.attachments.filter(
        (attachment) => attachment.id !== id
      );
      emit("updatedAttachments", attachments);
    })
    .catch((error) => {
      deletionErrorMessages.value.set(id, error);
    })
    .finally(() => {
      isDeltingAttachment.value = isDeltingAttachment.value.filter(
        (idPendingForDeletion) => idPendingForDeletion !== id
      );
    });
}
</script>
<style scoped lang="scss"></style>
