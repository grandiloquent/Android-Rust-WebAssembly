package psycho.euphoria.plane;

import android.app.Activity;

public class MainActivity extends Activity {

    static {
/*
加载编译Rust代码后得到共享库。它完整的名称为librust.so
  */
        System.loadLibrary("rust");
    }

   
}