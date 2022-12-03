<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { socketMainStore } from "../../stores/state";
import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
const socket = socketMainStore();

const greetMsg = ref('');
const content = ref('');
const port = ref(socket.tcp.port);

async function send() {
    await invoke('tcp_send', { port: port.value, content: content.value })
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

async function read() {
    content.value = await readTextFile('data.txt', { dir: BaseDirectory.Resource });
}
read();

</script>

<template>
    <div class="card">
        <div class="content">
            <p class="note">讀取同目錄下的 data.txt</p>
            <p class="text" v-for="text of content.split('\n')">{{ text }}</p>
            <p class="note"> {{ greetMsg }} </p>
        </div>

        <div class="btn-group">
            <button type="button" class="btn" @click="read()">Read</button>
            <button type="button" class="btn" @click="send()" :disabled="!socket.tcp.isListen">Send</button>
        </div>
    </div>
</template>

<style scoped lang="scss">
.card {
    display: flex;
    flex-direction: row;
    column-gap: 10px;
    flex-grow: 1;
    justify-content: center;
    width: 100%;

    .content {
        padding: 0 10px;
        text-align: left;
        flex-grow: 1;

        .text {
            margin: 0;
        }

        .note {
            color: gray;
            font-size: 14px;
            margin: 0 0 1px 0;
        }
    }

    .btn-group {
        display: flex;
        flex-direction: column;
        row-gap: 10px;
    }
}
</style>
