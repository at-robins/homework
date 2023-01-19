import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "@quasar/extras/material-icons/material-icons.css";
import "quasar/src/css/index.sass";
import { Quasar } from "quasar";
import quasarLang from "quasar/lang/en-GB";
import quasarIconSet from "quasar/icon-set/svg-material-icons";
import { createI18n } from "vue-i18n";
import VueClickAway from "vue3-click-away";
import messages from "@/scripts/messages";

function getCurrentLocale(): string {
  const currentLocale = localStorage.getItem("app_locale");
  return currentLocale ? JSON.parse(currentLocale) : "de";
}

const app = createApp(App);
const i18n = createI18n({
  legacy: false,
  locale: getCurrentLocale(),
  fallbackLocale: "en",
  globalInjection: true,
  messages,
});

app.use(router);
app.use(Quasar, {
  plugins: {},
  lang: quasarLang,
  iconSet: quasarIconSet,
});
app.use(i18n);
app.use(VueClickAway);

app.mount("#app");
