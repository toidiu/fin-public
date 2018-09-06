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
          columnsNames: ['Symbol', 'Fee', 'Price', 'Kind', 'Description',
            'Goal %', 'Current %', 'Current $'],
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
