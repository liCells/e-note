import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: "/chooseFolder",
    name: 'chooseFolder',
    component: () => import(/* webpackChunkName: "about" */ '../components/chooseFolder.vue')
  },
  {
    path: "/writingRoom",
    name: 'writingRoom',
    component: () => import(/* webpackChunkName: "about" */ '../components/writingRoom.vue')
  },
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
