<template>
  <div>
    <portfolio-view :viewTableState="wrapper.viewTableState" />
    <old-fin-view :viewTableState="wrapper.oldViewTableState" />
  </div>

</template>

<script lang="ts">

  import OldFinView from "./OldFinView.vue";
  import PortfolioView from "./PortfolioView.vue";
  import { Ticker, FinTableState, PortfolioState } from '../models';
  import axios from 'axios';


  export default {
    components: {
      OldFinView,
      PortfolioView,
    },
    data () {
      return {
        wrapper: {
          viewTableState: <PortfolioState> {
              tickers: [],
          }
          oldViewTableState: <FinTableState> {
            columnsNames: ['Symbol', 'Kind', 'Fee', 'Price',
              'Goal %', 'C %', 'C $'],
            columns: ['symbol', 'kind', 'fee', 'price',
              'goalPercent', 'currentPercent', 'currentValue'],
            portfolio : {
              current_detail: {
                stocks: [],
                bonds: [],
                goal_stock_percent: 3;
                current_stock_percent: 33;
                deviation_percent: 22;
              }
            },
          }
        }
      };
    },
    created () {
      this.fetchOldData()
      this.fetchData()
    },
    methods: {
      fetchOldData () {
        axios.get('http://localhost:8000/old')
        .then((resp) =>
            this.wrapper.oldViewTableState.portfolio = resp.data
        );
      },

      fetchData () {
        axios.get('http://localhost:8000/portfolio')
        .then((resp) =>
            this.wrapper.viewTableState = resp.data
            console.log(this.wrapper.viewTableState)
        );
      }
    },
  }

</script>
