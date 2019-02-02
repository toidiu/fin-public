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

    <portfolio-list-view
      v-if="portListState != null"
      :port-list-state="portListState"
    />
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import PortfolioListView from "./PortfolioListView.vue";
import router from "../../index.js";
import { Ticker, PortfolioGoalList } from "../../data/models";
import axios from "axios";
import Vue from "vue";

const ax = axios.create({
  baseURL: "http://localhost:8000/portfolio/",
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
    PortfolioListView
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
      ax.get("goal")
        .then(resp => {
          this.portListState = resp.data;
          this.isLoading = false;
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
