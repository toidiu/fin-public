<template>
  <div>
    <h3 class="title is-3">{{ portState.name }}</h3>

    <div class="level">
      <div class="level-item has-text-centered">
        <div>
          <p class="heading">
            Total Portfolio Value
          </p>
          <p>${{ portState.total_value }}</p>
        </div>
      </div>
      <div class="level-item has-text-centered">
        <div>
          <p class="heading">Goal stock distribution</p>
          <p class="title is-5">{{ portState.goal_stock_percent }}%</p>
        </div>
      </div>
      <div class="level-item has-text-centered">
        <div>
          <p class="heading">Actual stock distribution</p>
          <p>{{ portState.actual_stock_percent }}%</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { FinPortfolioResp } from "./models";
import { Ticker } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  props: {
    portState: Object as () => FinPortfolioResp
  }
});
</script>

<style lang="scss" scoped></style>
