import { load, Store } from "@tauri-apps/plugin-store";

let r_store: Store;

export async function setStoreValue(key: string, value: any) {
  await r_store.set(key, value);
  await r_store.save();
}

export async function getStoreValue<T>(
  key: string,
  defaultValue: T,
): Promise<T> {
  const stored = await r_store.get<T>(key);
  return stored ?? defaultValue;
}

export async function initializeStore() {
  r_store = await load("r_store.json");
}
