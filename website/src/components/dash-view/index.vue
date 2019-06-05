<template>
  <div class="container">
    <template>
      <loader-view class="" v-show="isLoading" :is-loading="isLoading" />
    </template>

    <template>
      <nav-view />
    </template>

    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <div class="bg" v-if="portListState != null">
      <dash-view
        :port-list-state="portListState"
        @go_to_portfolio="goToPortfolioHandler"
        @add_portfolio="addPortfolioHandler"
      />
    </div>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import DashView from "./DashView.vue";
import router from "../../index.js";
import { Ticker, PortfolioGoalList } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  components: {
    NavView,
    ErrorsView,
    LoaderView,
    DashView
  },
  data() {
    return {
      portListState: null, //PortfolioGoalList[]
      isLoading: true,
      buyNextState: null, //BuyNextResp
      errors: [] as String[]
    };
  },
  mounted() {
    this.getPortfolioList();
  },
  methods: {
    getPortfolioList() {
      this.clearErrors();
      /* get portfolio list */
      this.isLoading = true;
      this.$appGlobal.axi
        .get("portfolio/actual")
        .then(resp => {
          if (resp != undefined) {
            this.portListState = resp.data;
          } else {
            this.errors.push("oops,looks like we made a mistake");
          }
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.response);
          this.isLoading = false;
        });
    },
    goToPortfolioHandler(id) {
      router.push({ name: "portfolio", params: { id: id } });
    },
    addPortfolioHandler() {
      router.push({ name: "portAdd" });
    },
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>
