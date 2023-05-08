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
import android.net.wifi.WifiInfo;
import android.net.wifi.WifiManager;
import android.os.Environment;
import android.os.IBinder;
import android.os.Process;
import android.preference.PreferenceManager;
import android.util.Log;

import java.io.File;
import java.io.IOException;
import java.lang.reflect.Method;
import java.net.InetAddress;
import java.net.NetworkInterface;
import java.net.ServerSocket;
import java.net.SocketException;
import java.net.UnknownHostException;
import java.util.Enumeration;

/*
https://developer.android.com/reference/android/content/Intent

*/

public class ServerService extends Service {
    public static final String ACTION_DISMISS = "psycho.euphoria.plane.ServerService.ACTION_DISMISS";
    public static final String KP_NOTIFICATION_CHANNEL_ID = "notification_channel";
    public static final String START_SERVER_ACTION = "psycho.euphoria.plane.MainActivity.startServer";

    static {
        // 加载 Rust 共享库。它的完整名称
        // 为 librust.so
        System.loadLibrary("rust");
    }

    SharedPreferences mSharedPreferences;

    public static void createNotification(ServerService context, String address) {
        PendingIntent piDismiss = getPendingIntentDismiss(context);
        Intent intent = new Intent(context, MainActivity.class);
        intent.putExtra("address", address);
        intent.setFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        Notification notification = new Notification.Builder(context, KP_NOTIFICATION_CHANNEL_ID).setContentTitle("本地服务器").setSmallIcon(android.R.drawable.stat_sys_download).addAction(getAction(piDismiss))
                .setContentIntent(PendingIntent.getActivity(context, 0, intent, PendingIntent.FLAG_IMMUTABLE))
                .build();
        context.startForeground(1, notification);
    }

    public static void createNotificationChannel(Context context) {
        NotificationChannel notificationChannel = new NotificationChannel(KP_NOTIFICATION_CHANNEL_ID, "本地服务器", NotificationManager.IMPORTANCE_LOW);
        ((NotificationManager) context.getSystemService(Context.NOTIFICATION_SERVICE)).createNotificationChannel(notificationChannel);
    }

    public static Notification.Action getAction(PendingIntent piDismiss) {
        return new Notification.Action.Builder(null, "关闭", piDismiss).build();
    }

    // 获取连接 WIFI 时被分配的局域网 
    // IP，如果手机没有连接 WIFI，将
    // 返回 0.0.0.0，在这种情况下，可
    // 以打开手机热点，然后通过连接该热点进
    // 行数据交互
    public static String getDeviceIP(Context context) {
        WifiManager wifiManager = (WifiManager) context.getApplicationContext().getSystemService(Context.WIFI_SERVICE);
        try {
            WifiInfo wifiInfo = wifiManager.getConnectionInfo();
            int rawIp = wifiInfo.getIpAddress();
            if (rawIp == 0) {
                Method method = wifiManager.getClass().getDeclaredMethod("isWifiApEnabled");
                method.setAccessible(true);
                boolean isWifiApEnabled = (boolean) method.invoke(wifiManager);
                if (isWifiApEnabled)
                    return getWifiApIpAddress();
                else
                    return "0.0.0.0";
            }
            //Log.e("B5aOx2", String.format("getDeviceIP, %s", wifiManager.getConnectionInfo().getSupplicantState().name()));
            InetAddress inetAddress = intToInetAddress(rawIp);
            return inetAddress.getHostAddress();
        } catch (Exception e) {
            return "0.0.0.0";
        }
    }

    public static PendingIntent getPendingIntentDismiss(Context context) {
        Intent dismissIntent = new Intent(context, ServerService.class);
        dismissIntent.setAction(ACTION_DISMISS);
        return PendingIntent.getService(context, 0, dismissIntent, PendingIntent.FLAG_IMMUTABLE);
    }

    public String getString(String key) {
        return mSharedPreferences.getString(key, "");
    }

    // 获取从指定数值开始第一个空闲的端口
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

    public static String getWifiApIpAddress() {
        try {
            for (Enumeration<NetworkInterface> en = NetworkInterface.getNetworkInterfaces(); en
                    .hasMoreElements(); ) {
                NetworkInterface intf = en.nextElement();
                if (intf.getName().contains("wlan")) {
                    for (Enumeration<InetAddress> enumIpAddr = intf.getInetAddresses(); enumIpAddr
                            .hasMoreElements(); ) {
                        InetAddress inetAddress = enumIpAddr.nextElement();
                        if (!inetAddress.isLoopbackAddress()
                                && (inetAddress.getAddress().length == 4)) {
                            return inetAddress.getHostAddress();
                        }
                    }
                }
            }
        } catch (SocketException ex) {
        }
        return null;
    }

    public static InetAddress intToInetAddress(int hostAddress) {
        byte[] addressBytes = {(byte) (0xff & hostAddress),
                (byte) (0xff & (hostAddress >> 8)),
                (byte) (0xff & (hostAddress >> 16)),
                (byte) (0xff & (hostAddress >> 24))};
        try {
            return InetAddress.getByAddress(addressBytes);
        } catch (UnknownHostException e) {
            throw new AssertionError();
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
                .putString("host", getDeviceIP(this))
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
        createNotificationChannel(this);
        mAddress = startServer(this, getAssets());
        Log.e(TAG, mAddress);
        createNotification(this, mAddress);
        launchActivity();
    }

    private final String TAG = "TAG/" + getClass().getSimpleName();

    String mAddress;

    // 当该服务创建并启动服务器后，通过广播
    // 将服务器的地址发送给侦听该特定的 Activity，
    // 在该Activity接受到广播后，它
    // 可以加载该地址。
    private void launchActivity() {
        Intent intent = new Intent(START_SERVER_ACTION);
        intent.putExtra("address", mAddress);
        sendBroadcast(intent);
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        // 彻底关闭该服务，防止系统重启该服务
        if (intent != null && intent.getAction() != null && intent.getAction().equals(ACTION_DISMISS)) {
            stopForeground(true);
            stopSelf();
            // 通过当前程序进程ID，终止该程序
            Process.killProcess(Process.myPid());
            return START_NOT_STICKY;
        }
        return super.onStartCommand(intent, flags, startId);
    }
}