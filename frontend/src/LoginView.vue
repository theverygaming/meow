<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const username = ref("");
const password = ref("");

async function do_login() {
  localStorage.setItem("API_KEY", password.value);
  router.replace({ path: '/' });
}
</script>
<template>
  <v-app>
    <v-app-bar :elevation="2">
      <v-app-bar-title>Login</v-app-bar-title>
    </v-app-bar>
    <v-main>
      <p>Sign in with your username and password:</p>
      <v-form>
        <v-text-field
          outline
          label="Username"
          type="text"
          v-model="username"></v-text-field>
        <v-text-field
          outline
          hide-details
          label="Password"
          type="password"
          v-model="password"></v-text-field>
      </v-form>    
      <v-spacer></v-spacer>
      <v-btn
        color="info"
        @click="do_login"
      >
        Login
      </v-btn>
    </v-main>
  </v-app>
</template>
