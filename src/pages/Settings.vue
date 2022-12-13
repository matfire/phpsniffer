<script setup>
import { useStandardStore } from "../stores/standardStore";
const standardStore = useStandardStore();
</script>

<template>
  <v-container>
    <v-row>
      <v-col>
        <v-select
          v-model="standardStore.standard"
          label="Coding Standard"
          :items="standardStore.choices"
        ></v-select>
      </v-col>
    </v-row>
  </v-container>
</template>
