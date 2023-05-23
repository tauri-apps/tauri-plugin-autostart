declare global {
    interface Window {
        __TAURI_INVOKE__: <T>(cmd: string, args?: unknown) => Promise<T>;
    }
}
export declare function isEnabled(): Promise<boolean>;
export declare function enable(): Promise<void>;
export declare function disable(): Promise<void>;
