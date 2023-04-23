package psycho.euphoria.plane;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.app.Service;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.content.SharedPreferences.Editor;
import android.content.res.AssetManager;
import android.os.Environment;
import android.os.IBinder;
import android.preference.PreferenceManager;
import android.util.Log;

import java.io.File;
import java.io.IOException;
import java.net.ServerSocket;

public class ServerService extends Service {
    public static final String ACTION_DISMISS = "psycho.euphoria.plane.ServerService.ACTION_DISMISS";
    public static final String KP_NOTIFICATION_CHANNEL_ID = "notification_channel";

    static {
/*
加载编译Rust代码后得到共享库。它完整的名称为librust.so
  */
        System.loadLibrary("rust");
    }

    SharedPreferences mSharedPreferences;

    public static void createNotification(ServerService context) {
        Notification notification = null;
        PendingIntent piDismiss = getPendingIntentDismiss(context);
        notification = new Notification.Builder(context, KP_NOTIFICATION_CHANNEL_ID).setContentTitle("本地服务器").setSmallIcon(android.R.drawable.stat_sys_download).addAction(getAction(piDismiss)).build();
        context.startForeground(1, notification);
    }

    public static void createNotificationChannel(Context context) {
        NotificationChannel notificationChannel = new NotificationChannel(KP_NOTIFICATION_CHANNEL_ID, "本地服务器", NotificationManager.IMPORTANCE_LOW);
        ((NotificationManager) context.getSystemService(Context.NOTIFICATION_SERVICE)).createNotificationChannel(notificationChannel);
    }

    public static Notification.Action getAction(PendingIntent piDismiss) {
        return new Notification.Action.Builder(null, "关闭", piDismiss).build();
    }

    public static PendingIntent getPendingIntentDismiss(Context context) {
        Intent dismissIntent = new Intent(context, ServerService.class);
        dismissIntent.setAction(ACTION_DISMISS);
        return PendingIntent.getService(context, 0, dismissIntent, 0);
    }

    public String getString(String key) {
        return mSharedPreferences.getString(key, "");
    }

    public static int getUsablePort(int start) {
        while (true) {
            try {
                ServerSocket serverPort = new ServerSocket(start);
                serverPort.close();
                return start;
            } catch (IOException ignored) {
                start++;
            }
        }
    }

    public void setString(String key, String value) {
        mSharedPreferences.edit().putString(key, value).apply();
    }

    public static native String startServer(ServerService service, AssetManager assetManager);

    private void initialSharedPreferences() {
        File dir = new File(Environment.getExternalStorageDirectory(), ".plane");
        if (!dir.exists()) {
            dir.mkdirs();
        }
        Editor editor = mSharedPreferences.edit();
        if (mSharedPreferences.getString("temp_dir", null) == null) {
            editor.putString("temp_dir", dir.getAbsolutePath());
        }
        if (mSharedPreferences.getString("db", null) == null) {
            String defaultDatabase = new File(dir, "videos.db").getAbsolutePath();
            editor.putString("db", defaultDatabase);
        }
        editor.putString("port", Integer.toString(getUsablePort(3000)))
                .commit();
    }

    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }

    @Override
    public void onCreate() {
        super.onCreate();
        mSharedPreferences = PreferenceManager.getDefaultSharedPreferences(this);
        initialSharedPreferences();
        createNotification(this);
        String address = startServer(this, getAssets());
        Log.e("B5aOx2", String.format("onCreate, %s", address));
    }
}