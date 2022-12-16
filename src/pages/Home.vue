<script setup>
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { useStandardStore } from "../stores/standardStore";

const loading = ref(false);
const folder = ref("");
const results = ref(null);
const standardStore = useStandardStore();

async function selectFolder() {
  loading.value = true;
  const selected = await open({
    directory: true,
    filters: [{ name: "php", extensions: ["php"] }],
  });
  if (!selected) {
    loading.value = false;
    return;
  }
  folder.value = selected.split("/").pop();
  const data = await invoke("run_sniffer", {
    folderPath: selected,
    parser: standardStore.standard,
  });
  results.value = JSON.parse(data);
  loading.value = false;
}
</script>

<template>
  <v-container class="fill-height">
    <v-row v-if="!results">
      <v-col cols="12" align="center">
        <v-btn @click="selectFolder" color="primary">Open Folder</v-btn>
      </v-col>
    </v-row>
    <v-row justify="space-between" v-if="results">
      <v-col class="text-h3">
        {{ folder }}
      </v-col>
      <v-col cols="4" align="right">
        <v-btn @click="selectFolder" color="primary">Open</v-btn>
      </v-col>
    </v-row>
    <v-container v-if="loading">
      <v-progress-linear indeterminate></v-progress-linear>
    </v-container>
    <v-container v-if="results">
      <v-row justify="space-between">
        <v-col lg="6" sm="12">
          <v-alert
            type="error"
            title="Errors"
            :text="results?.totals?.errors || 0"
          ></v-alert>
        </v-col>
        <v-col lg="6" sm="12">
          <v-alert
            title="Warnings"
            type="warning"
            :text="results?.totals?.warnings || 0"
          ></v-alert>
        </v-col>
      </v-row>
      <v-container>
        <v-expansion-panels>
          <v-expansion-panel
            v-for="(fileData, fileName) in results.files"
            :key="fileName"
          >
            <v-expansion-panel-title>
              {{ fileName.split("/").pop() }} -
              {{ fileData.messages.length }} errors
            </v-expansion-panel-title>
            <v-expansion-panel-text>
              <v-list lines="1">
                <v-list-item
                  :title="message.source"
                  v-for="(message, index) in fileData.messages"
                  :key="fileName + index"
                >
                  {{ message.message }} - {{ message.line }}:{{
                    message.column
                  }}
                </v-list-item>
              </v-list>
            </v-expansion-panel-text>
          </v-expansion-panel>
        </v-expansion-panels>
      </v-container>
    </v-container>
  </v-container>
</template>
