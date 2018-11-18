<template>
  <div id="table-wrapper">
    <div id="table-scroll"><slot></slot></div>
  </div>
</template>

<style lang="scss">
#table-wrapper {
  position: relative;
}
#table-scroll {
  width: 100%;
  overflow: auto;
  padding: 20px 0px;
}
#table-wrapper table {
  width: 100%;
}
table {
  table-layout: fixed;
}
th,
td {
  width: 80px;
}
</style>
