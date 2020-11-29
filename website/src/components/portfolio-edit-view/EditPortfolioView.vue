<template>
  <div class="container">
    <form @submit.prevent="editPortfolio">
      <div class="section">
        <h3 class="title is-3">Edit Portfolio</h3>
        <div class="field">
          <label class="label">Name</label>
          <div class="control">
            <input
              class="input text-input"
              type="text"
              name="name"
              :value="portState.name"
              required
            />
          </div>
        </div>

        <div class="field">
          <label class="label">Goal Portfolio</label>
          <div class="control">
            {{ portState.fk_port_g_id }}
            <p class="help">
              The portfolio definition which sets the stocks and their % weight.
            </p>
          </div>
        </div>

        <div class="field">
          <label class="label">Description</label>
          <div class="control">
            <textarea name="description" required rows="3" cols="50">{{
              portState.description
            }}</textarea>
          </div>
        </div>

        <div class="field">
          <label class="label">Stock %</label>
          <div class="control">
            <input
              class="input text-input"
              type="number"
              min="0"
              max="100"
              placeholder="50"
              name="stockPer"
              :value="portState.stock_percent"
            />
          </div>
          <p class="help">
            What percent of the portfolio should be invested in stocks
          </p>
        </div>

        <div class="field is-grouped">
          <div class="control">
            <button class="button is-primary">Save</button>
          </div>
        </div>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import { EditData } from "./models";
import Vue from "vue";

export default Vue.extend({
  props: {
    portState: Object
  },
  methods: {
    editPortfolio: function(submitEvent) {
      console.log(submitEvent.target);
      let data = new Object() as EditData;
      data.name = submitEvent.target.elements.name.value;
      data.description = submitEvent.target.elements.description.value;
      data.stockPercent = parseInt(submitEvent.target.elements.stockPer.value);
      data.goalPortId = parseInt(this.portState.fk_port_g_id);
      this.$emit("edit-portfolio-event", data);
    }
  }
});
</script>

<style lang="scss" scoped>
.section {
  max-width: 50em;
  padding-top: 2rem;
}
.text-input {
  max-width: 10em;
}
.center-img-in-container {
  display: flex;
  height: 100%;
  width: 64px;
}
.col {
  margin-left: 0.75em;
  margin-right: 0.75em;
}
.col-item {
  cursor: pointer;
  background-color: #1bbde7;
  margin: 0.5em;
  border-radius: 2%;
  border: 1px solid #fff;

  &:hover {
    border-color: #000;
    background-color: #17aad1;
  }
  &:active {
    border-color: #000;
    background-color: #1293b6;
  }
}
</style>
