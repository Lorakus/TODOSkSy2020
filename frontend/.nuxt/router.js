import Vue from 'vue'
import Router from 'vue-router'
import { interopDefault } from './utils'
import scrollBehavior from './router.scrollBehavior.js'

const _c49b82a4 = () => interopDefault(import('..\\pages\\add.vue' /* webpackChunkName: "pages_add" */))
const _77478366 = () => interopDefault(import('..\\pages\\edit.vue' /* webpackChunkName: "pages_edit" */))
const _68a0afc4 = () => interopDefault(import('..\\pages\\impressum.vue' /* webpackChunkName: "pages_impressum" */))
const _08e3451f = () => interopDefault(import('..\\pages\\index.vue' /* webpackChunkName: "pages_index" */))

// TODO: remove in Nuxt 3
const emptyFn = () => {}
const originalPush = Router.prototype.push
Router.prototype.push = function push (location, onComplete = emptyFn, onAbort) {
  return originalPush.call(this, location, onComplete, onAbort)
}

Vue.use(Router)

export const routerOptions = {
  mode: 'history',
  base: decodeURI('/'),
  linkActiveClass: 'nuxt-link-active',
  linkExactActiveClass: 'nuxt-link-exact-active',
  scrollBehavior,

  routes: [{
    path: "/add",
    component: _c49b82a4,
    name: "add"
  }, {
    path: "/edit",
    component: _77478366,
    name: "edit",
    props:true
  }, {
    path: "/impressum",
    component: _68a0afc4,
    name: "impressum"
  }, {
    path: "/",
    component: _08e3451f,
    name: "index"
  }],

  fallback: false
}

export function createRouter () {
  return new Router(routerOptions)
}
