<template>
  <div id="calc-investment" class="box">
    <form @submit.prevent="calcInvestmentEvent">
      <div>How much would you like to invest?</div>
      <input
        id="calc-investment-input"
        type="number"
        placeholder="1000"
        class="input text-input"
        name="amount"
        required
        min="1"
        max="10000"
      />
      <button class="button is-dark" type="submit">
        Calculate Investment
      </button>
    </form>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  methods: {
    calcInvestmentEvent: function(submitEvent) {
      this.$emit(
        "calc-investment-event",
        submitEvent.target.elements.amount.value
      );
    }
  }
});
</script>

<style lang="scss" scoped>
#calc-investment {
  margin: 1em 0 4em 0;
}
#calc-investment-input {
  display: inline;
  width: 150px;
}
</style>
