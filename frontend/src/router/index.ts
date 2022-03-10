import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import AttachmentsView from "../views/AttachmentsView.vue";
import RecipesView from "../views/RecipesView.vue";
import RecipeDetailsView from "../views/RecipeDetailsView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      redirect: "/ui/",
    },
    {
      path: "/ui/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/ui/attachments",
      name: "attachments",
      component: AttachmentsView,
    },
    {
      path: "/ui/recipes",
      name: "recipes",
      component: RecipesView,
    },
    {
      path: "/ui/recipe/:id",
      name: "recipe",
      component: RecipeDetailsView,
      props: true,
    },
  ],
});

export default router;
