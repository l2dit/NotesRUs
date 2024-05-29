<template>
  <FileUpload />
  <router-view />
</template>

<script>
import FileUpload from '../components/FileUpload.vue';
export default {
  components: {
    FileUpload
  }
};
</script>

<style></style>
