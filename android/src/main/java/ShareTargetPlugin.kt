package app.tauri.sharetarget

import android.app.Activity
import android.net.Uri
import android.os.Parcelable
import android.content.Intent
import android.content.Context
import android.provider.OpenableColumns
import android.util.Log
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Channel
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.io.File

@InvokeArg
class PingArgs {
    var value: String? = null
}

@TauriPlugin
class ShareTargetPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = ShareTarget()

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve(ret)
    }

    /// Send all new intents to registered listeners.
    override fun onNewIntent(intent: Intent) {
        if (intent.action == Intent.ACTION_SEND) {
            val payload = intentToJson(intent)
            val targetUri = intent.getParcelableExtra<Parcelable>(Intent.EXTRA_STREAM).toString()
            val name = getNameFromUri(activity.applicationContext, Uri.parse(targetUri))
            if (name != null && name != "") {
                payload.put("name", name)
                Log.i("got name", name)
            }
            Log.i("triggering event", payload.toString())
            trigger("share", payload)
        }
    }
}

fun intentToJson(intent: Intent): JSObject {
    val json = JSObject()
    Log.i("processing", intent.toUri(0))
    json.put("uri", intent.toUri(0))
    json.put("content_type", intent.type)
    val streamUrl = intent.extras?.get("android.intent.extra.STREAM")
    if (streamUrl != null) {
        json.put("stream", streamUrl)
    }
    /*
        }
    }
    */
    return json
}
 fun getNameFromUri(context: Context, uri: Uri): String? {
    var displayName: String? = ""
    val projection = arrayOf(OpenableColumns.DISPLAY_NAME)
    val cursor =
        context.contentResolver.query(uri, projection, null, null, null)
    if (cursor != null) {
        cursor.moveToFirst()
        val columnIdx = cursor.getColumnIndex(projection[0])
        displayName = cursor.getString(columnIdx)
        cursor.close()
    }
    if (displayName.isNullOrEmpty()) {
        displayName = uri.lastPathSegment
    }
    return displayName
}
