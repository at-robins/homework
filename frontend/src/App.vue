<script setup lang="ts">
import { ref } from "vue";
import { RouterView, useRouter } from "vue-router";

const leftMenuOpen = ref(false);
const router = useRouter();

function navigateToRecipes() {
  router.push({ name: "recipes" });
}

function navigateToPayments() {
  router.push({ name: "payments" });
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

        <q-toolbar-title>
          <q-avatar>
            <img src="/icon_main.svg" />
          </q-avatar>
          Heimarbeit
        </q-toolbar-title>
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
