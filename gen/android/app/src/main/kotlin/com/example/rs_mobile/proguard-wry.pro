# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class com.example.rs_mobile.* {
  native <methods>;
}

-keep class com.example.rs_mobile.WryActivity {
  public <init>(...);

  void setWebView(com.example.rs_mobile.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class com.example.rs_mobile.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class com.example.rs_mobile.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void setAutoPlay(...);
  void setUserAgent(...);
  void evalScript(...);
}

-keep class com.example.rs_mobile.RustWebChromeClient,com.example.rs_mobile.RustWebViewClient {
  public <init>(...);
}