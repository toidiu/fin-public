<template>

  <div id="app">
    <portfolio />
  </div>

</template>

<script>
import Portfolio from "./components/portfolio-view/index.vue";

export default {
  name: "app",
  components: {
    Portfolio
  }
};
</script>

<style lang="css">
html {
  box-sizing: border-box;
  font-size: 16px;
}

*, *:before, *:after {
  box-sizing: inherit;
}

body, h1, h2, h3, h4, h5, h6, p, ol, ul {
  margin: 0;
  padding: 0;
  font-weight: normal;
}

ol, ul {
  list-style: none;
}

img {
  max-width: 100%;
  height: auto;
}
</style>
