<template>
  <div class="errors">
    <ul>
      <li v-for="(error, idx) in errors" :key="idx">{{ error }}</li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  props: {
    errors: Array
  }
});
</script>

<style lang="scss" scoped>
.errors {
  color: #b74b4b;
  max-height: 100px;
  overflow: auto;
  display: block;
  position: relative;
  ul {
    list-style: disc;
    padding: 10px 20px;
  }
}
</style>
