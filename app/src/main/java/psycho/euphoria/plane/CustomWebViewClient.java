package psycho.euphoria.plane;

import android.util.Log;
import android.webkit.CookieManager;
import android.webkit.WebResourceRequest;
import android.webkit.WebResourceResponse;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.Toast;

import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

public class CustomWebViewClient extends WebViewClient {


    private final MainActivity mContext;

    public CustomWebViewClient(MainActivity context) {
        mContext = context;
    }


    @Override
    public void onPageFinished(WebView view, String url) {
        String cookie;
        if (url.contains("/vodplay/") && (cookie = CookieManager.getInstance().getCookie(url)) != null) {
            new Database(mContext).insertCookie(5, cookie);
            Toast.makeText(mContext, "成功", Toast.LENGTH_SHORT).show();
        } else if (url.contains("jable.tv/") && (cookie = CookieManager.getInstance().getCookie(url)) != null) {
            new Database(mContext).insertCookie(7, cookie);
            Toast.makeText(mContext, "成功", Toast.LENGTH_SHORT).show();
        } else if (url.contains("mahua11.com/") && (cookie = CookieManager.getInstance().getCookie(url)) != null) {
            new Database(mContext).insertCookie(6, cookie);
            Toast.makeText(mContext, "成功", Toast.LENGTH_SHORT).show();
        }
    }

    public static class OptionsAllowResponse {
        static final SimpleDateFormat formatter = new SimpleDateFormat("E, dd MMM yyyy kk:mm:ss", Locale.US);

        static WebResourceResponse build() {
            Date date = new Date();
            final String dateString = formatter.format(date);

            Map<String, String> headers = new HashMap<String, String>() {{
                put("Connection", "close");
                put("Content-Type", "text/plain");
                put("Date", dateString + " GMT");
                put("Access-Control-Allow-Origin", "*");
                put("Access-Control-Allow-Methods", "GET, POST, DELETE, PUT, OPTIONS");
                put("Access-Control-Max-Age", "600");
                put("Access-Control-Allow-Credentials", "true");
                put("Access-Control-Allow-Headers", "accept, authorization, Content-Type");
                put("Via", "1.1 vegur");
            }};

            return new WebResourceResponse("text/plain", "UTF-8", 200, "OK", headers, null);
        }
    }

    @Override
    public WebResourceResponse shouldInterceptRequest(WebView view, WebResourceRequest request) {
        if (request.getMethod().equalsIgnoreCase("OPTIONS")) {
            return OptionsAllowResponse.build();
        }

        return null;
    }

    @Override
    public boolean shouldOverrideUrlLoading(WebView view, WebResourceRequest request) {

        String url = request.getUrl().toString();
        if ((url.startsWith("https://") || url.startsWith("http://") || url.startsWith("file://"))) {
            view.loadUrl(url);
        }
        return true;
    }

}