<script setup lang="ts">
import { computed, ref, watch, type Ref } from "vue";
import { RouterView, useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { matLanguage, matMenu } from "@quasar/extras/material-icons";
import type { QPopupProxy } from "quasar";
import "/node_modules/flag-icons/css/flag-icons.min.css";

const { t, locale } = useI18n({ useScope: "global" });
const leftMenuOpen = ref(false);
const languagePopupReference: Ref<QPopupProxy | null> = ref(null);
const router = useRouter();

const selectedLanguage = ref(locale.value);
const languages = [
  {
    label: "language_de",
    value: "de",
    icon: "de",
  },
  {
    label: "language_en",
    value: "en",
    icon: "gb",
  },
];

watch(selectedLanguage, (newValue) => {
  changeLanguage(newValue);
  languagePopupReference.value?.hide();
});

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

function changeLanguage(language: Record<string, unknown>) {
  locale.value = language;
  localStorage.setItem("app_locale", JSON.stringify(language));
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
          :icon="matMenu"
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
            {{ t("toolbar_language_tooltip") }}
          </q-tooltip>
          <q-popup-proxy ref="languagePopupReference">
            <q-card class="card-language">
              <q-card-section>
                <div class="text-h6">{{ t("toolbar_language_title") }}</div>
              </q-card-section>

              <q-separator />
              <div class="q-pa-lg">
                <q-option-group
                  v-model="selectedLanguage"
                  :options="languages"
                  color="primary"
                >
                  <template v-slot:label="opt">
                    <div class="row items-center">
                      <span class="q-mr-sm fi" :class="'fi-' + opt.icon" />
                      <span>{{ t(opt.label) }}</span>
                    </div>
                  </template>
                </q-option-group>
              </div>
            </q-card>
          </q-popup-proxy>
        </q-btn>
      </q-toolbar>
    </q-header>

    <q-drawer v-model="leftMenuOpen" side="left" overlay elevated>
      <q-list separator>
        <q-item clickable v-ripple @click="navigateToRecipes">
          <q-item-section>{{ t("toolbar_link_recipes") }}</q-item-section>
        </q-item>
        <q-item clickable v-ripple @click="navigateToPayments">
          <q-item-section>{{ t("toolbar_link_payments") }}</q-item-section>
        </q-item>
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view :key="router.currentRoute.value.path" />
    </q-page-container>
  </q-layout>
</template>

<style lang="css" scoped>
.card-language {
  width: max-content;
}
</style>
