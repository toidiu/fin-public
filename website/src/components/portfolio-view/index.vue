<template>
  <div>
    <template>
      <loader-view class="loader" v-show="isLoading" :is-loading="isLoading" />
    </template>

    <template>
      <nav-view />
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
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import PortfolioView from "./PortfolioView.vue";
import BuyNextView from "./BuyNextView.vue";
import ScrollView from "./ScrollView.vue";
import router from "../../index.js";
import {
  BuyNextResp,
  Ticker,
  FinPortfolioResp,
  Action
} from "../../data/models";
import axios from "axios";
import Vue from "vue";

const ax = axios.create({
  baseURL: "http://localhost:8000/portfolio",
  timeout: 5000,
  withCredentials: true
  //headers: { "Access-Control-Max-Age": "1" },
});

ax.interceptors.response.use(
  function(response) {
    return response;
  },
  function(error) {
    if (401 === error.response.status) {
      router.push({ name: "login" });
      return Promise.reject(error);
    } else if (404 === error.response.status) {
      router.push({ name: "portfolio", params: { id: "1" } });
      return Promise.reject(error);
    }
  }
);

export default Vue.extend({
  components: {
    NavView,
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
  mounted() {
    this.getPortfolio();
  },
  methods: {
    getPortfolio() {
      this.clearErrors();
      /* get portfolio */
      this.isLoading = true;
      ax.get("actual/" + this.$route.params.id)
        .then(resp => {
          this.portState = resp.data;
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.response);
          this.isLoading = false;
        });
    },
    calcInvestmentHandler(amount: Number) {
      this.clearErrors();
      this.buyNextState = null;
      this.isLoading = true;
      ax.get(`actual/buy?goal_id=1&amount=${amount}`)
        .then(resp => {
          this.isLoading = false;
          var actions = resp.data.actions;
          if (!Array.isArray(actions) || !actions.length) {
            this.errors.push(
              "enter a higher amount to invest; unable to buy anything at this price"
            );
            return;
          }
          this.buyNextState = resp.data;
        })
        .catch(error => {
          this.errors.push(error.response);
          this.isLoading = false;
        });
    },
    buyNextHandler(actions: Action) {
      this.clearErrors();
      this.isLoading = true;
      ax.post("actual/buy", {
        goal_id: 1,
        actions: actions
      })
        .then(resp => {
          this.portState = resp.data;
          this.clearBuyNext();
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.response);
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
