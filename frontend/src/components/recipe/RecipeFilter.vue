<template>
  <div class="row wrap q-gutter-md q-pa-md">
    <q-select
      v-model="nameModel"
      :options="nameOptions"
      label="Rezeptnamen filtern"
      clearable
      use-input
      input-debounce="0"
      @filter="applyInputFilterNames"
      class="col-3"
    />
    <q-select
      v-model="tagModel"
      :options="tagOptions"
      label="Schlagwörter filtern"
      clearable
      multiple
      use-input
      input-debounce="0"
      @filter="applyInputFilterTags"
      class="col-3"
    />
    <q-select
      v-model="ingredientModel"
      :options="ingredientOptions"
      label="Zutaten filtern"
      clearable
      multiple
      use-input
      input-debounce="0"
      @filter="applyInputFilterIngredients"
      class="col-3"
    />
  </div>
</template>

<script setup lang="ts">
import type { Recipe } from "@/scripts/types";
import type { QSelect } from "quasar";
import { computed, ref, watch, type Ref } from "vue";

const props = defineProps({
  recipes: { type: Array as () => Array<Recipe>, required: true },
});

const emit = defineEmits<{
  (event: "updatedFilter", recipes: Array<Recipe>): void;
}>();

const tagModel: Ref<Array<string>> = ref([]);
const ingredientModel: Ref<Array<string>> = ref([]);
const nameModel = ref("");

const allTags = computed(() => {
  const tags = props.recipes.flatMap((recipe) => recipe.tags);
  return [...new Set<string>(tags)].sort();
});

const allNames = computed(() => {
  const names = props.recipes.flatMap((recipe) => recipe.title);
  return [...new Set<string>(names)];
});

const allIngredients = computed(() => {
  const ingredients = props.recipes
    .flatMap((recipe) => recipe.ingredients)
    .map((ingredient) => ingredient.text);
  return [...new Set<string>(ingredients)].sort();
});

const nameOptions: Ref<Array<string>> = ref(allNames.value);
const tagOptions: Ref<Array<string>> = ref(allTags.value);
const ingredientOptions: Ref<Array<string>> = ref(allIngredients.value);

watch([nameModel, tagModel, ingredientModel], () => {
  emit("updatedFilter", matchingRecipes());
});

function matchingRecipes(): Recipe[] {
  let filteredRecipes = props.recipes;
  if (nameModel.value) {
    filteredRecipes = filteredRecipes.filter((recipe) =>
      recipe.title.includes(nameModel.value)
    );
  }
  if (tagModel.value.length > 0) {
    filteredRecipes = filteredRecipes.filter((recipe) =>
      tagModel.value.every((tag) => recipe.tags.includes(tag))
    );
  }
  if (ingredientModel.value.length > 0) {
    filteredRecipes = filteredRecipes.filter((recipe) =>
      ingredientModel.value.every((ingredient) =>
        recipe.ingredients.map((i) => i.text).includes(ingredient)
      )
    );
  }
  return filteredRecipes;
}

function applyInputFilterNames(
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
        nameOptions.value = allNames.value;
      } else {
        const needle = val.toLowerCase();
        nameOptions.value = allNames.value.filter((v) =>
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

function applyInputFilterTags(
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
        tagOptions.value = allTags.value;
      } else {
        const needle = val.toLowerCase();
        tagOptions.value = allTags.value.filter((v) =>
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

function applyInputFilterIngredients(
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
        ingredientOptions.value = allIngredients.value;
      } else {
        const needle = val.toLowerCase();
        ingredientOptions.value = allIngredients.value.filter((v) =>
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
</script>
<style scoped lang="scss"></style>
