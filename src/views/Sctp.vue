<script setup lang="ts">
</script>

<template>
    <div class="box">
        <div class="option-group">
            <router-link class="btn" to="./port">監聽 Port</router-link>
            <router-link class="btn" to="./data">傳送 Data</router-link>
            <router-link class="btn" to="./receive">接收 Data</router-link>
        </div>

        <router-view />
    </div>
</template>

<style scoped lang="scss">
.box {
    display: flex;
    height: 100%;
    column-gap: 5px;

    .option-group {
        display: flex;
        flex-direction: column;
        row-gap: 10px;
        border-right: 1px solid black;
        padding-right: 5px;
        min-width: 120px;

        .btn {
            background-color: #2A2A2A;
            box-shadow: unset;
        }
    }
}

@media (prefers-color-scheme: dark) {}
</style>
