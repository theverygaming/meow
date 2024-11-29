<script setup lang="ts">
import { ref, reactive } from 'vue';

// @ts-expect-error Crud.vue is JS
import Crud from './CrudView.vue';

import { getQuestItemsList, createQuestItem, updateQuestItem, deleteQuestItem } from '../api/quest_item';
import type { QuestItemObj, QuestItemObjList } from '../api/quest_item';
import { getQuestsList } from '../api/quest';

const fields = ref([
  {
    "displayName": "Related Quest",
    "key": "quest_id",
    "type": "relationalmany2one",
    "attrs": {
      "getAllItems": function() {
        const data: { title: string; value: string; }[] = reactive([]);
        getQuestsList(1, -1).then((r) => {
          for (const item of r.items) {
            data.push({title: item.name, value: item.id});
          }
        });
        return data;
      },
    },
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
  do_read: async (page: number, items_per_page: number): Promise<QuestItemObjList> => {
    const data = await getQuestItemsList(page, items_per_page);
    for (let i = 0; i < data.items.length; i++) {
      // @ts-expect-error me skill issue
      data.items[i]["__item_edit_values"] = JSON.parse(JSON.stringify(data.items[i])); // deep copy
      const quest_search = await getQuestsList(1, 1, [["id", "=", data.items[i]["quest_id"]]]);
      if (quest_search.items.length != 0) {
        data.items[i]["quest_id"] = quest_search.items[0].name;
      } else {
        data.items[i]["quest_id"] = `Quest with ID ${data.items[i]["quest_id"]} (unknown name!)`;
      }
    }
    return data;
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
