# Tauri Plugin share-target
A plugin for Tauri applications to appear as a share target under Android.
Behaviour on other platforms is indeterminate. Support is limited to the [`Intent`](https://developer.android.com/reference/android/content/Intent)'s
URI, attached `Bundle`s are dropped. You were warned.

## Installation
In `src-tauri/Cargo.toml` :
``` toml 
[dependencies]
tauri-plugin-share-target = 0.1.0
```
In `src-tauri/src/lib.rs`, add the plugin entry :
``` rust 
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_share_target::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
When you first build for Android, some files get generated. To signal your app
as a share target to Android, you need to modify your [`AndroidManifest.xml`](https://developer.android.com/guide/topics/manifest/manifest-intro).
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

## Authorization
First you need permissions, just to get ipc events in javascript.
In `src-tauri/capabilities/default.json`, add `share-target` to the permissions :
``` json
{
    "$schema": "../gen/schemas/desktop-schema.json",
    "identifier": "anything_you_like",
    "windows": ["main"],
    "permissions": [
        ...
        "share-target:default"
    ]
}
```

## Usage
For example in React, in `src/main.tsx` :
``` tsx
import { useEffect, useState } from "react";
import { addPluginListener } from '@tauri-apps/api/core';

function App() {
    const [logs, setLogs] = useState("");
    useEffect(() => {
        let listener: PluginListener;
        addPluginListener(
            'share-target',
            'share',
            (event: { url: string }) => { setLogs(event.url) }
        ).then((l: PluginListener) => {
            listener = l;
        };
        return () => { listener?.unregister() };
    });
    return (<>
        <h3>Share this</h3>
        <p>{ logs }</p>
        <button onClick={ yourCallbackFunction }>share</button>
    </>);
}
```
