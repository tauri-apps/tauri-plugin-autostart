'use strict';

var primitives = require('@tauri-apps/api/primitives');

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
async function isEnabled() {
    return await primitives.invoke("plugin:autostart|is_enabled");
}
async function enable() {
    await primitives.invoke("plugin:autostart|enable");
}
async function disable() {
    await primitives.invoke("plugin:autostart|disable");
}

exports.disable = disable;
exports.enable = enable;
exports.isEnabled = isEnabled;
