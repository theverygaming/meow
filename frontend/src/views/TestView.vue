<script setup lang="ts">
</script>
<template>
    <h1>Hello world!</h1>
    <v-combobox
        label="Combobox"
        :items="['meow', 'dfsdf', ':3']"
    ></v-combobox>
    <v-alert text="meow" type="success"></v-alert>
</template>
