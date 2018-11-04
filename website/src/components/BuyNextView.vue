<template>

  <div>

    <table>
      <tr>
        <th>BUY</th>
        <template v-for="(ticker, idx) in portState.tickers">
          <td v-bind:class="ticker['kind'].toLowerCase()">
            {{ get_shares(ticker) }}
          </td>
        </template>
        </tr>
      </tr>
    </table>

    <span>
      BUY VALUE
      ${{ buyNextState.buy_value }}
    </span>
    <button v-on:click="executeOrder" type="submit">Execute order</button>

  </div>

</template>

<script lang="ts">
import { Action, BuyNextResp, FinPortfolioResp } from "../models";
import axios from "axios";

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
    executeOrder() {
      axios
        .post("http://localhost:8000/buy", {
          user_id: 1,
          goal_id: 1,
          amount: this.buyNextState.requested_value
        })
        .then(resp => {
          console.log(resp);
          this.isLoading = false;
        })
        .catch(error => {
          console.log(error.response);
          this.isLoading = false;
        });
    }
  }
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
  &.stock {
    background-color: #eee;
  }
}
</style>
