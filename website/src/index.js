import Vue from "vue";
import App from "./App.vue";
import '../static/test.scss';

let v = new Vue({
    el: "#app",
    render: h => h(App)
});
