<script setup lang="ts">
import { ref } from 'vue';

import { getLogsList } from '../api/brainlog';

const itemsPerPage = ref(5);
const headers = ref([
    {
        title: 'Text',
        align: 'start',
        sortable: false,
        key: 'body',
    },
    { title: 'Type', key: 'log_type', align: 'end' },
    { title: 'Time', key: 'time', align: 'end' },
]);
const search = ref('');
const serverItems = ref([]);
const totalItems = ref(0);
const loading = ref(true);
function loadItems ({ page, itemsPerPage, sortBy }) {
    loading.value = true;
    getLogsList(page, itemsPerPage).then(({ items, total_items }) => {
        serverItems.value = items;
        totalItems.value = total_items;
        loading.value = false;
    });
}

</script>
<template>
    <v-data-table-server
        v-model:items-per-page="itemsPerPage"
        :headers="headers"
        :items="serverItems"
        :items-length="totalItems"
        :loading="loading"
        :search="search"
        item-value="name"
        @update:options="loadItems"
  ></v-data-table-server>
</template>
