import Vue from "vue";

<template>
  <div id="app">
    <h1>{{ msg }}</h1>
     Name: <input v-model="name" type="text">
     <hello-component :name="name" :initialEnthusiasm="5" />
  </div>
</template>

<script>
import HelloComponent from "./components/Hello.vue";

export default {
  name: 'app',
  components: {
    HelloComponent
  },
  data () {
    return {
      msg: 'Welcome to Your Vue.js App!',
      name: "World"
    }
  }
}
</script>

<style lang="css">
  #app {
    color: #56b983;
  }
</style>
