import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import Login from "./components/login-view/index.vue";
import Portfolio from "./components/portfolio-view/index.vue";

Vue.use(VueRouter);

const router = new VueRouter({
  routes: [
    {
      path: "/",
      component: Login,
      name: "login"
    },
    {
      path: "/portfolio",
      component: Portfolio,
      name: "portfolio"
    }
  ]
});

const app = new Vue({
  router,
  render: createEle => createEle(App)
}).$mount("#app");

export default router;
