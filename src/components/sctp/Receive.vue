<script setup lang="ts">
import { socketMainStore } from "../../stores/state";

const socket = socketMainStore();

</script>

<template>
    <div class="card">
        <div v-for="res of socket.sctp.data" :key="res.time">
            <div class="time">{{ new Date(res.time).toLocaleString() }}</div>

            <div class="content">
                <p class="text" v-for="text of res.content.split('\n')">{{ text }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    flex-grow: 1;
    text-align: left;
    padding-left: 5px;
    row-gap: 10px;
    overflow-y: auto;

    .time {
        color: gray;
        font-size: 14px;
    }

    .content {
        margin-left: 15px;

        .text {
            margin: 0;
        }
    }
}
</style>
