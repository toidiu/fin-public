<template>
  <nav>
    <a href="#" v-if="!isUserAuth" v-on:click="logout">logout</a>
    <router-link v-if="isUserAuth" to="/login"> <a>login</a> </router-link>
    <router-link v-if="isUserAuth" to="/signup"><a>signup</a></router-link>
    <router-link to="/plan/portfolio"> <a>plan</a> </router-link>
    <router-link to="/portfolio"> <a>portfolio</a> </router-link>
  </nav>
</template>

<script lang="ts">
import Vue from "vue";
import axios from "axios";
import router from "../index.js";

const ax = axios.create({
  baseURL: "http://localhost:8000/users",
  timeout: 10000,
  withCredentials: true
  //headers: { "Access-Control-Max-Age": "1" },
});
ax.interceptors.response.use(
  function(response) {
    return response;
  },
  function(error) {
    if (401 === error.response.status) {
      router.push({ name: "login" });
      return Promise.reject(error);
    }
  }
);

export default Vue.extend({
  props: {
    isUserAuth: {
      type: Boolean,
      default: false
    }
  },
  methods: {
    logout: function(event) {
      event.preventDefault();
      ax.post("/logout").catch(error => {});
      router.push({ name: "login" });
    }
  }
});
</script>

<style lang="scss" scoped></style>
