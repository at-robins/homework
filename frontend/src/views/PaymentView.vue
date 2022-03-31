<script setup lang="ts">
import PaymentList from "../components/PaymentList.vue";
import PaymentCreation from "../components/payment/PaymentCreation.vue";
</script>

<template>
  <main>
    <PaymentCreation class="q-ma-md" />
    <PaymentList />
  </main>
</template>
