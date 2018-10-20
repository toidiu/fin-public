<template>
  <div>
    <portfolio-view :portState="wrapper.portState" />

    Buy next: {{ wrapper.nextBuy.symbol }} | {{ wrapper.nextBuy.kind }}
  </div>

</template>

<script lang="ts">
import PortfolioView from "./PortfolioView.vue";
import { Ticker, PortfolioViewStart } from "../models";
import axios from "axios";

export default {
  components: {
    PortfolioView
  },
  data() {
    return {
      wrapper: {
        portState: <FinPortfolioResp>{
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
