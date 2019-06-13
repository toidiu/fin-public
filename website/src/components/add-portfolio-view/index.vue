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

    <div class="bg" v-if="portGoalState != null">
      <add-portfolio-view
        :port-goal-state="portGoalState"
        @add-portfolio-event="addPortfolioHandler"
      />
    </div>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import AddPortfolioView from "./AddPortfolioView.vue";
import router from "../../index.js";
import { AddData } from "./models";
import Vue from "vue";

export default Vue.extend({
  components: {
    NavView,
    ErrorsView,
    LoaderView,
    AddPortfolioView
  },
  data() {
    return {
      portGoalState: null, //PortfolioGoalList[]
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
        .get("portfolio/goal")
        .then(resp => {
          if (resp != undefined) {
            this.portGoalState = resp.data;
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
    addPortfolioHandler(data: AddData) {
      this.clearErrors();
      this.isLoading = true;
      this.$appGlobal.axi
        .post("portfolio/actual", data)
        .then(resp => {
          router.push({ name: "dash" });
        })
        .catch(error => {
          this.errors.push(error.response);
          this.isLoading = false;
        });
    },
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>
