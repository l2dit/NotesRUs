import { createMemoryHistory, createRouter } from 'vue-router'
import Upload from "./views/upload.vue";
import Download from "./views/download.vue";
const routes = [
  {
    path: '/',
    component: Upload
  },
  {
    path: '/upload',
    component: Upload
  },
  {
    path: '/download',
    component: Download
  }
];

const router = createRouter({
  history: createMemoryHistory(),
  routes
});

export default router;
