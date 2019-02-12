<template>
  <div>
    <template>
      <nav-view :is-user-auth="true" />
    </template>

    <div class="not-found">
      page not found
    </div>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import Vue from "vue";

export default Vue.extend({
  components: {
    NavView
  }
});
</script>

<style lang="scss" scoped>
.not-found {
  margin: 20px;
}
</style>
