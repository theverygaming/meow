<script setup lang="ts">
import { ref } from 'vue';

// @ts-ignore
import Crud from './Crud.vue';

import { getQuestItemsList, createQuestItem, updateQuestItem, deleteQuestItem } from '../api/quest_item';
import type { QuestItemObj, QuestItemObjId } from '../api/quest_item';

const fields = ref([
  {
    "displayName": "Related Quest",
    "key": "quest_id",
    "type": "text",
  },
  {
    "displayName": "Attributes",
    "key": "attributes",
    "type": "text",
  },
  {
    "displayName": "Name",
    "key": "name",
    "type": "text",
  },
  {
    "displayName": "Body",
    "key": "body",
    "type": "text",
  },
]);

const operations = {
  do_create: async (values: QuestItemObj) => {
    await createQuestItem(values);
  },
  do_read: async (page: number, items_per_page: number): Promise<QuestItemObjId[]> => {
    return await getQuestItemsList(page, items_per_page);
  },
  do_update: async (id: string, values: QuestItemObj) => {
    await updateQuestItem(id, values);
  },
  do_delete: async (id: string) => {
    await deleteQuestItem(id);
  },
};

</script>

<template>
  <Crud
    title="Quest Items"
    :operations="operations"
    :fields="fields"
  ></Crud>
</template>
