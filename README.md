# Tauri Plugin sharetarget
A plugin for Tauri applications to appear as a share target under Android.
Behaviour on other platforms is indeterminate. It doesn't support multi-selection
share at the moment.

## Installation
In `src-tauri/Cargo.toml` :
``` toml 
[dependencies]
tauri-plugin-sharetarget = 0.1.1
```
In `src-tauri/src/lib.rs`, add the plugin entry :
``` rust 
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sharetarget::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
To build for Android, you must first `tauri android init` successfully. This gets some files
generated. To signal your app as a share target to Android, you then need to modify your
[`AndroidManifest.xml`](https://developer.android.com/guide/topics/manifest/manifest-intro).
In `src-tauri/gen/android/app/src/main/AndroidManifest.xml`, add your `intent-filter`s :
``` xml
<?xml version="1.0" encoding="utf-8">
<manifest ...>
    ...
    <application ...>
        ...
        <activity ...>
            <intent-filter>
                <!-- Support receiving share events. -->
                <action android:name="android.intent.action.SEND" />
                <category android:name="android.intent.category.DEFAULT" />
                <!-- You can scope any MIME type here. You'll see what Intent Android returns. -->
                <data android:mimeType="text/*" />
            </intent-filter>
        </activity ...>
        ...
```

## Permissions
First you need permissions in tauri, just to get ipc events in javascript.
In `src-tauri/capabilities/default.json`, add `sharetarget` to the permissions :
``` json
{
    "$schema": "../gen/schemas/desktop-schema.json",
    "identifier": "anything_you_like",
    "windows": ["main"],
    "permissions": [
        ...
        "sharetarget:default"
    ]
}
```

## Usage
Use the provided API in javascript/typescript. For example in React, in `src/main.tsx` :
``` tsx
import { useEffect, useState } from 'react';
import { listenForShareEvents, type ShareEvent } from 'tauri-plugin-sharetarget';
import { PluginListener } from '@tauri-apps/api/core';

function App() {
    const [logs, setLogs] = useState('');
    useEffect(() => {
        // Twisted initialization to satisfy returning a sync destructor (React+ts demand).
        let listener: PluginListener;
        listenForShareEvents((intent: ShareEvent) => { setLogs(intent.uri); })
            .then((l: PluginListener) => { listener = l; });
        return () => { listener?.unregister(); };
    };
    return (<>
        <h3>Share this</h3>
        <p>{ logs }</p>
        <button onClick={ yourCallbackFunction }>share</button>
    </>);
}
```

### Receive attached stream (images, etc)
To receive shared images, you need
  - an `intent` targeting `image/*`
  - `tauri-plugin-fs` to read sent data
  - add `fs:default` to the capabilities of your app
  - in javascript, use `readFile()` from `@tauri-apps/plugin-fs` on the
  intent's `stream`

`readFile()` will return binary data, you just need to process it.

Unfortunately, multiple shares are not supported right now. PR welcome !

