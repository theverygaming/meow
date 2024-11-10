<script setup lang="ts">
import { ref } from 'vue';

const drawer = ref(false);
</script>

<template>
  <header>
    
  </header>

  <main>
    <v-app>
      <v-app-bar :elevation="2">
        <template v-slot:prepend>
          <v-app-bar-nav-icon @click.stop="drawer = !drawer"/>
        </template>

        <v-app-bar-title>Meow</v-app-bar-title>
      </v-app-bar>
      <v-navigation-drawer v-model="drawer">
        <v-list-item link title="Silly" subtitle="test"></v-list-item>
      </v-navigation-drawer>
      <v-combobox
        label="Combobox"
        :items="['meow', 'dfsdf', ':3']"
      ></v-combobox>
      <v-alert text="awawa" type="success"></v-alert>
    </v-app>
    <img alt="xenia" src="./assets/xenia_drawing1.svg" width="20%" />
  </main>
</template>
