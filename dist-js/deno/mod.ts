import { invoke } from "https://esm.sh/@tauri-apps/api@1.5.0/tauri";

export async function get(name: string) {
    return await invoke("plugin:env-var|get", { name }) as string;
}

export async function getPermission(name: string) {
    return await invoke("plugin:env-var|get_permission", { name }) as string;
}

export async function set(name: string, value: string) {
    await invoke("plugin:env-var|set", { name, value });
}

