<template>
  <div id="buy-next">
    <div>Buy value: ${{ buyNextState.buy_value }}</div>
    <table class="table">
      <tr>
        <th>
          <button
            class="button is-primary is-small"
            v-on:click="buyNextEvent"
            type="submit"
          >
            BUY
          </button>
        </th>
        <template v-for="(ticker, idx) in portState.tickers">
          <td v-bind:class="ticker['kind'].toLowerCase()" :key="idx">
            {{ get_shares(ticker) }}
          </td>
        </template>
      </tr>
    </table>
  </div>
</template>

<script lang="ts">
import { BuyNextResp, FinPortfolioResp } from "./models";
import { Action, Ticker } from "../../data/models";

export default {
  props: {
    portState: Object as () => FinPortfolioResp,
    buyNextState: Object as () => BuyNextResp
  },
  data: function() {
    return {};
  },
  methods: {
    get_shares: function(ticker: Ticker) {
      var action = this.buyNextState.action_summary[ticker.id];
      if (action != undefined) {
        return action["shares"];
      }
    },
    buyNextEvent: function() {
      this.$emit("buy-next-event", this.buyNextState.actions);
    }
  }
};
</script>

<style lang="scss" scoped>
#buy-next {
  margin: 10px 0;
}
table {
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

.table th {
  border: none;
}

td {
  background-color: #f9f9f9;
  &.stock {
    background-color: #eee;
  }
}
</style>
