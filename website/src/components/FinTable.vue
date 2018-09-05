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
          columnsNames: ['symbol', 'fee', 'price', 'kind', 'description',
            'Goal %', 'current %', 'current $'],
          columns: ['symbol', 'fee', 'price', 'investment_kind', 'description',
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
          this.initialState.tickerList = resp.data
        );
      }
    },
  }

</script>
