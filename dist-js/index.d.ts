/**
 * Checks whether autostart is enabled for the application.
 * @example
 * ```typescript
 * import { isEnabled } from '@tauri-apps/plugin-autostart';
 * const enabled = await isEnabled();
 * ```
 *
 * @returns A promise resolving to `true` if the application launches at startup, `false` otherwise.
 * @since 2.0.0
 */
export declare function isEnabled(): Promise<boolean>;
/**
 * Enables autostart for the application.
 * @example
 * ```typescript
 * import { enable } from '@tauri-apps/plugin-autostart';
 * await enable();
 * ```
 *
 * @since 2.0.0
 */
export declare function enable(): Promise<void>;
/**
 * Disables autostart for the application.
 * @example
 * ```typescript
 * import { disable } from '@tauri-apps/plugin-autostart';
 * await disable();
 * ```
 *
 * @since 2.0.0
 */
export declare function disable(): Promise<void>;
