<template>
  <div>
    <template>
      <nav-view :is-user-auth="true" />
    </template>

    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <center-view>
      <div class="welcome">Welcome to Fin</div>

      <div>
        <form @submit.prevent="login">
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
          <div class="login-button input-text">
            <button
              v-bind:class="{ 'is-loading': tryingLogin }"
              class="button is-primary"
              type="submit"
            >
              Login
            </button>
          </div>
        </form>
        <router-link to="/signup">
          <a class="">Sign Up</a>
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
      tryingLogin: false
    };
  },
  methods: {
    login(submitEvent) {
      this.clearErrors();
      var email = submitEvent.target.elements.email.value;
      var pw = submitEvent.target.elements.password.value;
      this.tryingLogin = true;
      this.$appGlobal.axi
        .post("users/login", {
          email: email,
          password: pw
        })
        .then(resp => {
          this.tryingLogin = false;
          router.push({ name: "dash" });
        })
        .catch(error => {
          this.tryingLogin = false;
          var status = error.response.status;
          switch (status) {
            case 401:
              return this.errors.push("invalid login");
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
.login-button {
  padding-top: 20px;
  margin-bottom: 30px;
}
</style>
