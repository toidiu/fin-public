<template>
  <div class="outer">
    <div class="middle">
      <div class="inner"><slot></slot></div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.outer {
  display: table;
  position: absolute;
  height: 100%;
  width: 100%;
}
.middle {
  display: table-cell;
  vertical-align: middle;
  text-align: center;
}
.inner {
  padding: 20px 0;
  margin: 50px auto;
}
</style>
