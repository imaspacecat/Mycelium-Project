'use client'

import { invoke } from '@tauri-apps/api/tauri'

function StartNetworkTableHandler(): void {
    invoke('start_network_table_handler', { })
        .catch(console.error)
}

// TODO: backend must be implemented
function StopNetworkTableHandler(): void {
    invoke('stop_network_table_handler', { })
        .catch(console.error)
}

// TODO: backend must be implemented
function Subscribe(topic: String, periodic?: number, all?: boolean, prefix?: String): void {
    invoke('subscribe_to_topic', { topic, periodic, all, prefix })
        .catch(console.error)
}

// TODO: backend must be implemented
function Publish(topic: String, message: String): void {
    invoke('publish_topic', { topic, message })
        .catch(console.error)
}