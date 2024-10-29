import { addPluginListener, invoke, PluginListener } from '@tauri-apps/api/core'

/** A ping function that lets you check that the plugin is alive.
*/
export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:sharetarget|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

/** A ShareEvent intent imported from Android. Note that all Bundle attachments are dropped.
 * Please fork and adapt to your liking.
*/
export type ShareEvent = {
    uri: string,
}

/** Use callback on all ShareEvent intents sent by Android.
*/
export async function listenForShareEvents(
    callback: (event: ShareEvent) => void
): Promise<PluginListener> {
    return await addPluginListener('sharetarget', 'share', callback);
}
