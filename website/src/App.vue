<template>

  <div id="app">
    <fin-table />
  </div>

</template>

<script>
import FinTable from "./components/FinTable.vue";

export default {
  name: "app",
  components: {
    FinTable
  }
};
</script>

<style lang="css">
  #app {
  }
</style>
