<template>
  <div>

    <loader class="loader" v-show="isLoading" :isLoading="isLoading"></loader>

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
import Loader from './Loader.vue'
import PortfolioView from "./PortfolioView.vue";
import BuyNextView from "./BuyNextView.vue";
import { BuyNextResp, Ticker, FinPortfolioResp } from "../models";
import axios from "axios";

export default {
  components: {
    Loader,
    PortfolioView,
    BuyNextView
  },
  data() {
    return {
      portState: <FinPortfolioResp>{
        tickers: []
      },
      isLoading: true,
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
      this.isLoading = true
      axios
        .get("http://localhost:8000/portfolio?user_id=1&goal_id=1")
        .then(resp => {
          this.portState = resp.data
      this.isLoading = false
        })
        .catch(error => {
          console.log(error.response)
          this.isLoading = false
        });
    }
    fetchBuyNext(submitEvent) {
      var amount = submitEvent.target.elements.amount.value
      if (amount <= 0) {
        this.errors.push('enter a positive amount');
        return;
      }

      this.isLoading = true
      axios
        .get(`http://localhost:8000/buy?user_id=1&goal_id=1&amount=${amount}`)
        .then(resp => {
          this.buyNext = resp.data
          this.isLoading = false
        })
        .catch(error => {
          console.log(error.response)
          this.isLoading = false
        });
    }
  }
};
</script>

<style lang="scss">
.loader {
  z-index: 2;
}
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
