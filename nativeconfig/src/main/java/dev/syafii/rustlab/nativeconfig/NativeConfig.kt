package dev.syafii.rustlab.nativeconfig

object NativeConfig {
  init {
    System.loadLibrary("nativeconfig")
  }

  external fun getBaseUrl(): String
  external fun clearBaseUrl()
  external fun getPreferenceName() : String

  external fun setToken(token: String)
  external fun getToken(): String
  external fun clearToken()
}