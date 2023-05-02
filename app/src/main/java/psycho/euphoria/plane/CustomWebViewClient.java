package psycho.euphoria.plane;

import android.util.Log;
import android.webkit.CookieManager;
import android.webkit.WebResourceRequest;
import android.webkit.WebResourceResponse;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.widget.Toast;

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
        }
    }




    @Override
    @SuppressWarnings("deprecation")
    public WebResourceResponse shouldInterceptRequest(WebView view, String url) {
        return super.shouldInterceptRequest(view, url);

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