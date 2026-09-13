'use strict';

var core = require('@tauri-apps/api/core');

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
async function isEnabled() {
    return await core.invoke('plugin:autostart|is_enabled');
}
async function enable() {
    await core.invoke('plugin:autostart|enable');
}
async function disable() {
    await core.invoke('plugin:autostart|disable');
}

exports.disable = disable;
exports.enable = enable;
exports.isEnabled = isEnabled;
