<template>
  <div>
    <portfolio-view :iPortView="wrapper.iPortView" />
  </div>

</template>

<script lang="ts">

  import PortfolioView from "./PortfolioView.vue";
  import { Ticker, FinTableState, PortfolioState } from '../models';
  import axios from 'axios';


  export default {
    components: {
      PortfolioView,
    },
    data () {
      return {
        wrapper: {
          iPortView: <PortfolioState> {
              tickers: [],
          },
        }
      };
    },
    created () {
      this.fetchData()
    },
    methods: {
      fetchData () {
        axios.get('http://localhost:8000/portfolio')
        .then((resp) =>
            this.wrapper.iPortView = resp.data
            console.log(this.wrapper.iPortView)
        );
      }
    },
  }

</script>
