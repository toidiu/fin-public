<template>
  <div>
    <template>
      <nav-view :is-user-auth="true" />
    </template>

    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <center-view>
      <div class="welcome">Signup for a new Account</div>

      <div>
        <form @submit.prevent="signup">
          <div class="input-text">
            <label for="email">Email</label>
            <input
              class="input"
              id="email"
              type="email"
              name="email"
              placeholder="email"
              required
            />
          </div>
          <div class="input-text">
            <label for="password">Password</label>
            <input
              class="input"
              id="password"
              type="password"
              name="password"
              placeholder="password"
              required
            />
          </div>

          <div class="signup-button input-text">
            <button
              v-bind:class="{ 'is-loading': tryingSignup }"
              class="button is-primary"
              type="submit"
            >
              Sign Up
            </button>
          </div>
        </form>
        <router-link to="/login">
          <a class="">Login</a>
        </router-link>
      </div>
    </center-view>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import CenterView from "../CenterView.vue";
import PulseLoader from "vue-spinner/src/PulseLoader.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import router from "../../index.js";

import Vue from "vue";
import { EventEmitter } from "events";

export default Vue.extend({
  components: {
    NavView,
    CenterView,
    LoaderView,
    ErrorsView,
    PulseLoader
  },
  data() {
    return {
      errors: [] as String[],
      tryingSignup: false
    };
  },
  methods: {
    signup(submitEvent) {
      this.clearErrors();
      var email = submitEvent.target.elements.email.value;
      var pw = submitEvent.target.elements.password.value;
      this.$appGlobal.axi
        .post("users/signup", {
          email: email,
          password: pw
        })
        .then(resp => {
          router.push({ name: "dash" });
        })
        .catch(error => {
          var status = error.response.status;
          switch (status) {
            case 401:
              return this.errors.push("invalid signup");
            default:
              return this.errors.push(`error ${status}. please try later`);
          }
        });
    },
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>

<style lang="scss" scoped>
input {
  max-width: 200px;
}
.welcome {
  padding: 20px;
  font-size: 30px;
}
.input-text {
  margin: 10px 0px;
  & label {
    display: block;
    margin-bottom: 5px;
  }
}
.signup-button {
  padding-top: 20px;
  margin-bottom: 30px;
}
</style>
