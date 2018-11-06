<template>
  <div>

    <loader-view class="loader"
      v-show="isLoading"
      :isLoading="isLoading" />

    <errors-view :errors="errors" />

    <div id="table-wrapper">
      <div id="table-scroll">

        <template v-if="portState.tickers.length" >
          <portfolio-view
            :portState="portState"
            @calcInvestmentEvent="calcInvestmentHandler" />
        </template>

        <buy-next-view v-if="buyNextState.actions.length"
          :portState="portState"
          :buyNextState="buyNextState"
          @buyNextEvent="buyNextHandler" />

        </div>
      </div>
    </div>

</template>

<script lang="ts">
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import PortfolioView from "./PortfolioView.vue";
import BuyNextView from "./BuyNextView.vue";
import { BuyNextResp, Ticker, FinPortfolioResp } from "../../models";
import axios from "axios";

export default {
  components: {
    ErrorsView,
    LoaderView,
    PortfolioView,
    BuyNextView
  },
  data() {
    return {
      portState: <FinPortfolioResp>{
        tickers: []
      },
      isLoading: true,
      buyNextState: <BuyNextResp>{
        actions: [],
        buy_value: 0
      },
      errors: []
    };
  },
  created() {
    this.getPortfolio();
  },
  methods: {
    getPortfolio() {
      this.clearErrors();
      /* get portfolio */
      this.isLoading = true;
      axios
        .get("http://localhost:8000/portfolio?user_id=1&goal_id=1")
        .then(resp => {
          this.portState = resp.data;
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.response.data);
          this.isLoading = false;
        });
    },
    calcInvestmentHandler(amount) {
      this.clearErrors();
      //var amount = submitEvent.target.elements.amount.value;
      console.log(23);
      if (amount <= 0) {
        this.errors.push("enter a positive amount to invest");
        return;
      }
      if (amount > 10000) {
        this.errors.push("enter an amount less than $10,000");
        return;
      }

      this.isLoading = true;
      axios
        .get(`http://localhost:8000/buy?user_id=1&goal_id=1&amount=${amount}`)
        .then(resp => {
          var actions = resp.data.actions;
          if (!Array.isArray(actions) || !actions.length) {
            this.errors.push(
              "enter a higher amount to invest; unable to buy anything at this price"
            );
          }
          this.buyNextState = resp.data;
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.response.data);
          this.isLoading = false;
        });
    },
    buyNextHandler(actions) {
      this.clearErrors();
      this.isLoading = true;
      axios
        .post("http://localhost:8000/buy", {
          user_id: 1,
          goal_id: 1,
          actions: actions
        })
        .then(resp => {
          this.portState = resp.data;
          this.clearBuyNext();
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.response.data);
          this.isLoading = false;
        });
    },
    clearBuyNext() {
      this.buyNextState.actions = [];
    },
    clearErrors() {
      this.errors = [];
    }
  }
};
</script>

<style lang="scss">
.errors {
  color: #b74b4b;
  ul {
    list-style: disc;
    padding: 10px 20px;
  }
}
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
  width: 80px;
}
</style>
