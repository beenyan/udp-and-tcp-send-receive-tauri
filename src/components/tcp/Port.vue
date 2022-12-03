<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { socketMainStore } from "../../stores/state";
const socket = socketMainStore();

const greetMsg = ref('');
const port = ref(65530);

async function start() {
    if (!Number.isInteger(port.value) || port.value > 65535 || port.value < 0) {
        alert('port range in 0 ~ 65535.')
        return;
    }

    socket.tcp.isListen = true;
    socket.tcp.port = port.value;
    await invoke('tcp', { port: port.value })
        .then((message) => {
            console.log(message);
            greetMsg.value = message as string;
        })
        .catch((error) => {
            console.log(error);
            greetMsg.value = error as string;
            socket.tcp.isListen = false;
        })
}
</script>

<template>
    <div class="card">
        <input type="number" min="0" max="65535" v-model.number="port" placeholder="Enter a port..." />
        <button type="button" class="btn" @click="start()" :disabled="socket.tcp.isListen">Start</button>
        <p> {{ greetMsg }} </p>
    </div>
</template>

<style scoped lang="scss">
.card {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    row-gap: 10px;
    width: max-content;
    flex-grow: 1;

    input {
        min-width: 160px;
    }
}
</style>
