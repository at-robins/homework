<template>
  <div class="q-pa-md">Hello world!</div>
  <div>{{ t("app_title") }}</div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
const { t } = useI18n();
</script>
