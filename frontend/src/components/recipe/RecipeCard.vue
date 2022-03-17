<template>
  <q-card bordered class="cursor-pointer" @click="navigateToRecipe">
    <q-img :src="imageUrl" ratio="1">
      <div class="absolute-bottom text-subtitle2 text-center">
        {{ recipe.title }}
      </div>
    </q-img>
  </q-card>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import { getRecipeImageUrl } from "@/scripts/utilities";
import { computed } from "vue";
import { useRouter } from "vue-router";

const props = defineProps({
  recipe: { type: Object as () => Recipe, required: true },
});

const imageUrl = computed(() => {
  return getRecipeImageUrl(props.recipe);
});

const router = useRouter();

function navigateToRecipe() {
  router.push({ name: "recipe", params: { id: props.recipe.id } });
}
</script>
<style scoped lang="scss"></style>
