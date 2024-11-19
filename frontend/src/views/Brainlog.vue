<script setup lang="ts">
import { ref } from 'vue';

import Crud from './Crud.vue';

import { getLogsList, createLog, updateLog, deleteLog } from '../api/brainlog';

const fields = ref([
  {
    "displayName": "Text",
    "key": "body",
    "type": "text",
  },
  {
    "displayName": "Type",
    "key": "log_type",
    "type": "select",
    "attrs": {
      "items": [
        // from Arya's poc_4 script :3
        "achievement",
        "problem",
        "thought",
        "decision",
        "task",
        "question",
        "feeling",
        "health",
        "observation",
      ],
    },
  },
  {
    "displayName": "Time",
    "key": "time",
    "type": "isodatetime",
  },
]);

async function do_create(values) {
  await createLog(values);
}

async function do_read(page: number, items_per_page: number) {
  return await getLogsList(page, items_per_page);
}

async function do_update(id: string, values) {
  await updateLog(id, values);
}

async function do_delete(id: string) {
  await deleteLog(id);
}

</script>

<template>
  <Crud
    title="brainlog"
    :do_create="do_create"
    :do_read="do_read"
    :do_update="do_update"
    :do_delete="do_delete"
    :fields="fields"
  ></Crud>
</template>
