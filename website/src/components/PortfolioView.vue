<template>

  <div>

    <h1>
      fin portfolio
    </h1>

    <div id="table-wrapper">
      <div id="table-scroll">

        <table>
          <thead>
            <tr>
              <th v-for="(key, idx) in dataW.columnsNames" v-bind:key="idx">
                {{ key }}
              </th>
            </tr>
          </thead>

          <!-- Stocks -->
          <tbody>
            <tr v-for="(entry, idx) in stocks" v-bind:key="entry + idx">
              <td v-bind:class="[key, ticker_state(entry)]" v-for="(key, jdx) in dataW.columnsKeys" v-bind:key="entry.symbol + jdx">
                {{ entry[key] }}
              </td>
            </tr>
          </tbody>

          <!-- Summary between stocks and bonds -->
          <tbody>
            <tr>
              <td class="summary">stocks</td>
              <td class="summary">-</td>
              <td class="summary">-</td>
              <td class="summary">-</td>
              <td class="summary goal_percent">{{ portState.goal_stock_percent }}</td>
              <td v-bind:class="stock_action"  class="summary actual_percent">{{ portState.actual_stock_percent }}</td>
              <td class="summary">-</td>
            </tr>
          </tbody>

          <!-- Bonds -->
          <tbody>
            <tr v-for="(entry, idx) in bonds" v-bind:key="entry + idx">
              <td v-bind:class="[key, ticker_state(entry)]" v-for="(key, jdx) in dataW.columnsKeys" v-bind:key="entry.symbol + jdx">
                {{ entry[key] }}
              </td>
            </tr>
          </tbody>


        </table>
      </div>
    </div>

  </div>

</template>

<script lang="ts">
import { PortfolioViewStart } from "../models";

export default {
  props: {
    portState: Object as () => PortfolioViewStart
  },
  data: function() {
    return {
      dataW: {
        columnsNames: [
          "Symbol",
          "Kind",
          "Fee",
          "Price",
          "Goal %",
          "Actual %",
          "Actual $"
        ],
        columnsKeys: [
          "symbol",
          "kind",
          "fee",
          "price",
          "goal_percent",
          "actual_percent",
          "actual_value"
        ]
      }
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
  computed: {
    stocks: function() {
      return this.portState.tickers.filter(tic => tic.kind === "STOCK");
    },
    bonds: function() {
      return this.portState.tickers.filter(tic => tic.kind === "BOND");
    },
    stock_action: function() {
      var diff =
        this.portState.goal_stock_percent - this.portState.actual_stock_percent;
      var deviation = this.portState.deviation_percent;
      return this.high_or_low(
        this.portState.goal_stock_percent,
        this.portState.actual_stock_percent,
        deviation
      );
    }
  }
};
</script>

<style lang="scss">
table {
  border: 1px solid #000;
  border-radius: 3px;
}

th {
  /* color: #aa704e; */
  background-color: #c9c9c9;
  /* color: rgba(255, 255, 255, 0.66); */
  /* cursor: pointer; */
  /* -webkit-user-select: none; */
  /* -moz-user-select: none; */
  /* -ms-user-select: none; */
  /* user-select: none; */
  font-size: 13px;
}

td {
  background-color: #f9f9f9;
  /* color: #0079bf; */
  font-size: 13px;
}

td {
  &.goal_percent,
  &.symbol {
    & {
      font-weight: bold;
    }
  }
}

td.summary {
  background-color: #d5d5d5;
}

td.actual_percent {
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

td {
  &.fee,
  &.actual_percent,
  &.goal_percent {
    &::after {
      content: "%";
    }
  }
}

td {
  &.price,
  &.actual_value {
    &::before {
      content: "$";
      margin: 0 -2px;
    }
  }
}

th,
td {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  min-width: 40px;
  max-width: 60px;
  padding: 10px 10px;
  text-align: center;
}

#table-wrapper {
  position: relative;
}
#table-scroll {
  width: 100%;
  overflow: auto;
  margin-top: 20px;
}
#table-wrapper table {
  /* width:100%; */
}
</style>
