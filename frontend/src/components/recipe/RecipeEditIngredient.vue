<template>
  <div v-click-away="clickOutside" class="q-pa-xs q-gutter-xs">
    <q-item class="row">
      <q-item-section top class="col-xs-3 col-md-2 col-lg-1">
        <q-item-label
          v-if="ingredient.id && !editMode"
          class="q-mt-sm text-right"
          >{{ ingredient.amount }}</q-item-label
        >
        <q-input
          v-else
          ref="amountInputRef"
          v-model="amountModel"
          :placeholder="t('recipe_ingredient_amount_label')"
          input-class="text-right"
          @keydown.enter="addOrUpdateIngredient"
        ></q-input>
      </q-item-section>

      <q-item-section top class="col-xs-3 col-md-2 col-lg-1">
        <q-item-label v-if="ingredient.id && !editMode" class="q-mt-sm"
          ><b>{{ ingredient.unit }}</b></q-item-label
        >
        <q-input
          v-else
          v-model="unitModel"
          :placeholder="t('recipe_ingredient_unit_label')"
          @keydown.enter="addOrUpdateIngredient"
        ></q-input>
      </q-item-section>

      <q-item-section top class="col-xs-4 col-md-6 col-lg-8">
        <q-item-label v-if="ingredient.id && !editMode" class="q-mt-sm">
          <router-link
            v-if="ingredient.recipeReference"
            :to="'/ui/recipe/' + ingredient.recipeReference"
          >
            {{ ingredient.text }}
          </router-link>
          <div v-else>
            {{ ingredient.text }}
          </div>
        </q-item-label>
        <q-input
          v-else-if="!showRecipeReferencesMode && !showFilterTextMode"
          v-model="textModel"
          :placeholder="t('recipe_ingredient_name_label')"
          @keydown.enter="addOrUpdateIngredient"
        />
        <q-input
          v-else-if="!showRecipeReferencesMode && showFilterTextMode"
          :placeholder="t('recipe_ingredient_filter_text_label')"
          v-model="filterTextModel"
          @keydown.enter="confirmFilterText"
        />
        <q-select
          v-else
          v-model="recipeReferenceModel"
          :label="t('recipe_ingredient_reference_label')"
          :options="availableRecipeReferences.references"
          emit-value
          map-options
          @keydown.enter="selectedRecipeReference"
          @update:model-value="selectedRecipeReference"
        >
          <template v-slot:append>
            <q-icon
              v-if="recipeReferenceModel"
              class="cursor-pointer"
              :name="matClear"
              @click.stop="
                recipeReferenceModel = null;
                showRecipeReferencesMode = false;
              "
            />
          </template>
        </q-select>
      </q-item-section>

      <q-item-section top side class="col-2">
        <div class="text-grey-8 q-gutter-xs">
          <q-btn
            v-show="ingredient.id && !editMode"
            class="gt-xs"
            size="sm"
            flat
            dense
            round
            :icon="matArrowDropUp"
            :color="!updateOrderingErrorMessage ? 'grey' : 'negative'"
            :loading="isCreatingOrUpdatingIngredient || isUdatingOrdering"
            @click="clickUpButton"
          >
            <q-tooltip ref="upButtonRef">
              <div v-if="!updateOrderingErrorMessage">
                {{ t("recipe_ingredient_move_up_tooltip") }}
              </div>
              <div v-else>
                {{ updateOrderingErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
          <q-btn
            v-show="ingredient.id && !editMode"
            class="gt-xs"
            size="sm"
            flat
            dense
            round
            :icon="matArrowDropDown"
            :color="!updateOrderingErrorMessage ? 'grey' : 'negative'"
            :loading="isCreatingOrUpdatingIngredient || isUdatingOrdering"
            @click="clickDownButton"
          >
            <q-tooltip ref="downButtonRef">
              <div v-if="!updateOrderingErrorMessage">
                {{ t("recipe_ingredient_move_down_tooltip") }}
              </div>
              <div v-else>
                {{ updateOrderingErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
          <q-btn
            v-show="ingredient.id && !editMode"
            class="gt-xs"
            size="sm"
            flat
            dense
            round
            :icon="matEdit"
            :color="!creationOrUpdateErrorMessage ? 'grey' : 'negative'"
            :loading="isCreatingOrUpdatingIngredient || isUdatingOrdering"
            @click="clickEditButton"
          >
            <q-tooltip>
              <div v-if="!creationOrUpdateErrorMessage">
                {{ t("recipe_ingredient_edit_tooltip") }}
              </div>
              <div v-else>
                {{ creationOrUpdateErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
          <q-btn
            v-show="!ingredient.id || editMode"
            class="gt-xs"
            size="sm"
            flat
            dense
            round
            :icon="matCheck"
            :color="!creationOrUpdateErrorMessage ? 'grey' : 'negative'"
            :loading="isCreatingOrUpdatingIngredient"
            @click="addOrUpdateIngredient"
          >
            <q-tooltip>
              <div v-if="!creationOrUpdateErrorMessage">
                {{ t("recipe_ingredient_add_tooltip") }}
              </div>
              <div v-else>
                {{ creationOrUpdateErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
          <q-btn
            v-show="!ingredient.id || editMode"
            class="gt-xs"
            size="12px"
            flat
            dense
            round
            :icon="filterTextModel ? matFilterAlt : matFilterAltOff"
            :color="showFilterTextMode ? 'primary' : 'grey'"
            @click="clickFilterTextButton"
          >
            <q-tooltip>
              <div>
                {{ t("recipe_ingredient_filter_text_tooltip") }}
              </div>
            </q-tooltip>
          </q-btn>
          <q-btn
            v-show="!ingredient.id || editMode"
            class="gt-xs"
            size="12px"
            flat
            dense
            round
            :icon="matAttachment"
            :color="
              !availableRecipeReferences.error
                ? showRecipeReferencesMode
                  ? 'primary'
                  : 'grey'
                : 'negative'
            "
            @click="clickRecipeReferenceButton"
          >
            <q-tooltip>
              <div v-if="!availableRecipeReferences.error">
                {{ t("recipe_ingredient_link_recipe_label") }}
              </div>
              <div v-else>
                {{ availableRecipeReferences.error }}
              </div>
            </q-tooltip>
          </q-btn>
          <delete-button
            v-show="props.ingredient.id && !editMode"
            class="gt-xs"
            size="12px"
            color="grey"
            :loading="isDeltingIngredient"
            :error="deletionErrorMessage"
            :tooltip="t('recipe_ingredient_deletion_tooltip')"
            @deletion-confirmed="deleteIngredient"
          />
        </div>
      </q-item-section>
    </q-item>
  </div>
</template>

<script setup lang="ts">
import type { Ingredient, RecipeReferences } from "@/scripts/types";
import { equality_shallow_object } from "@/scripts/utilities";
import axios from "axios";
import type { QTooltip } from "quasar";
import { nextTick, ref, watch, type Ref } from "vue";
import DeleteButton from "../general/DeleteButton.vue";
import {
  matArrowDropDown,
  matArrowDropUp,
  matAttachment,
  matCheck,
  matClear,
  matEdit,
  matFilterAlt,
  matFilterAltOff,
} from "@quasar/extras/material-icons";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const emit = defineEmits<{
  (event: "addedIngredient", ingredient: Ingredient): void;
  (event: "updatedIngredient", ingredient: Ingredient): void;
  (event: "removedIngredient", id: string): void;
  (event: "moveIngredientUp", id: string): void;
  (event: "moveIngredientDown", id: string): void;
}>();

const props = defineProps({
  ingredient: { type: Object as () => Ingredient, required: true },
  availableRecipeReferences: {
    type: Object as () => RecipeReferences,
    required: false,
    default: () => {
      return { error: "", references: [] };
    },
  },
  ordering: {
    type: Number,
    required: false,
    default: () => {
      return 0;
    },
  },
  isUdatingOrdering: {
    type: Boolean,
    required: false,
    default: () => {
      return false;
    },
  },
  updateOrderingErrorMessage: {
    type: String,
    required: false,
    default: () => {
      return "";
    },
  },
});

const isCreatingOrUpdatingIngredient = ref(false);
const isDeltingIngredient = ref(false);
const creationOrUpdateErrorMessage = ref("");
const deletionErrorMessage = ref("");
const amountModel = ref(props.ingredient.amount);
const unitModel = ref(props.ingredient.unit);
const textModel = ref(props.ingredient.text);
const recipeReferenceModel = ref(props.ingredient.recipeReference);
const filterTextModel = ref(props.ingredient.filterText);
const editMode = ref(false);
const showRecipeReferencesMode = ref(false);
const showFilterTextMode = ref(false);
const amountInputRef: Ref<HTMLInputElement | null> = ref(null);
const upButtonRef: Ref<QTooltip | null> = ref(null);
const downButtonRef: Ref<QTooltip | null> = ref(null);

watch(
  () => props.ingredient,
  (newValue) => {
    amountModel.value = newValue.amount;
    unitModel.value = newValue.unit;
    textModel.value = newValue.text;
    recipeReferenceModel.value = newValue.recipeReference;
    filterTextModel.value = newValue.filterText;
  }
);

function addOrUpdateIngredient() {
  if (props.ingredient.id) {
    updateIngredient();
  } else {
    addIngredient();
  }
}

function clickOutside() {
  if (!props.ingredient.id || editMode.value) {
    addOrUpdateIngredient();
  }
}

function clickEditButton() {
  editMode.value = true;
  nextTick(() => {
    if (amountInputRef.value) {
      amountInputRef.value.focus();
    }
  });
}

function clickRecipeReferenceButton() {
  showFilterTextMode.value = false;
  showRecipeReferencesMode.value = !showRecipeReferencesMode.value;
}

function clickFilterTextButton() {
  showRecipeReferencesMode.value = false;
  showFilterTextMode.value = !showFilterTextMode.value;
}

function clickUpButton() {
  emit("moveIngredientUp", props.ingredient.id);
  // Manually hide the tooltip to prevent it being permanently shown after moving the target button.
  if (upButtonRef.value) {
    upButtonRef.value.hide();
  }
}

function clickDownButton() {
  emit("moveIngredientDown", props.ingredient.id);
  // Manually hide the tooltip to prevent it being permanently shown after moving the target button.
  if (downButtonRef.value) {
    downButtonRef.value.hide();
  }
}

function addIngredient() {
  if (
    !isCreatingOrUpdatingIngredient.value &&
    !props.ingredient.id &&
    (amountModel.value ||
      unitModel.value ||
      textModel.value ||
      recipeReferenceModel.value)
  ) {
    isCreatingOrUpdatingIngredient.value = true;
    creationOrUpdateErrorMessage.value = "";
    const createdIngredient: Ingredient = {
      // This is a dummy address since the server expects a valid UUID
      // during deserialisation. It will be overwritten on the server side.
      id: "8a6e0804-2bd0-4672-b79d-d97027f9071a",
      amount: amountModel.value,
      unit: unitModel.value,
      text: textModel.value,
      recipeReference: recipeReferenceModel.value,
      recipeId: props.ingredient.recipeId,
      // The timestamp will be overwritten on the server side.
      creationTime: new Date().toISOString(),
      ordering: props.ordering,
      filterText: filterTextModel.value,
    };
    const formData = JSON.stringify(createdIngredient);
    const config = {
      headers: {
        "Content-Type": "application/json",
      },
    };
    axios
      .post(
        "/api/recipe/" + props.ingredient.recipeId + "/ingredients",
        formData,
        config
      )
      .then((response) => {
        createdIngredient.id = response.data;
        emit("addedIngredient", createdIngredient);
        amountModel.value = props.ingredient.amount;
        unitModel.value = props.ingredient.unit;
        textModel.value = props.ingredient.text;
        recipeReferenceModel.value = props.ingredient.recipeReference;
        if (amountInputRef.value) {
          amountInputRef.value.focus();
        }
      })
      .catch((error) => {
        creationOrUpdateErrorMessage.value = error;
      })
      .finally(() => {
        isCreatingOrUpdatingIngredient.value = false;
        editMode.value = false;
        showRecipeReferencesMode.value = false;
      });
  }
}

function updateIngredient() {
  if (!isCreatingOrUpdatingIngredient.value && props.ingredient.id) {
    const updatedIngredient: Ingredient = {
      id: props.ingredient.id,
      amount: amountModel.value,
      unit: unitModel.value,
      text: textModel.value,
      recipeReference: recipeReferenceModel.value,
      recipeId: props.ingredient.recipeId,
      creationTime: props.ingredient.creationTime,
      ordering: props.ordering,
      filterText: filterTextModel.value,
    };
    // Only update if there are changes.
    if (!equality_shallow_object(updatedIngredient, props.ingredient)) {
      isCreatingOrUpdatingIngredient.value = true;
      creationOrUpdateErrorMessage.value = "";
      const formData = JSON.stringify(updatedIngredient);
      const config = {
        headers: {
          "Content-Type": "application/json",
        },
      };
      axios
        .patch(
          "/api/recipe/" + props.ingredient.recipeId + "/ingredients",
          formData,
          config
        )
        .then(() => {
          emit("updatedIngredient", updatedIngredient);
        })
        .catch((error) => {
          creationOrUpdateErrorMessage.value = error;
        })
        .finally(() => {
          isCreatingOrUpdatingIngredient.value = false;
          editMode.value = false;
          showRecipeReferencesMode.value = false;
          showFilterTextMode.value = false;
        });
    } else {
      editMode.value = false;
      showRecipeReferencesMode.value = false;
      showFilterTextMode.value = false;
    }
  }
}

function deleteIngredient() {
  if (!isDeltingIngredient.value && props.ingredient.id) {
    isDeltingIngredient.value = true;
    deletionErrorMessage.value = "";
    const ingredientId = props.ingredient.id;
    axios
      .delete(
        "/api/recipe/" +
          props.ingredient.recipeId +
          "/ingredient/" +
          ingredientId
      )
      .then(() => {
        emit("removedIngredient", ingredientId);
      })
      .catch((error) => {
        deletionErrorMessage.value = error;
      })
      .finally(() => {
        isDeltingIngredient.value = false;
      });
  }
}

function confirmFilterText() {
  showFilterTextMode.value = false;
  addOrUpdateIngredient();
}

function selectedRecipeReference() {
  showRecipeReferencesMode.value = false;
  if (!textModel.value) {
    const mapping = props.availableRecipeReferences.references.find(
      (recipeRef) => recipeRef.value === recipeReferenceModel.value
    );
    if (mapping) {
      textModel.value = mapping.label;
    }
  }
}
</script>
<style scoped lang="scss"></style>
