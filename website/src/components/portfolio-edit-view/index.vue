<template>
  <div class="container">
    <template>
      <loader-view class="" v-show="isLoading" :is-loading="isLoading" />
    </template>

    <template>
      <nav-view />
    </template>

    <template>
      <errors-view :errors="errors" v-show="errors.length" />
    </template>

    <div class="bg" v-if="portState != null">
      <edit-portfolio-view
        :port-state="portState"
        @edit-portfolio-event="editPortfolioHandler"
      />
    </div>
  </div>
</template>

<script lang="ts">
import NavView from "../NavView.vue";
import LoaderView from "../LoaderView.vue";
import ErrorsView from "../ErrorsView.vue";
import EditPortfolioView from "./EditPortfolioView.vue";
import router from "../../index.js";
import { EditData } from "./models";
import Vue from "vue";

export default Vue.extend({
  components: {
    NavView,
    ErrorsView,
    LoaderView,
    EditPortfolioView
  },
  data() {
    return {
      portState: null, //FinPortfolioResp
      isLoading: true,
      actualId: this.$route.params.id,
      errors: [] as String[]
    };
  },
  mounted() {
    this.getPortfolioDetail();
  },
  methods: {
    getPortfolioDetail() {
      this.clearErrors();
      /* get portfolio */
      this.isLoading = true;
      this.$appGlobal.axi
        .get(`portfolio/actual/${this.actualId}`)
        .then(resp => {
          this.portState = resp.data;
          this.isLoading = false;
        })
        .catch(error => {
          this.errors.push(error.status);
          this.errors.push(error.statusText);
          this.isLoading = false;
        });
    },
    editPortfolioHandler(data: EditData) {
      console.log(data);
      this.clearErrors();
      this.isLoading = true;
      this.$appGlobal.axi
        .put(`portfolio/actual/edit/${this.actualId}`, data)
        .then(resp => {
          router.push({ name: "dash" });
        })
        .catch(error => {
          this.errors.push(error.status);
          this.errors.push(error.statusText);
          this.isLoading = false;
        });
    },
    clearErrors() {
      this.errors = [];
    }
  }
});
</script>
