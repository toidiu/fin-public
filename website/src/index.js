import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import Login from "./components/login-view/index.vue";
import Signup from "./components/signup-view/index.vue";
import Portfolio from "./components/portfolio-view/index.vue";
import PortfolioList from "./components/portfolio-list-view/index.vue";

Vue.use(VueRouter);

const router = new VueRouter({
  routes: [
    {
      path: "/",
      component: Login,
      name: "home"
    },
    {
      path: "/login",
      component: Login,
      name: "login"
    },
    {
      path: "/signup",
      component: Signup,
      name: "signup"
    },
    {
      path: "/plan/portfolio",
      component: PortfolioList,
      name: "plan"
    },
    {
      path: "/portfolio/:id",
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
