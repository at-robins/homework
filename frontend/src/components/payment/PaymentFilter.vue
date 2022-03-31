<template>
  <div class="row wrap q-gutter-md q-pa-md">
    <input-filter
      v-model="nameModel"
      :options="allNames"
      label="Bezug filtern"
      multiple
      class="col-2"
    />
    <input-filter
      v-model="tagModel"
      :options="allTags"
      label="Schlagwörter filtern"
      multiple
      class="col-2"
    />
    <input-filter
      v-model="paidModel"
      :options="allPaid"
      label="Bezahler filtern"
      multiple
      class="col-2"
    />
    <input-filter
      v-model="involvedModel"
      :options="allInvolved"
      label="Beteiligte filtern"
      multiple
      class="col-2"
    />
  </div>
</template>

<script setup lang="ts">
import type { Payment } from "@/scripts/types";
import { computed, ref, watch, type Ref } from "vue";
import InputFilter from "../general/InputFilter.vue";

const props = defineProps({
  payments: { type: Array as () => Array<Payment>, required: true },
});

const emit = defineEmits<{
  (event: "updatedFilter", payments: Array<Payment>): void;
}>();

const tagModel: Ref<Array<string> | null> = ref(null);
const paidModel: Ref<Array<string> | null> = ref(null);
const involvedModel: Ref<Array<string> | null> = ref(null);
const nameModel: Ref<Array<string> | null> = ref(null);

const allTags = computed(() => {
  const tags = props.payments.flatMap((payment) => payment.tags);
  return [...new Set<string>(tags)].sort();
});

const allNames = computed(() => {
  const names = props.payments.flatMap((payment) => payment.target);
  return [...new Set<string>(names)];
});

const allInvolved = computed(() => {
  const involved = props.payments.flatMap((payment) =>
    Object.keys(payment.involved)
  );
  return [...new Set<string>(involved)].sort();
});

const allPaid = computed(() => {
  const paid = props.payments.flatMap((payment) => Object.keys(payment.paid));
  return [...new Set<string>(paid)].sort();
});

watch([nameModel, tagModel, paidModel, involvedModel], () => {
  emit("updatedFilter", matchingPayments());
});

function matchingPayments(): Payment[] {
  let filteredPayments = props.payments;
  const filteredNames = nameModel.value;
  if (filteredNames && filteredNames.length > 0) {
    filteredPayments = filteredPayments.filter((payment) =>
      filteredNames.every((name) => payment.target.includes(name))
    );
  }
  const filteredTags = tagModel.value;
  if (filteredTags && filteredTags.length > 0) {
    filteredPayments = filteredPayments.filter((payment) =>
      filteredTags.every((tag) => payment.tags.includes(tag))
    );
  }
  const filteredPaid = paidModel.value;
  if (filteredPaid && filteredPaid.length > 0) {
    filteredPayments = filteredPayments.filter((payment) =>
      filteredPaid.every((paid) => Object.keys(payment.paid).includes(paid))
    );
  }
  const filteredInvolved = paidModel.value;
  if (filteredInvolved && filteredInvolved.length > 0) {
    filteredPayments = filteredPayments.filter((payment) =>
      filteredInvolved.every((involved) =>
        Object.keys(payment.involved).includes(involved)
      )
    );
  }
  return filteredPayments;
}
</script>
<style scoped lang="scss"></style>
