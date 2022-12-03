import { defineStore } from 'pinia'

interface Result {
    content: string,
    time: number
}

export const socketMainStore = defineStore({
    id: 'socket',
    state: () => ({
        tcp: {
            port: 0,
            isListen: false,
            data: [] as Result[]
        },
        udp: {
            port: 0,
            isListen: false,
            data: [] as Result[]
        },
        sctp: {
            port: 0,
            isListen: false,
            data: [] as Result[]
        },
    }),
})