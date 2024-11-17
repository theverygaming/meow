<script setup lang="ts">
import { ref, watch, watchEffect } from 'vue';

const model = defineModel();

const props = defineProps<{
  label: string;
}>();

const time = ref("");

watch(time, async (newtime) => {
  const parsed = new Date(Date.parse(newtime));
  const iso = parsed.toISOString();
  console.log(newtime, parsed, iso);
  model.value = iso;
});

watchEffect(() => {
  let dt = new Date();
  const parsed = Date.parse(model.value);
  if (!Number.isNaN(parsed)) {
    dt = new Date(parsed);
  }

  // https://stackoverflow.com/a/61082536
  dt.setMinutes(dt.getMinutes() - dt.getTimezoneOffset());
  time.value = dt.toISOString().slice(0,16);
});

</script>
<template>
    <v-text-field
      :label="props.label"  
      v-model="time"
      type="datetime-local"
    />
</template>
