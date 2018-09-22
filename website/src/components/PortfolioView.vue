<template>

  <div>

    <h1>
      fin portfolio
    </h1>

    <div id="table-wrapper">
      <div id="table-scroll">

        <table>
          <thead>
            <tr>
              <th v-for="(key, idx) in dataW.columnsNames" v-bind:key="idx">
                {{ key }}
              </th>
            </tr>
          </thead>

          <!-- Stocks -->
          <tbody>
            <tr  v-for="(entry, idx) in stocks" v-bind:key="entry + idx">
              <td v-bind:class=key v-for="(key, jdx) in dataW.columnsKeys" v-bind:key="entry.symbol + jdx">
                {{ entry[key] }}
              </td>
            </tr>
          </tbody>

          <!-- Summary between stocks and bonds -->
          <tbody>
            <tr>
              <td class="summary"></td>
              <td class="summary"></td>
              <td class="summary"></td>
              <td class="summary"></td>
              <td class="summary goal_percent">{{ portState.goal_stock_percent }}</td>
              <td class="summary actual_percent">{{ portState.actual_stock_percent }}</td>
              <td class="summary"></td>
            </tr>
          </tbody>

          <!-- Bonds -->
          <tbody>
            <tr  v-for="(entry, idx) in bonds" v-bind:key="entry + idx">
              <td v-bind:class=key v-for="(key, jdx) in dataW.columnsKeys" v-bind:key="entry.symbol + jdx">
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
import { FinTableState } from "../models";

export default {
  props: {
    portState: Object as () => FinTableState
  },
  data: function() {
    return {
      dataW: {
        columnsNames: ['Symbol', 'Kind', 'Fee', 'Price', 'Goal %', 'Actual %', 'Actual $'],
        columnsKeys: ['symbol', 'kind', 'fee', 'price', 'goal_percent', 'actual_percent', 'actual_value'],
      }
    }
  }
  computed: {
    stocks: function() {
      return this.portState.tickers.filter(tic => tic.kind === "stock");
    },
    bonds: function() {
      return this.portState.tickers.filter(tic => tic.kind === "bond");
    }
  }
};
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
  &.goal_percent,
  &.symbol {
    & {
      font-weight: bold;
    }
  }
}

td.summary {
  background-color: #d5d5d5;
}

td {
  &.fee,
  &.actual_percent,
  &.goal_percent {
    &.summary {
      color: red;
    }

    &::after {
      content: "%";
    }
  }
}

td {
  &.price,
  &.actual_value {
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
  position: relative;
}
#table-scroll {
  width: 100%;
  overflow: auto;
  margin-top: 20px;
}
#table-wrapper table {
  /* width:100%; */
}
</style>
