package app.tauri.shared

import android.util.Log

class Shared {
    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }
}
