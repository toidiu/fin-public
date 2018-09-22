<template>

  <div>

    <h1>
      {{ viewTableState.portfolio.name }}
    </h1>

    <div id="table-wrapper">
      <div id="table-scroll">

        <table>
          <thead>
            <tr>
              <th v-for="(key, idx) in viewTableState.columnsNames" v-bind:key="idx">
                {{ key }}
              </th>
            </tr>
          </thead>

          <tbody>
            <tr  v-for="(entry, idx) in viewTableState.portfolio.current_detail.stocks" v-bind:key="entry + idx">
              <td v-bind:class=key v-for="(key, jdx) in viewTableState.columns" v-bind:key="entry.symbol + jdx">
                {{ entry[key] }}
              </td>
            </tr>
          </tbody>

          <tbody>
            <tr>
              <td class="summary"></td>
              <td class="summary"></td>
              <td class="summary"></td>
              <td class="summary"></td>
              <td class="summary goalPercent">{{ viewTableState.portfolio.current_detail.goal_stock_percent }}</td>
              <td class="summary currentPercent">{{ viewTableState.portfolio.current_detail.current_stock_percent }}</td>
              <td class="summary"></td>
            </tr>
          </tbody>

          <tbody>
            <tr  v-for="(entry, idx) in viewTableState.portfolio.current_detail.bonds" v-bind:key="entry + idx">
              <td v-bind:class=key v-for="(key, jdx) in viewTableState.columns" v-bind:key="entry.symbol + jdx">
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
    viewTableState: Object as () => FinTableState
    oldViewTableState: Object as () => FinTableState
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
  &.goalPercent,
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
  &.currentPercent,
  &.goalPercent {
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
  &.currentValue {
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
