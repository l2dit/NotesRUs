<template>
    <Sidebar />
    <div id="app">
      <router-view />
    </div>
  </template>
  
  <script>
  import Sidebar from './components/Sidebar.vue';
  export default {
    components: {
      Sidebar,
    }
  };
  </script>
  
  <style>
  div {
    justify-content: center;
    align-items: center;
  }
    div#app {
      height: 95vh;
      background-color: #777777;
      margin: 1% 1% 0% 5%;
      border-radius: 1rem;
    }
   
    
    * {
        
    }
  </style>
  
