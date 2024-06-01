import { createMemoryHistory, createRouter } from 'vue-router'
import Upload from "./views/upload.vue";
import Download from "./views/download.vue";
import Preview from "./views/preview.vue";
import Delete from "./views/delete.vue"
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
  },
  {
    path: '/delete',
    component: Delete
  }

];

const router = createRouter({
  history: createMemoryHistory(),
  routes
});

export default router;
