import {defineStore} from 'pinia';
import {ref} from 'vue';

export const useThemeStore = defineStore ('theme', () => {
  const theme = ref ('dark');
  const setTheme = t => (theme.value = t);
  return {theme, setTheme};
});
