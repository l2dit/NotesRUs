
<template>
    <div id="app">
      <Sidebar />
      <FileUpload />
    </div>
  </template>
  
  <script>
  import FileUpload from './components/FileUpload.vue';
  import Sidebar from './components/Sidebar.vue';
  export default {
    components: {
      Sidebar,
      FileUpload
    }
  };
  </script>
  
  <style>
  /* put syyles here 0_0*/
  </style>
  