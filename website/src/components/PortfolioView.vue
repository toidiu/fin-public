<template>

  <div>

    <table>
      <tr v-for="([colName, colKey], idx) in columns">
          <th>{{ colName }}</th>
          <template v-for="(ticker, idx) in portState.tickers">
            <td v-bind:class="[colKey, ticker_state(ticker), ticker['kind'].toLowerCase()]">
              {{ ticker[colKey] }}
            </td>
          </template>
        </tr>

        </td>
    </table>

    <span>
      TOTAL
      ${{ portState.total_value }}
    </span>

  </div>

</template>

<script lang="ts">
import { FinPortfolioResp } from "../models";

export default {
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
        ["Goal %", "goal_percent"],
        ["Actual %", "actual_percent"],
        ["Actual $", "actual_value"]
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
    high_or_low: function(goal, actual, deviation) {
      var diff = goal - actual;
      if (diff > 0 && Math.abs(diff) > deviation) {
        return "high";
      } else if (diff < 0 && Math.abs(diff) > deviation) {
        return "low";
      } else {
        return "balance";
      }
    }
  },
  computed: {}
};
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
