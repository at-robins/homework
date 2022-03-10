<template>
  <div>
    <div class="bg-primary text-white">
      <q-toolbar>
        <q-btn flat round dense icon="menu" class="q-mr-sm">
          <q-menu auto-close>
            <q-list style="min-width: 100px">
              <q-item clickable>
                <q-item-section>New tab</q-item-section>
              </q-item>
              <q-item clickable>
                <q-item-section>New incognito tab</q-item-section>
              </q-item>
              <q-separator />
              <q-item clickable>
                <q-item-section>Recent tabs</q-item-section>
              </q-item>
              <q-item clickable>
                <q-item-section>History</q-item-section>
              </q-item>
              <q-item clickable>
                <q-item-section>Downloads</q-item-section>
              </q-item>
              <q-separator />
              <q-item clickable>
                <q-item-section>Settings</q-item-section>
              </q-item>
              <q-separator />
              <q-item clickable>
                <q-item-section>Help &amp; Feedback</q-item-section>
              </q-item>
            </q-list>
          </q-menu>
        </q-btn>
        <q-space />
        <q-btn flat round dense icon="search" class="q-mr-xs" />
        <q-btn flat round dense icon="group_add" />
      </q-toolbar>
      <q-toolbar inset>
        <q-toolbar-title>Heimarbeit</q-toolbar-title>
      </q-toolbar>
    </div>
  </div>
</template>
