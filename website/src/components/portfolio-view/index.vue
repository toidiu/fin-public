<template>
  <div>
    <template>
      <loader-view class="" v-show="isLoading" :is-loading="isLoading" />
    </template>

    <template>
      <nav-view />
    </template>

    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <div class="bg">
      <template>
        <portfolio-header-view
          v-if="portState != null"
          :port-state="portState"
        />
      </template>
      <scroll-view>
        <template>
          <portfolio-table-view
            v-if="portState != null"
            :port-state="portState"
            @calc-investment-event="calcInvestmentHandler"
          />
        </template>

        <buy-next-view
          v-show="buyNextState != null"
          :port-state="portState"
          :buy-next-state="buyNextState"
          @buy-next-event="buyNextHandler"
        />
      </scroll-view>
      <calc-investment-view @calc-investment-event="calcInvestmentHandler" />
    </div>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import PortfolioTableView from "./PortfolioTableView.vue";
import PortfolioHeaderView from "./PortfolioHeaderView.vue";
import CalcInvestmentView from "./CalcInvestmentView.vue";
import BuyNextView from "./BuyNextView.vue";
import ScrollView from "./ScrollView.vue";
import router from "../../index.js";
import { BuyNextData, BuyNextResp, FinPortfolioResp } from "./models";
import { Ticker, Action } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  components: {
    NavView,
    ErrorsView,
    LoaderView,
    PortfolioTableView,
    PortfolioHeaderView,
    CalcInvestmentView,
    BuyNextView,
    ScrollView
  },
  data() {
    return {
      portState: null, //FinPortfolioResp
      actualId: this.$route.params.id,
      isLoading: true,
      buyNextState: null, //BuyNextResp
      errors: [] as String[]
    };
  },
  mounted() {
    this.getPortfolio();
    console.log();
  },
  methods: {
    getPortfolio() {
      this.clearErrors();
      /* get portfolio */
      this.isLoading = true;
      this.$appGlobal.axi
        .get(`portfolio/actual/${this.actualId}`)
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
      // FIXME ==========================
      this.$appGlobal.axi
        .get(
          `portfolio/actual/buy?goal_port_id=${
            this.portState.goal_id
          }&actual_port_id=${this.actualId}&amount=${amount}`
        )
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
    buyNextHandler(actions: Action[]) {
      this.clearErrors();
      this.isLoading = true;

      let data = new Object() as BuyNextData;
      data.goal_id = parseInt(this.portState.goal_id);
      data.port_a_id = parseInt(this.actualId);
      data.actions = actions;
      this.$appGlobal.axi
        .post("portfolio/actual/buy", data)
        // ax.post("actual/buy", {
        //   goal_id: this.portState.goal_id,
        //   port_a_id: this.actualId,
        //   actions: actions
        // })
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
