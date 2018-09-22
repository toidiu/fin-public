<template>
  <div>
    <portfolio-view :portState="wrapper.portState" />

    Buy next: {{ wrapper.nextBuy.symbol }}
    Buy next: {{ wrapper.nextBuy.kind }}
  </div>

</template>

<script lang="ts">
import PortfolioView from "./PortfolioView.vue";
import { Ticker, FinTableState, PortfolioState } from "../models";
import axios from "axios";

export default {
  components: {
    PortfolioView
  },
  data() {
    return {
      wrapper: {
        portState: <PortfolioState>{
          tickers: []
        },
        nextBuy: <Ticker>{}
      }
    };
  },
  created() {
    this.fetchPortfolio();
    this.fetchNextBuy();
  },
  methods: {
    fetchPortfolio() {
      /* get portfolio */
      axios
        .get("http://localhost:8000/portfolio")
        .then(resp => (this.wrapper.portState = resp.data));
    }
    fetchNextBuy() {
      /* get next buy */
      axios
        .get("http://localhost:8000/next")
        .then(resp => (this.wrapper.nextBuy = resp.data));
    }
  }
};
</script>
