<template>
  <div class="q-pa-md q-gutter-md">
    <q-file
      color="teal"
      filled
      v-model="model"
      label="Anhang hochladen"
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
      <q-item-label header>Anhänge</q-item-label>

      <div v-for="(attachment, index) in attachments" :key="attachment.id">
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
              <q-btn
                class="gt-xs"
                size="12px"
                flat
                dense
                round
                icon="delete"
                :color="
                  !deletionErrorMessages.has(attachment.id)
                    ? 'grey'
                    : 'negative'
                "
                :loading="isDeltingAttachment.includes(attachment.id)"
                @click="deleteAttachment(attachment.id)"
              >
                <q-tooltip v-if="!deletionErrorMessages.has(attachment.id)">
                  Anhang löschen
                </q-tooltip>
                <q-tooltip v-else>
                  {{ deletionErrorMessages.get(attachment.id) }}
                </q-tooltip>
              </q-btn>
            </div>
          </q-item-section>
        </q-item>

        <q-separator spaced v-if="index + 1 < attachments.length" />
      </div>
      <q-inner-loading :showing="isLoadingAttachments">
        <q-spinner size="50px" color="primary" />
      </q-inner-loading>
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
import axios from "axios";
import { ref, type Ref } from "vue";
const attachments: Ref<Array<Attachment>> = ref([]);
const model: Ref<File | null> = ref(null);
const isLoadingAttachments = ref(false);
const isUploadingAttachment = ref(false);
const isDeltingAttachment: Ref<Array<string>> = ref([]);
const deletionErrorMessages: Ref<Map<string, string>> = ref(new Map());
const loadAttachmentsErrorMessage = ref("");
const uploadErrorMessage = ref("");
loadAttachments();

function loadAttachments() {
  isLoadingAttachments.value = true;
  loadAttachmentsErrorMessage.value = "";
  axios
    .get("/api/attachments")
    .then((response) => {
      attachments.value = response.data;
    })
    .catch((error) => {
      attachments.value = [];
      loadAttachmentsErrorMessage.value = error;
    })
    .finally(() => {
      isLoadingAttachments.value = false;
    });
}

function uploadAttachment(value: File | null) {
  if (value) {
    isUploadingAttachment.value = true;
    uploadErrorMessage.value = "";
    const formData = new FormData();
    formData.append("file", value);
    const config = {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    };
    axios
      .post("/api/attachments", formData, config)
      .then(() => {
        loadAttachments();
      })
      .catch((error) => {
        uploadErrorMessage.value = error;
      })
      .finally(() => {
        model.value = null;
        isUploadingAttachment.value = false;
      });
  }
}

function deleteAttachment(id: string) {
  isDeltingAttachment.value.push(id);
  axios
    .delete("/api/attachment/" + id)
    .then(() => {
      loadAttachments();
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
