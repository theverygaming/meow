<script setup lang="ts">
import { ref } from 'vue';

import Crud from './Crud.vue';

import { getLogsList, createLog } from '../api/brainlog';

const fields = [
  {"displayName": "Text", "key": "body"},
  {"displayName": "Type", "key": "log_type"},
  {"displayName": "Time", "key": "time"},
];

async function do_create(values) {
  await createLog(values);
}

async function do_read(page: number, items_per_page: number) {
  return await getLogsList(page, items_per_page);
}

async function do_update(id: string, values) {
  console.log(`update ${id}`, values);
}

async function do_delete(id: string) {
  console.log(`delete ${id}`);
}

</script>

<template>
  <Crud
    :do_create="do_create"
    :do_read="do_read"
    :do_update="do_update"
    :do_delete="do_delete"
    :fields="fields"
  ></Crud>
</template>
