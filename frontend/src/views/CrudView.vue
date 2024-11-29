<!-- eslint-disable vue/block-lang -->
<script setup>
// vue.js TS does not support generics in props and tbh i just fucking gave up on using TS here
import { ref, nextTick, computed } from 'vue';

import ISODateTimePicker from '../components/ISODateTimePicker.vue';

const props = defineProps({
  title: String,
  operations: Object,
  fields: Array,
});

const itemsPerPage = ref(5);
const headers = computed(() => {
  const x = [];
  for (const [i, field] of props.fields.entries()) {
    x.push({
      title: field.displayName,
      key: field.key,
      sortable: false,
      align: i == 0 ? "start" : "end",
    });
  }
  x.push({
    title: "Actions",
    key: "actions",
    sortable: false,
    align: "end",
  });
  return x;
});
const search = ref("");
const serverItems = ref([]);
const totalItems = ref(0);
const loading = ref(true);
const prevPage = ref(1);
const previtemsPerPage = ref(1);
function loadItems ({ page, itemsPerPage, sortBy }) {
    console.log(`loadItems ${page}, ${itemsPerPage}`)
    loading.value = true;
    props.operations.do_read(page, itemsPerPage).then(({ items, total_items }) => {
        serverItems.value = items;
        totalItems.value = total_items;
        loading.value = false;
    });
    prevPage.value = page;
    previtemsPerPage.value = itemsPerPage;
}

// https://github.com/vuetifyjs/vuetify/blob/master/packages/docs/src/examples/v-data-table/misc-crud.vue

// Create & Edit logic

const dialog = ref(false);
const editedItem = ref({});
const editedId = ref(-1);

function close () {
    dialog.value = false;
    nextTick(() => {
      editedItem.value = Object.assign({}, {});
      editedId.value = -1;
    });
}

function save () {
    if (editedId.value != -1) {
        props.operations.do_update(editedId.value, editedItem.value).then(() => {
          loadItems({page: prevPage.value, itemsPerPage: previtemsPerPage.value, sortBy: ""})
        });
    } else {
        props.operations.do_create(editedItem.value).then(() => {
          loadItems({page: prevPage.value, itemsPerPage: previtemsPerPage.value, sortBy: ""})
        });
    }
    close();
}

function getEditItem(item) {
  if (Object.hasOwn(item, "__item_edit_values")) {
    return item["__item_edit_values"];
  }
  return item;
}

function editItem (item) {
    editedId.value = item.id;
    editedItem.value = Object.assign({}, getEditItem(item));
    dialog.value = true;
}

// Delete logic
const dialogDelete = ref(false);
function deleteItem (item) {
    editedId.value = item.id;
    editedItem.value = Object.assign({}, getEditItem(item));
    dialogDelete.value = true;
}

function deleteItemConfirm () {
    props.operations.do_delete(editedId.value).then(() => {
      loadItems({page: prevPage.value, itemsPerPage: previtemsPerPage.value, sortBy: ""})
    });
    closeDelete();
}

function closeDelete () {
    dialogDelete.value = false;
    nextTick(() => {
      editedItem.value = Object.assign({});
      editedId.value = -1;
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
  >
    <template v-slot:top>
      <v-toolbar
        flat
      >
        <v-toolbar-title>{{ props.title }}</v-toolbar-title>
        <v-spacer/>
        <v-dialog v-model="dialog" max-width="500px">
          <template v-slot:activator="{ props }">
            <v-btn v-bind="props">Create</v-btn>
          </template>
          <v-card>
            <v-card-title>
              <span class="text-h5">Edit/Create</span>
            </v-card-title>

            <v-card-text>
              <v-container>
                <v-row v-for="field in props.fields">
                  <v-col>
                    <v-text-field 
                      v-if="field.type == 'text'"
                      :label="field.displayName"
                      v-model="editedItem[field.key]"
                    ></v-text-field>
                    <ISODateTimePicker
                      v-if="field.type == 'isodatetime'"
                      :label="field.displayName"
                      v-model="editedItem[field.key]"
                    ></ISODateTimePicker>
                    <v-select
                      v-if="field.type == 'select'"
                      :label="field.displayName"
                      :items="field.attrs.items"
                      v-model="editedItem[field.key]"
                    ></v-select>
                    <v-select
                      v-if="field.type == 'relationalmany2one'"
                      :label="field.displayName"
                      :items="field.attrs.getAllItems()"
                      v-model="editedItem[field.key]"
                    ></v-select>
                  </v-col>
                </v-row>
              </v-container>
            </v-card-text>

            <v-card-actions>
              <v-spacer></v-spacer>
              <v-btn
                color="blue-darken-1"
                variant="text"
                @click="close"
              >
                Cancel
              </v-btn>
              <v-btn
                color="blue-darken-1"
                variant="text"
                @click="save"
              >
                Save
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>
        <v-dialog v-model="dialogDelete" max-width="500px">
          <v-card>
            <v-card-title class="text-h5">Are you sure you want to delete this item?</v-card-title>
            <v-card-actions>
              <v-spacer></v-spacer>
              <v-btn color="blue-darken-1" variant="text" @click="closeDelete">Cancel</v-btn>
              <v-btn color="blue-darken-1" variant="text" @click="deleteItemConfirm">OK</v-btn>
              <v-spacer></v-spacer>
            </v-card-actions>
          </v-card>
        </v-dialog>
      </v-toolbar>
    </template>
    <template v-slot:item.actions="{ item }">
      <v-btn size="small" @click="editItem(item)">
        edit
      </v-btn>
      <v-btn size="small" @click="deleteItem(item)">
        delete
      </v-btn>
    </template>
  </v-data-table-server>
</template>
