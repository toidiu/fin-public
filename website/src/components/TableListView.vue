<template>

  <div id="table-wrapper">
    <div id="table-scroll">

      <table>
        <thead>
          <tr>
            <th v-for="key in columnsNames">
              {{ key }}
            </th>
          </tr>
        </thead>

        <tbody>
          <tr  v-for="entry in tickerList">
            <td v-for="key in columns">
              {{ entry[key] }}
            </td>
          </tr>
        </tbody>

      </table>
    </div>
  </div>

</template>

<script lang="ts">

  import { FinTableState } from '../models';

  export default {
    data () {
      const initialState: FinTableState = {
        columnsNames: ['ticker', 'fee', 'goal %', 'current %'],
        columns: ['name', 'fee', 'currentGoal', 'currentPercent'],
        tickerList : [
          { name: "vtx", fee: .25, currentGoal: 3, currentPercent: 2 },
          { name: "voe", fee: .05 },
        ],
      };

      return initialState;
    },
    created () {
      this.fetchData()
    },
    methods: {
      fetchData () {
        this.msg = "bla"
      }
    },
  }
</script>

<style lang="css">

  table {
    border: 2px solid #42b983;
    border-radius: 3px;
    background-color: #fff;
  }

  th {
    background-color: #42b983;
    color: rgba(255, 255, 255, 0.66);
    cursor: pointer;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }

  td {
    background-color: #f9f9f9;
  }

  th,
  td {
    min-width: 50px;
    padding: 10px 20px;
  }

  th.active {
    color: #fff;
  }


  #table-wrapper {
    position:relative;
  }
  #table-scroll {
    height:150px;
    width: 80%;
    overflow:auto;
    margin-top:20px;
  }
  #table-wrapper table {
    width:100%;
  }
</style>
