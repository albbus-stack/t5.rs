# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class com.example.t5_rs.* {
  native <methods>;
}

-keep class com.example.t5_rs.WryActivity {
  public <init>(...);

  void setWebView(com.example.t5_rs.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class com.example.t5_rs.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class com.example.t5_rs.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void setAutoPlay(...);
  void setUserAgent(...);
  void evalScript(...);
}

-keep class com.example.t5_rs.RustWebChromeClient,com.example.t5_rs.RustWebViewClient {
  public <init>(...);
}