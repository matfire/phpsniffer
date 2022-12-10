import {createApp} from 'vue';
import './style.css';
import App from './App.vue';
import {vuetify} from './plugins/vuetify';
import {createPinia} from 'pinia';
import {createRouter, createWebHistory} from 'vue-router';
const app = createApp (App);

const pinia = createPinia ();
app.use (pinia);
app.use (vuetify);

import Home from './pages/Home.vue';
const router = createRouter ({
  history: createWebHistory (),
  routes: [{name: 'Home', component: Home}],
});
app.use (router);
app.mount ('#app');
