<script setup lang="ts">
import { ref } from 'vue';

const drawer = ref(false);
</script>

<template>
  <v-app>
    <v-app-bar :elevation="2">
      <template v-slot:prepend>
        <v-app-bar-nav-icon @click.stop="drawer = !drawer"/>
      </template>
      <v-app-bar-title>Meow</v-app-bar-title>
    </v-app-bar>
    <v-navigation-drawer v-model="drawer">
      <v-list-item link to="/test" title="Silly" subtitle="test"/>
      <v-list-item link to="/test2" title="Silly2" subtitle="test2"/>
      <v-list-item link to="/brainlog" title="Brainlog" subtitle="Log for thoughts n stuff"/>
      <v-list-item link to="/quest" title="Quests" subtitle="Quests"/>
      <v-list-item link to="/quest/items" title="Quest Items" subtitle="Quest Items"/>
    </v-navigation-drawer>
    <v-main>
      <RouterView />
    </v-main>
  </v-app>
</template>
