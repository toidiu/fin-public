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
          columnsNames: ['Symbol', 'Kind', 'Fee', 'Price', 'Shares',
            'Goal %', 'C %', 'C $'],
          columns: ['symbol', 'kind', 'fee', 'price', 'currentQuantity',
            'goalPercent', 'currentPercent', 'currentValue'],
          tickerList : [],
        }
      };
    },
    created () {
      this.fetchData()
    },
    methods: {
      fetchData () {
        axios.get('http://localhost:8000/')
        .then((resp) =>
          this.initialState.portfolio = resp.data
        );
      }
    },
  }

</script>
