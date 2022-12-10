<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { message } from '@tauri-apps/api/dialog';
import { exit } from '@tauri-apps/api/process';
import { useThemeStore } from "./stores/themeStore"

const status = ref(null);
const themeStore = useThemeStore()
onMounted(async () => {
  status.value = await invoke("check_php");
  if (!status) {
    await message("No php found on your system. Please install one and rerun this program.", { type: "error", title: "Oops" })
    await exit(1);
  }
})

const changeTheme = () => {
  themeStore.setTheme(themeStore.theme === "dark" ? "light" : "dark")
}

</script>

<template>
  <v-app :theme="themeStore.theme">
    <v-app-bar title="PhpSniffer">
      <v-spacer></v-spacer>

      <v-btn :prepend-icon="themeStore.theme === 'light' ? 'mdi-weather-sunny' : 'mdi-weather-night'"
        @click="changeTheme">Toggle Theme</v-btn></v-app-bar>
    <v-main>
      <router-view></router-view>
    </v-main>
  </v-app>
</template>

<style scoped>

</style>
