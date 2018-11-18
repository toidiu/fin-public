<template>
  <center-view class="center">
    <pulse-loader
      class="_loader"
      :loading="isLoading"
      :color="loadColor"
      :size="loadSize"
    />
  </center-view>
</template>

<script lang="ts">
import PulseLoader from "vue-spinner/src/PulseLoader.vue";
import CenterView from "./CenterView.vue";
import Vue from "vue";

export default Vue.extend({
  components: {
    PulseLoader,
    CenterView
  },
  props: {
    isLoading: Boolean
  },
  data() {
    return {
      loadColor: "#234",
      loadSize: "10px"
    };
  }
});
</script>

<style lang="scss" scoped>
._loader {
  display: inline-block;
  z-index: 2;
  opacity: 1;
}
.center {
  background: #e9e9e9;
  opacity: 0.5;
  position: absolute;
}
</style>
