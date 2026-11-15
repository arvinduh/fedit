import "./styles/global.css";
import { mount } from "svelte";
import App from "./App.svelte";

const target = document.getElementById("app");

if (!target) {
  throw new Error("Target container #app not found");
}

export const app = mount(App, {
  target,
});
