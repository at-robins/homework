<template>
  <div class="q-pa-md q-gutter-md">
    <q-item class="row">
      <q-item-section top class="col-2">
        <q-item-label
          v-if="ingredient.id && !editMode"
          class="q-mt-sm text-right"
          >{{ ingredient.amount }}</q-item-label
        >
        <q-input
          v-else
          v-model="amountModel"
          input-class="text-right"
          @keydown.enter="addOrUpdateIngredient"
        ></q-input>
      </q-item-section>

      <q-item-section top class="col-1">
        <q-item-label v-if="ingredient.id && !editMode" class="q-mt-sm"
          ><b>{{ ingredient.unit }}</b></q-item-label
        >
        <q-input
          v-else
          v-model="unitModel"
          @keydown.enter="addOrUpdateIngredient"
        ></q-input>
      </q-item-section>

      <q-item-section top class="col-8">
        <q-item-label v-if="ingredient.id && !editMode" class="q-mt-sm"
          ><b>{{ ingredient.text }}</b></q-item-label
        >
        <q-input
          v-else
          v-model="textModel"
          @keydown.enter="addOrUpdateIngredient"
        ></q-input>
      </q-item-section>

      <q-item-section top side class="col-1">
        <div class="text-grey-8 q-gutter-xs">
          <q-btn
            v-if="ingredient.id && !editMode"
            class="gt-xs"
            size="12px"
            flat
            dense
            round
            icon="edit"
            :color="!creationOrUpdateErrorMessage ? 'grey' : 'negative'"
            :loading="isCreatingOrUpdatingIngredient"
            @click="editMode = true"
          >
            <q-tooltip>
              <div v-if="!creationOrUpdateErrorMessage">Zutat ändern</div>
              <div v-else>
                {{ creationOrUpdateErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
          <q-btn
            v-else
            class="gt-xs"
            size="12px"
            flat
            dense
            round
            icon="check"
            :color="!creationOrUpdateErrorMessage ? 'grey' : 'negative'"
            :loading="isCreatingOrUpdatingIngredient"
            @click="addOrUpdateIngredient"
          >
            <q-tooltip>
              <div v-if="!creationOrUpdateErrorMessage">Zutat hinzufügen</div>
              <div v-else>
                {{ creationOrUpdateErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
          <q-btn
            v-if="props.ingredient.id"
            class="gt-xs"
            size="12px"
            flat
            dense
            round
            icon="delete"
            :color="!deletionErrorMessage ? 'grey' : 'negative'"
            :loading="isDeltingIngredient"
            @click="deleteIngredient"
          >
            <q-tooltip>
              <div v-if="!deletionErrorMessage">Zutat löschen</div>
              <div v-else>
                {{ deletionErrorMessage }}
              </div>
            </q-tooltip>
          </q-btn>
        </div>
      </q-item-section>
    </q-item>
  </div>
</template>

<script setup lang="ts">
import type { Ingredient } from "@/scripts/types";
import axios from "axios";
import { ref, watch } from "vue";

const emit = defineEmits<{
  (event: "addedIngredient", ingredient: Ingredient): void;
  (event: "updatedIngredient", ingredient: Ingredient): void;
  (event: "removedIngredient", id: string): void;
}>();

const props = defineProps({
  ingredient: { type: Object as () => Ingredient, required: true },
});

const isCreatingOrUpdatingIngredient = ref(false);
const isDeltingIngredient = ref(false);
const creationOrUpdateErrorMessage = ref("");
const deletionErrorMessage = ref("");
const amountModel = ref(props.ingredient.amount);
const unitModel = ref(props.ingredient.unit);
const textModel = ref(props.ingredient.text);
const recipeReferenceModel = ref(props.ingredient.recipeReference);
const editMode = ref(false);

watch(props.ingredient, (newValue) => {
  amountModel.value = newValue.amount;
  unitModel.value = newValue.unit;
  textModel.value = newValue.text;
  recipeReferenceModel.value = newValue.recipeReference;
});

function addOrUpdateIngredient() {
  if (props.ingredient.id) {
    updateIngredient();
  } else {
    addIngredient();
  }
}

function addIngredient() {
  if (!isCreatingOrUpdatingIngredient.value && !props.ingredient.id) {
    isCreatingOrUpdatingIngredient.value = true;
    creationOrUpdateErrorMessage.value = "";
    let createdIngredient: Ingredient = {
      // This is a dummy address since the server expects a valid UUID
      // during deserialisation. It will be overwritten on the server side.
      id: "8a6e0804-2bd0-4672-b79d-d97027f9071a",
      amount: amountModel.value,
      unit: unitModel.value,
      text: textModel.value,
      recipeReference: recipeReferenceModel.value,
      recipeId: props.ingredient.recipeId,
      creationTime: new Date(),
    };
    const formData = JSON.stringify(createdIngredient);
    const config = {
      headers: {
        "content-type": "application/json",
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
      })
      .catch((error) => {
        creationOrUpdateErrorMessage.value = error;
      })
      .finally(() => {
        isCreatingOrUpdatingIngredient.value = false;
        editMode.value = false;
      });
  }
}

function updateIngredient() {
  if (!isCreatingOrUpdatingIngredient.value && props.ingredient.id) {
    isCreatingOrUpdatingIngredient.value = true;
    creationOrUpdateErrorMessage.value = "";
    let updatedIngredient: Ingredient = {
      id: props.ingredient.id,
      amount: amountModel.value,
      unit: unitModel.value,
      text: textModel.value,
      recipeReference: recipeReferenceModel.value,
      recipeId: props.ingredient.recipeId,
      creationTime: props.ingredient.creationTime,
    };
    const formData = JSON.stringify(updatedIngredient);
    const config = {
      headers: {
        "content-type": "application/json",
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
      });
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
</script>
<style scoped lang="scss"></style>
