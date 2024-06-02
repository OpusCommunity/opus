import { isEnabled, enable, disable } from '@tauri-apps/plugin-autostart';

export async function is_autostart_enabled(): Promise<boolean> {
  return await isEnabled();
}

export async function enable_autostart(): Promise<void> {
  return await enable();
}

export async function disable_autostart(): Promise<void> {
  return await disable();
}
