<script setup lang="ts">
import { socketMainStore } from "./stores/state"
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
const socket = socketMainStore();

interface Payload {
  message: string,
}

const tcp_listen = async () => {
  invoke('tcp_event');

  return await listen<Payload>('tcp-event', (event) => {
    const { message } = event.payload;
    socket.tcp.data.push({
      content: message,
      time: +new Date()
    })
  })
}
tcp_listen();

const udp_listen = async () => {
  invoke('udp_event');

  return await listen<Payload>('udp-event', (event) => {
    const { message } = event.payload;
    socket.udp.data.push({
      content: message,
      time: +new Date()
    })
  })
}
udp_listen();

const sctp_listen = async () => {
  invoke('sctp_event');

  return await listen<Payload>('sctp-event', (event) => {
    const { message } = event.payload;
    socket.sctp.data.push({
      content: message,
      time: +new Date()
    })
  })
}
sctp_listen();
</script>

<template>
  <div class="container">
    <div class="socket-type">
      <router-link class="btn" to="/tcp/port" replace>TCP
        <div class="state" :class="{ active: socket.tcp.isListen }"></div>
      </router-link>

      <router-link class="btn" to="/udp/port" replace>
        UDP
        <div class="state" :class="{ active: socket.udp.isListen }"></div>
      </router-link>

      <router-link :disabled="true" class="btn" to="/sctp/port" replace>
        SCTP
        <div class="state" :class="{ active: socket.sctp.isListen }"></div>
      </router-link>
    </div>

    <div class="info">
      <router-view />
    </div>
  </div>
</template>

<style scoped lang="scss">
.socket-type {
  display: flex;
  justify-content: space-between;

  .btn {
    display: flex;
    align-items: center;
    column-gap: 10px;

    .state {
      background-color: gray;
      display: inline-block;
      border-radius: 50%;
      height: 7px;
      width: 7px;
    }

    .active {
      background-color: greenyellow;
    }
  }
}

.info {
  padding: 15px 10px;
  border-radius: 0.5rem;
  height: 100%;
  box-shadow: 0.5rem 0.5rem 0.5rem #00000080;
  max-height: 80%;
}

@media (prefers-color-scheme: dark) {
  .info {
    background-color: #3a3a3a;
  }
}
</style>
