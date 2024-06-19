import { createWebHistory, createRouter } from 'vue-router'
import Upload from "./views/upload.vue";
import Download from "./views/download.vue";
import Preview from "./views/preview.vue";
import Delete from "./views/delete.vue"
import Browse from "./views/browse.vue"
import Landing from "./views/landing.vue"
const routes = [
  {
    path: '/upload',
    component: Upload,
    alias: '/'
  },
  {
    path: '/download',
    component: Download
  },
  {
    path: '/preview',
    component: Preview
  },
  {
    path: '/delete',
    component: Delete
  },
  {
    path: '/browse',
    component: Browse
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
