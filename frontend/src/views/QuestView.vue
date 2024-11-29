<script setup lang="ts">
import { ref } from 'vue';

// @ts-expect-error Crud.vue is JS
import Crud from './CrudView.vue';

import { getQuestsList, createQuest, updateQuest, deleteQuest } from '../api/quest';
import type { QuestObj, QuestObjList } from '../api/quest';

const fields = ref([
  {
    "displayName": "Name",
    "key": "name",
    "type": "text",
  },
]);

const operations = {
  do_create: async (values: QuestObj) => {
    await createQuest(values);
  },
  do_read: async (page: number, items_per_page: number): Promise<QuestObjList> => {
    return await getQuestsList(page, items_per_page);
  },
  do_update: async (id: string, values: QuestObj) => {
    await updateQuest(id, values);
  },
  do_delete: async (id: string) => {
    await deleteQuest(id);
  },
};

</script>

<template>
  <Crud
    title="Quests"
    :operations="operations"
    :fields="fields"
  ></Crud>
</template>
