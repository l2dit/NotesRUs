<template>
  <Sidebar />
  <router-view />
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
#app > body {
  background-color: #F4F5FA;
}

div {
  justify-content: center;
  align-items: center;
}

div#app {
  height: 95vh;
  background-color: #FFFFFF;
  margin: 1% 1% 0% 5%;
  border-radius: 0.8rem;
  border: 1px solid rgb(99, 99, 99);
  box-shadow: 0px 0px 0.2rem black;
  overflow-y: auto;
}
</style>
