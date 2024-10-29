package app.tauri.shareTarget

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.content.Intent
import android.util.Log

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

    override fun onNewIntent(intent: Intent) {
        if (intent?.action == Intent.ACTION_SEND) {
            val payload = intentToJson(intent)
            Log.i("triggering event", payload.toString())
            trigger("share", payload)
        }
    }
}

fun intentToJson(intent: Intent): JSObject {
    val json = JSObject()
    json.put("uri", intent.toUri(0))
    /*
    val bundle = intent.getBundle()
    val keys = bundle.keySet()
    for (key in keys) {
        try {
            json.put(key, bundle.get(key))
        } catch(e: JSException) {
            Log.e("could not serialize intent", e)
        }
    }
    */
    return json
}
