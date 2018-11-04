<template>
  <div class="outer">
    <div class="middle">
      <div class="inner">
        <pulse-loader class="_loader"
          :loading="isLoading"
          :color="loadColor"
          :size="loadSize" />

        </div>
      </div>
    </div>
</template>

<script lang="ts">
import PulseLoader from 'vue-spinner/src/PulseLoader.vue'

export default {
  components: {
    PulseLoader,
  },
  props: {
    isLoading: Boolean,
  },
  data() {
    return {
      portState: <FinPortfolioResp>{
        tickers: []
      },
      loadColor: "#234",
      loadSize: "10px",
    }
  }
</script>


<style lang="scss" scoped>
._loader {
  display: inline-block;
}
.outer {
  background: #e9e9e9;
  opacity: 0.5;
  display: table;
  position: absolute;
  height: 100%;
  width: 100%;
}
.middle {
  display: table-cell;
  vertical-align: middle;
  text-align: center;
}
.inner {
  padding: 20px 0;
  margin: 50px auto;
}
</style>
