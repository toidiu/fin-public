<template>
  <div>
    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <center-view>
      <div class="welcome">Welcome to Fin</div>

      <div>
        <form @submit.prevent="login">
          <div class="input-text">
            <label for="email">email</label>
            <input id="email" type="email" name="email" required />
          </div>
          <div class="input-text">
            <label for="password">password</label>
            <input id="password" type="password" name="password" required />
          </div>
          <div class="input-text"><button type="submit">login</button></div>
        </form>
      </div>
    </center-view>
  </div>
</template>

<script lang="ts">
import CenterView from "../CenterView.vue";
import PulseLoader from "vue-spinner/src/PulseLoader.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import router from "../../index.js";

import axios from "axios";
import Vue from "vue";
import { EventEmitter } from "events";

const ax = axios.create({
  baseURL: "http://localhost:8000/users",
  timeout: 10000,
  withCredentials: true
  //headers: { "Access-Control-Max-Age": "1" },
});

export default Vue.extend({
  components: {
    CenterView,
    LoaderView,
    ErrorsView,
    PulseLoader
  },
  data() {
    return {
      errors: [] as String[]
    };
  },
  methods: {
    login(submitEvent) {
      this.clearErrors();
      var email = submitEvent.target.elements.email.value;
      var pw = submitEvent.target.elements.password.value;
      ax.post("/login", {
        email: email,
        password: pw
      })
        .then(resp => {
          router.push({ name: "portfolio.index" });
        })
        .catch(error => {
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
</style>
