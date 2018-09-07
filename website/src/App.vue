<template>

  <div id="app">
    <h1>{{ msg }}</h1>
    Name: <input v-model="name" type="text">
    <hello :name="name" :initialEnthusiasm="5" />

    <br/>

    <fin-table />
  </div>

</template>

<script>
  import Hello from "./components/Hello.vue";
  import FinTable from "./components/FinTable.vue";

  export default {
    name: 'app',
    components: {
      Hello,
      FinTable,
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
  }
</style>
