<template>

  <div>
    this is {{ initialState.portfolio.name }}

    </br>

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
            <tr  v-for="(entry, idx) in initialState.portfolio.current_detail.stocks" v-bind:key="entry + idx">
              <td v-bind:class=key v-for="(key, jdx) in initialState.columns" v-bind:key="entry.symbol + jdx">
                {{ entry[key] }}
              </td>
            </tr>
          </tbody>

          <tbody>
            <tr>
              <td></td>
              <td></td>
              <td></td>
              <td></td>
              <td></td>
              <td class="goalPercent">{{ initialState.portfolio.current_detail.current_stock_percent }}</td>
              <td class="currentPercent">{{ initialState.portfolio.current_detail.current_stock_percent }}</td>
              <td></td>
            </tr>
          </tbody>

          <tbody>
            <tr  v-for="(entry, idx) in initialState.portfolio.current_detail.bonds" v-bind:key="entry + idx">
              <td v-bind:class=key v-for="(key, jdx) in initialState.columns" v-bind:key="entry.symbol + jdx">
                {{ entry[key] }}
              </td>
            </tr>
          </tbody>



        </table>
      </div>
    </div>
  </div>

</template>

<script lang="ts">

  import { FinTableState } from '../models';

  export default {
    props: {
      initialState: Object
    },
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
    font-size: 13px;
  }

  td {
    &.goalPercent,
    &.symbol, {

      & {
        font-weight: bold;
      }

    }
  }

  td {
    &.fee,
    &.currentPercent,
    &.goalPercent, {

      &::after {
        content: "%";
      }

    }
  }

  td {
    &.price,
    &.currentValue, {

      &::before {
        content: "$";
        margin: 0 -2px;
      }

    }
  }

  th,
  td {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 40px;
    max-width: 60px;
    padding: 10px 10px;
    text-align: center;
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
