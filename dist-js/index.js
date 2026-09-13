import { invoke } from '@tauri-apps/api/core';

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
async function isEnabled() {
    return await invoke('plugin:autostart|is_enabled');
}
async function enable() {
    await invoke('plugin:autostart|enable');
}
async function disable() {
    await invoke('plugin:autostart|disable');
}

export { disable, enable, isEnabled };
