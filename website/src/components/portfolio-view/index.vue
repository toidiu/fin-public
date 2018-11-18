<template>
  <div>
    <template>
      <loader-view class="loader" v-show="isLoading" :is-loading="isLoading" />
    </template>
    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <scroll-view>
      <template>
        <portfolio-view
          v-if="portState != null"
          :port-state="portState"
          @calc-investment-event="calcInvestmentHandler"
        />
      </template>

      <buy-next-view
        v-if="buyNextState != null"
        :port-state="portState"
        :buy-next-state="buyNextState"
        @buy-next-event="buyNextHandler"
      />
    </scroll-view>
  </div>
</template>

<script lang="ts">
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import PortfolioView from "./PortfolioView.vue";
import BuyNextView from "./BuyNextView.vue";
import ScrollView from "./ScrollView.vue";
import { BuyNextResp, Ticker, FinPortfolioResp, Action } from "../models";
import axios from "axios";
import Vue from "vue";

const ax = axios.create({
  baseURL: "http://localhost:8000/portfolio",
  timeout: 10000,
  withCredentials: true
  //headers: { "Access-Control-Max-Age": "1" },
});

export default Vue.extend({
  components: {
    ErrorsView,
    LoaderView,
    PortfolioView,
    BuyNextView,
    ScrollView
  },
  data() {
    return {
      portState: null, //FinPortfolioResp
      isLoading: true,
      buyNextState: null, //BuyNextResp
      errors: [] as String[]
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
      ax.get("/?user_id=1&goal_id=1")
        .then(resp => {
          this.portState = resp.data;
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error);
          this.isLoading = false;
        });
    },
    calcInvestmentHandler(amount: Number) {
      this.clearErrors();
      if (amount <= 0) {
        this.errors.push("enter a positive amount to invest");
        return;
      }
      if (amount > 10000) {
        this.errors.push("enter an amount less than $10,000");
        return;
      }

      this.isLoading = true;
      ax.get(`/buy?user_id=1&goal_id=1&amount=${amount}`)
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
    buyNextHandler(actions: Action) {
      this.clearErrors();
      this.isLoading = true;
      ax.post("/buy", {
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
      this.buyNextState = null;
    },
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>
