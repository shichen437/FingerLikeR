import { check } from "@tauri-apps/plugin-updater";
import { getVersion } from "@tauri-apps/api/app";

export async function getLatestVersion(): Promise<string> {
  try {
    const update = await check();
    console.log("Update object:", update);

    if (update?.version) {
      return update.version;
    }

    return await getVersion();
  } catch (error) {
    return await getVersion();
  }
}
