import Vue from "vue";

<template>

  <div id="table-wrapper">
    <div id="table-scroll">
      <table  border=1 id="fin" align="center" style="width:100%">
        <tr>
          <th>ticker</th>
          <th>fees</th>
          <th>goal %</th>
          <th>current %</th>
        </tr>
        <tr>
          <td>{{msg}}</td>
          <td>Smith</td>
          <td>50</td>
          <td>50</td>
        </tr>
        <tr>
          <td>Eve</td>
          <td>Jackson</td>
          <td>94</td>
          <td>94</td>
        </tr>
      </table>
    </div>
  </div>

</template>

<script lang="ts">
  export default {
    data () {
      return {
        msg: '',
      }
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
  tbody {
    height: 80em;
    overflow: scroll;
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
  #table-wrapper table * {
    color: black;
  }
  #table-wrapper table thead th .text {
    position:absolute;
    top:-20px;
    z-index:2;
    height:20px;
    width:35%;
    border:1px solid red;
  }
</style>
