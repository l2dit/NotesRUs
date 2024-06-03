import { createMemoryHistory, createRouter } from 'vue-router'
import Upload from "./views/upload.vue";
import Download from "./views/download.vue";
import Preview from "./views/preview.vue";
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
  },
  {
    path: '/preview',
    component: Preview
  }
];

const router = createRouter({
  history: createMemoryHistory(),
  routes
});

export default router;
