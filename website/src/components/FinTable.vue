<template>

    <fin-view :initialState="initialState" />

</template>

<script lang="ts">

  import FinView from "./FinView.vue";
  import { Ticker, FinTableState } from '../models';
  import axios from 'axios';


  export default {
    components: {
      FinView,
    },
    data () {
      return {
        initialState: <FinTableState> {
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
      };
    },
    created () {
      this.fetchData()
    },
    methods: {
      fetchData () {
        axios.get('http://localhost:8000/old')
        .then((resp) =>
            this.initialState.portfolio = resp.data
        );
      }
    },
  }

</script>
