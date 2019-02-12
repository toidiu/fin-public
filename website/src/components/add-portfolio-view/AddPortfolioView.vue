<template>
  <div class="container">
    <form @submit.prevent="addPortfolio">
      <div class="section">
        <div class="control">
          <label class="label">Choose a Portfolio</label>

          <!-- radio input that shows all the portfolio goals -->
          <template v-for="(port, idx) in portGoalState">
            <label class="radio" v-bind:key="port.id + idx">
              <input name="goalPortId" type="radio" :value="port.id" required /
              checked >
              {{ port.name }}
              <span class="help">{{ port.description }}</span>
              <!-- <div v-for="(tg, idx) in port.tickers_goal"> -->
              <!-- {{ tg.symbol }} -->
              <!-- {{ tg.goal_percent }} -->
              <!-- </div> -->
            </label>
          </template>
        </div>
      </div>

      <div class="section">
        <div class="field">
          <label class="label">Choose Stock %</label>
          <div class="control">
            <input
              class="input text-input"
              type="number"
              min="0"
              max="100"
              placeholder="50"
              name="stockPer"
              value="34"
            />
          </div>
          <p class="help">
            what percent of the portfolio should be invested in stocks
          </p>
        </div>
      </div>

      <div class="section">
        <div class="field is-grouped">
          <div class="control">
            <button class="button is-primary">Create</button>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { AddData } from "./models";
import Vue from "vue";

export default Vue.extend({
  props: {
    portGoalState: Array as () => Object[]
  },
  methods: {
    addPortfolio: function(submitEvent) {
      let data = new Object() as AddData;
      data.goalPortId = parseInt(submitEvent.target.elements.goalPortId.value);
      data.stockPercent = parseInt(submitEvent.target.elements.stockPer.value);
      this.$emit("add-portfolio-event", data);
    },
    cancelAddPortfolio: function() {
      this.$emit("cancel-add-portfolio-event");
    }
  }
});
</script>

<style lang="scss" scoped>
.text-input {
  max-width: 10em;
}
.center-img-in-container {
  display: flex;
  height: 100%;
  width: 64px;
}
.col {
  margin-left: 0.75em;
  margin-right: 0.75em;
}
.col-item {
  cursor: pointer;
  background-color: #1bbde7;
  margin: 0.5em;
  border-radius: 2%;
  border: 1px solid #fff;

  &:hover {
    border-color: #000;
    background-color: #17aad1;
  }
  &:active {
    border-color: #000;
    background-color: #1293b6;
  }
}
</style>
