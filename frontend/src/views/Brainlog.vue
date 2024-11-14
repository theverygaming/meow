<script setup lang="ts">
import { ref, nextTick } from 'vue';

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
    { title: 'Actions', key: 'actions', align: 'end' },
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

// TODO: simplify
// TODO: common CRUD view

// https://github.com/vuetifyjs/vuetify/blob/master/packages/docs/src/examples/v-data-table/misc-crud.vue

// Create & Edit logic

const dialog = ref(false);
const editedItem = ref({
    name: "",
    calories: 0,
    fat: 0,
    carbs: 0,
    protein: 0,
});

function close () {
    dialog.value = false;
    nextTick(() => {
      editedItem.value = Object.assign({}, defaultItem.value)
      editedIndex.value = -1
    });
}

function save () {
    if (editedIndex.value > -1) {
        Object.assign(desserts.value[editedIndex.value], editedItem.value);
    } else {
        desserts.value.push(editedItem.value);
    }
    close();
}

// Delete logic
const dialogDelete = ref(false);
function deleteItem (item) {
    editedIndex.value = desserts.value.indexOf(item);
    editedItem.value = Object.assign({}, item);
    dialogDelete.value = true;
}

function deleteItemConfirm () {
    desserts.value.splice(editedIndex.value, 1);
    closeDelete();
}

function closeDelete () {
    dialogDelete.value = false;
    nextTick(() => {
      editedItem.value = Object.assign({});
      editedIndex.value = -1;
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
        <v-toolbar-title>mrrrp</v-toolbar-title>
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
                <v-row>
                  <v-col
                    cols="12"
                    md="4"
                    sm="6"
                  >
                    <v-text-field
                      v-model="editedItem.name"
                      label="Dessert name"
                    ></v-text-field>
                  </v-col>
                  <v-col
                    cols="12"
                    md="4"
                    sm="6"
                  >
                    <v-text-field
                      v-model="editedItem.calories"
                      label="Calories"
                    ></v-text-field>
                  </v-col>
                  <v-col
                    cols="12"
                    md="4"
                    sm="6"
                  >
                    <v-text-field
                      v-model="editedItem.fat"
                      label="Fat (g)"
                    ></v-text-field>
                  </v-col>
                  <v-col
                    cols="12"
                    md="4"
                    sm="6"
                  >
                    <v-text-field
                      v-model="editedItem.carbs"
                      label="Carbs (g)"
                    ></v-text-field>
                  </v-col>
                  <v-col
                    cols="12"
                    md="4"
                    sm="6"
                  >
                    <v-text-field
                      v-model="editedItem.protein"
                      label="Protein (g)"
                    ></v-text-field>
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
