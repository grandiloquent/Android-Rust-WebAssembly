# 视频下载工具

一个主要使用 Rust 编写的，用于获取视频文件地址和下载视频的 Android 软件。

## 编译

在 \Plane\app\src\main\rust 目录内打开终端。或者使用 Set-Location 将当前目录切换到目录，然后执行如下命令，将 Rust 项目编译成 .so 共享库。

通过 -o 选项设置编译后文件的保存地址。在这里 \Plane\app\src\main\jniLibs 是 Android 项目默认存放共享库的目录。在此目录中的文件将被自动打包到最终的 apk 文件中。

```
Set-Location  \Plane\app\src\main\rust;cargo ndk -t arm64-v8a --platform 31 -o \Plane\app\src\main\jniLibs build --release
```

## 支持的视频平台

|平台|解析|
|---|---|
|https://eroticmv.com/|是|
|https://www.mahua11.com/|是|
|https://91porn.com/|是|
|https://www.xvideos.com/|是|
|https://52ck.cc/|是|
|https://jable.tv|是|

使用方法

大部分视频平台都使用 Cookie 保护视频资源。如果出现视频无法解析或播放的问题，请尝试用该程序内嵌的简易浏览器打开视频的原始网页，刷新1次或完成相应的验证后，再次尝试。