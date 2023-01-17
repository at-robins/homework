<script setup lang="ts">
import { computed, ref } from "vue";
import { RouterView, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { matLanguage } from "@quasar/extras/material-icons";

const { t } = useI18n();
const leftMenuOpen = ref(false);
const router = useRouter();

const isHome = computed(() => {
  return router.currentRoute.value.name === "home";
});

function navigateToRecipes() {
  router.push({ name: "recipes" });
}

function navigateToPayments() {
  router.push({ name: "payments" });
}

function navigateToHome() {
  router.push({ name: "home" });
}
</script>

<template>
  <q-layout view="hHh lpr fFf">
    <q-header elevated class="bg-primary text-white">
      <q-toolbar>
        <q-btn
          dense
          flat
          round
          icon="menu"
          @click="leftMenuOpen = !leftMenuOpen"
        />

        <q-avatar
          @click="navigateToHome"
          class="q-ml-md"
          :class="{ 'cursor-pointer': !isHome }"
        >
          <img src="/icon_main.svg" />
        </q-avatar>
        <q-toolbar-title
          @click="navigateToHome"
          :class="{ 'cursor-pointer': !isHome }"
        >
          {{ t("app_title") }}
        </q-toolbar-title>
        <q-btn flat round dense :icon="matLanguage">
          <q-tooltip>
            {{ t("toolbar_tooltip_language") }}
          </q-tooltip>
        </q-btn>
      </q-toolbar>
    </q-header>

    <q-drawer v-model="leftMenuOpen" side="left" overlay elevated>
      <q-list separator>
        <q-item clickable v-ripple @click="navigateToRecipes">
          <q-item-section>Rezepte</q-item-section>
        </q-item>
        <q-item clickable v-ripple @click="navigateToPayments">
          <q-item-section>Zahlungen</q-item-section>
        </q-item>
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view :key="router.currentRoute.value.path" />
    </q-page-container>
  </q-layout>
</template>
