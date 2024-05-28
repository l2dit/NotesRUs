<template>
    <div id="app">
      <FileUpload />
      <router-view />
    </div>
  </template>
  
  <script>
  import FileUpload from '../components/FileUpload.vue';
  export default {
    components: {
      FileUpload
    }
  };
  </script>
  
  <style>
  /* put syyles here 0_0*/
  </style>
 
