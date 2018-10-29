<template>
  <div>


    <template v-if="portState.tickers.length" >
      <h1>
        {{ portState.name }}
      </h1>

      <form @submit.prevent="fetchBuyNext">
        <div>How much do you want to invest?</div>
          <input type="number" name="amount">
          <button type="submit">Submit</button>
      </form>
      <div>
        <p v-if="errors.length">
           <ul>
             <li v-for="error in errors">{{ error }}</li>
           </ul>
        </p>
      </div>
    </template>

    <div id="table-wrapper">
      <div id="table-scroll">

        <template v-if="portState.tickers.length" >
          <portfolio-view :portState="portState" />
        </template>

        <buy-next-view v-if="buyNext.actions.length"
          :portState="portState" :buyNextState="buyNext" />

        </div>
      </div>
    </div>

</template>

<script lang="ts">
import PortfolioView from "./PortfolioView.vue";
import BuyNextView from "./BuyNextView.vue";
import { BuyNextResp, Ticker, FinPortfolioResp } from "../models";
import axios from "axios";

export default {
  components: {
    PortfolioView,
    BuyNextView
  },
  data() {
    return {
      portState: <FinPortfolioResp>{
        tickers: []
      },
      buyNext: <BuyNextResp>{
        actions: [],
        buy_value: 0,
      },
      errors: [],
    };
  },
  created() {
    this.fetchPortfolio();
  },
  methods: {
    fetchPortfolio() {
      /* get portfolio */
      axios
        .get("http://localhost:8000/portfolio?user_id=1&goal_id=1")
        .then(resp => (this.portState = resp.data));
    }
    fetchBuyNext(submitEvent) {
      var amount = submitEvent.target.elements.amount.value
      if (amount <= 0) {
        this.errors.push('enter a positive amount');
        return;
      }

      axios
        .get(`http://localhost:8000/buy?user_id=1&goal_id=1&amount=${amount}`)
        .then(resp => (this.buyNext = resp.data));
    }
  }
};
</script>

<style lang="scss">
#table-wrapper {
  position: relative;
}
#table-scroll {
  width: 100%;
  overflow: auto;
  padding: 20px 0px;
}
#table-wrapper table {
  width: 100%;
}
table {
  table-layout: fixed;
}
th,
td {
  width: 60px;
}
</style>
