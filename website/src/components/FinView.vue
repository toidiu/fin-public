<template>

  <div id="table-wrapper">
    <div id="table-scroll">

      <table>
        <thead>
          <tr>
            <th v-for="key in initialState.columnsNames">
              {{ key }}
            </th>
          </tr>
        </thead>

        <tbody>
          <tr  v-for="entry in initialState.tickerList">
            <td v-for="key in initialState.columns">
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
    props: ['initialState'],
  }
</script>
