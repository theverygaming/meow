<script setup lang="ts">
import { ref } from 'vue';

// @ts-ignore
import Crud from './Crud.vue';

import { getLogsList, createLog, updateLog, deleteLog } from '../api/brainlog';
import type { BrainlogObj, BrainlogObjId } from '../api/brainlog';

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

const operations = {
  do_create: async (values: BrainlogObj) => {
    await createLog(values);
  },
  do_read: async (page: number, items_per_page: number): Promise<BrainlogObjId[]> => {
    return await getLogsList(page, items_per_page);
  },
  do_update: async (id: string, values: BrainlogObj) => {
    await updateLog(id, values);
  },
  do_delete: async (id: string) => {
    await deleteLog(id);
  },
};

</script>

<template>
  <Crud
    title="brainlog"
    :operations="operations"
    :fields="fields"
  ></Crud>
</template>
