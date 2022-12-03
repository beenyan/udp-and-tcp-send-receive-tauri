import { createRouter, createWebHistory } from 'vue-router';

let history = createWebHistory();
let routes = [
  {
    path: '/',
    redirect: '/tcp/port'
  },
  {
    path: '/tcp',
    component: () => import('./views/Tcp.vue'),
    children: [
      {
        path: 'port',
        component:  () => import('./components/tcp/Port.vue'),
      },
      {
        path: 'data',
        component:  () => import('./components/tcp/Send.vue'),
      },
      {
        path: 'receive',
        component:  () => import('./components/tcp/Receive.vue'),
      },
    ],
  },
  {
    path: '/udp',
    component: () => import('./views/Udp.vue'),
    children: [
      {
        path: 'port',
        component:  () => import('./components/udp/Port.vue'),
      },
      {
        path: 'data',
        component:  () => import('./components/udp/Send.vue'),
      },
      {
        path: 'receive',
        component:  () => import('./components/udp/Receive.vue'),
      },
    ],
  },
  {
    path: '/sctp',
    component: () => import('./views/Sctp.vue'),
    children: [
      {
        path: 'port',
        component:  () => import('./components/sctp/Port.vue'),
      },
      {
        path: 'data',
        component:  () => import('./components/sctp/Send.vue'),
      },
      {
        path: 'receive',
        component:  () => import('./components/sctp/Receive.vue'),
      },
    ],
  },
]

export default createRouter({ history, routes });