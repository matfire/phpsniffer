import { defineStore } from "pinia";
import { ref } from "vue";

export const useStandardStore = defineStore("standard", () => {
  const standard = ref("PSR12");
  const choices = [
    "MySource",
    "PEAR",
    "PSR1",
    "PSR2",
    "PSR12",
    "Squiz",
    "Zend",
  ];
  const setStandard = (s) => (standard.value = s);

  return { standard, choices, setStandard };
});
