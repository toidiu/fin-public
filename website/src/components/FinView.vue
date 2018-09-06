<template>

  <div id="table-wrapper">
    <div id="table-scroll">

      <table>
        <thead>
          <tr>
            <th v-for="(key, idx) in initialState.columnsNames" v-bind:key="idx">
              {{ key }}
            </th>
          </tr>
        </thead>

        <tbody>
          <tr  v-for="(entry, idx) in initialState.tickerList" v-bind:key="entry + idx">
            <td v-for="(key, jdx) in initialState.columns" v-bind:key="entry.symbol + jdx">
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
    props: {
      initialState: Object
    },
    data () {
      return {
      }
    }
  }
</script>

<style lang="scss">

  table {
    border: 1px solid #000;
    border-radius: 3px;
  }

  th {
    /* color: #aa704e; */
    background-color: #c9c9c9;
    /* color: rgba(255, 255, 255, 0.66); */
    /* cursor: pointer; */
    /* -webkit-user-select: none; */
    /* -moz-user-select: none; */
    /* -ms-user-select: none; */
    /* user-select: none; */
    font-size: 13px;
  }

  td {
    background-color: #f9f9f9;
    /* color: #0079bf; */
    font-size: 14px;
  }

  th,
  td {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    /* min-width: 30px; */
    max-width: 60px;
    padding: 10px 10px;
  }

  #table-wrapper {
    position:relative;
  }
  #table-scroll {
    width: 100%;
    overflow:auto;
    margin-top:20px;
  }
  #table-wrapper table {
    /* width:100%; */
  }
</style>
