<template>
  <div class="container columns is-multiline">
    <template v-for="(port, idx) in portListState">
      <div
        v-on:click="goToPortfolio(port.id)"
        class="col-item column is-one-quarter"
      >
        <div class="tile is-parent is-vertical">
          <div class="tile is-child has-text-weight-semibold">
            {{ port.name }}
          </div>
          <div class="tile is-box is-child">
            <p>{{ port.stock_percent }}% stocks</p>
            <p>{{ 100 - port.stock_percent }}% bonds</p>
          </div>
          <p class="tile is-box is-child is-size-7">
            {{ port.description }}
          </p>
        </div>
        <button
          id="option"
          class="button is-text"
          v-on:click="editPortfolio(port.id)"
        >
          edit
        </button>
      </div>
    </template>

    <div
      v-on:click="addPortfolio"
      id="add-port"
      class="col-item column is-one-quarter"
    >
      <div class="container center-img-in-container">
        <img src="/../static/images/plus.svg" class="is-64x64" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { PortfolioActualList, Ticker } from "../../data/models";
import Vue from "vue";

export default Vue.extend({
  props: {
    portListState: Array as () => PortfolioActualList[]
  },
  methods: {
    goToPortfolio: function(id) {
      this.$emit("go_to_portfolio", id);
    },
    editPortfolio: function(id) {
      this.$emit("edit_portfolio", id);
    },
    addPortfolio: function() {
      this.$emit("add_portfolio");
    }
  }
});
</script>

<style lang="scss" scoped>
#add-port {
  border: 0;
}
.container {
  padding: 0.75em 0;
}
.center-img-in-container {
  display: flex;
  height: 100%;
  width: 64px;
}
.col-item {
  cursor: pointer;
  margin: 0.5em;
  border: 1px solid #fff;
  border-color: #000;

  &:hover {
    background-color: #eee;
  }
  &:active {
    background-color: #ccc;
  }
}
</style>
