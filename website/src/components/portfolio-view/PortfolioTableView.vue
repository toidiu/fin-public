<template>
  <div>
    <table class="table">
      <tr v-for="([colName, colKey], idx) in columns" :key="idx">
        <th>{{ colName }}</th>
        <template v-for="(ticker, tidx) in portState.tickers">
          <td
            v-bind:class="[
              colKey,
              ticker_state(ticker),
              ticker['kind'].toLowerCase()
            ]"
            :key="tidx"
          >
            {{ ticker[colKey] }}
          </td>
        </template>
      </tr>
    </table>
  </div>
</template>

<script lang="ts">
import { FinPortfolioResp } from "./models";
import { Ticker } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  props: {
    portState: Object as () => FinPortfolioResp
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
      var deviation = this.portState.deviation_percent;
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
    }
  }
});
</script>

<style lang="scss" scoped>
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
