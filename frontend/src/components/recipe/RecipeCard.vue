<template>
  <q-card bordered class="cursor-pointer" @click="navigateToRecipe">
    <q-img :src="imageUrl" ratio="1">
      <div class="absolute-bottom text-subtitle2 text-center">
        <div>
          {{ recipe.title }}
        </div>
        <delete-button
          class="button-delete"
          :loading="isDeltingRecipe"
          :error="deletionErrorMessage"
          tooltip="Halten um das Rezept zu löschen."
          @deletion-confirmed="deleteRecipe"
        />
      </div>
    </q-img>
  </q-card>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import { getImageAttachmentUrl } from "@/scripts/utilities";
import axios from "axios";
import { computed, ref } from "vue";
import { useRouter } from "vue-router";
import DeleteButton from "../general/DeleteButton.vue";

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const emit = defineEmits<{
  (event: "deletedRecipe", id: string): void;
}>();

const imageUrl = computed(() => {
  return getImageAttachmentUrl(props.recipe.thumbnail);
});

const router = useRouter();
const isDeltingRecipe = ref(false);
const deletionErrorMessage = ref("");

function navigateToRecipe() {
  router.push({ name: "recipe", params: { id: props.recipe.id } });
}

function deleteRecipe() {
  if (!isDeltingRecipe.value) {
    isDeltingRecipe.value = true;
    deletionErrorMessage.value = "";
    axios
      .delete("/api/recipe/" + props.recipe.id)
      .then(() => {
        emit("deletedRecipe", props.recipe.id);
      })
      .catch((error) => {
        deletionErrorMessage.value = error;
      })
      .finally(() => {
        isDeltingRecipe.value = false;
      });
  }
}
</script>
<style scoped lang="scss">
.button-delete {
  position: absolute;
  right: 4px;
  bottom: 4px;
}
</style>
