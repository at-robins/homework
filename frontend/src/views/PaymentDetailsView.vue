<script setup lang="ts">
import PaymentDetails from "../components/payment/PaymentDetails.vue";
defineProps({
  id: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <main>
    <PaymentDetails :id="id" class="q-ma-md" />
  </main>
</template>
