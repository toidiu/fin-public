<template>
  <div>
    <div v-for="(goal, idx) in portListState" :key="idx">
      <div>
        {{ goal.name }}
        {{ goal.goal_stock_percent }}%
        {{ goal.description }}
        <div v-for="(tickers_goal, idx) in goal.tickers_goal" :key="idx">
          {{ tickers_goal }}
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { PortfolioGoalList, Ticker } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  props: {
    portListState: Array as () => PortfolioGoalList[]
  },
  data: function() {
    return {
      columns: [
        ["Symbol", "symbol"],
        ["Id", "id"],
        ["Kind", "kind"],
        ["Fee", "fee"],
        ["Price", "price"],
        ["Shares", "actual_shares"],
        ["Actual $", "actual_value"],
        ["Goal %", "goal_percent"],
        ["Actual %", "actual_percent"]
      ]
    };
  },
  methods: {
    ticker_state: function(ticker: Ticker) {
      var deviation = this.portListState.deviation_percent;
      return this.high_or_low(
        ticker.goal_percent,
        ticker.actual_percent,
        deviation
      );
    },
    high_or_low: function(goal: number, actual: number, deviation: number) {
      var diff = goal - actual;
      if (diff > 0 && Math.abs(diff) > deviation) {
        return "high";
      } else if (diff < 0 && Math.abs(diff) > deviation) {
        return "low";
      } else {
        return "balance";
      }
    },
    calcInvestmentEvent: function(submitEvent) {
      this.$emit(
        "calc-investment-event",
        submitEvent.target.elements.amount.value
      );
    }
  }
});
</script>

<style lang="scss" scoped>
table {
  border: 1px solid #000;
  border-radius: 3px;
  margin: 10px 0px;
}

th,
td {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  padding: 10px 10px;
  text-align: center;
  font-size: 13px;
}

th {
  background-color: #c9c9c9;
}

td {
  background-color: #f9f9f9;

  &.goal_percent,
  &.symbol {
    font-weight: bold;
  }
  &.summary {
    background-color: #d5d5d5;
  }
  &.stock {
    background-color: #eee;
  }
  &.actual_percent {
    &.balance {
      color: black;
    }
    &.low {
      color: green;
    }
    &.high {
      color: red;
    }
  }

  // percent
  &.fee,
  &.actual_percent,
  &.goal_percent {
    &::after {
      content: "%";
    }
  }

  // dollar
  &.price,
  &.actual_value,
  &.total_value {
    &::before {
      content: "$";
    }
  }
}
</style>
