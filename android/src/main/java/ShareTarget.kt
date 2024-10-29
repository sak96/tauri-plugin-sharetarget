package app.tauri.share-target

import android.util.Log

class ShareTarget {
    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }
}
