import Vue from "vue";
import VueRouter from "vue-router";
import App from "./App.vue";
import Login from "./components/login-view/index.vue";
import Signup from "./components/signup-view/index.vue";
import Portfolio from "./components/portfolio-view/index.vue";
import Dash from "./components/dash-view/index.vue";
import PortAdd from "./components/add-portfolio-view/index.vue";
import PageNotFound from "./components/page-not-found/index.vue";
import "./../node_modules/bulma/css/bulma.css";

Vue.use(VueRouter);

const router = new VueRouter({
  routes: [
    {
      path: "/",
      component: Login
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
      path: "/dash",
      component: Dash,
      name: "dash"
    },
    {
      path: "/portfolio/add",
      component: PortAdd,
      name: "portAdd"
    },
    {
      path: "/portfolio/:id",
      component: Portfolio,
      name: "portfolio"
    },
    {
      path: "*",
      component: PageNotFound,
      name: "notfound"
    }
  ]
});

const app = new Vue({
  router,
  render: createEle => createEle(App)
}).$mount("#app");

export default router;
