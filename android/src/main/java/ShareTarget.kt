package app.tauri.shareTarget

import android.util.Log

class ShareTarget {
    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }
}
