package dev.syafii.rustlab

import android.os.Bundle
import android.util.Log
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.ViewCompat
import androidx.core.view.WindowInsetsCompat
import dev.syafii.rustlab.databinding.ActivityMainBinding
import dev.syafii.rustlab.nativeconfig.NativeConfig

class MainActivity : AppCompatActivity() {
  private val binding by lazy { ActivityMainBinding.inflate(layoutInflater) }

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
//    enableEdgeToEdge()
    setContentView(binding.root)
    /*ViewCompat.setOnApplyWindowInsetsListener(findViewById(R.id.main)) { v, insets ->
      val systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars())
      v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom)
      insets
    }*/
    initView()
  }

  private fun initView() = with(binding) {
    val baseUrl = NativeConfig.getBaseUrl()
    val prefName = NativeConfig.getPreferenceName()
    val token = NativeConfig.getToken()

    val displayText = """
        Base URL: $baseUrl
        Preference Name: $prefName
        Bearer Token: $token
    """.trimIndent()

    tvBaseUrl.text = displayText

    Log.d("RustJNI", "Base URL: $baseUrl")
    Log.d("RustJNI", "Bearer Token: $token")
    Log.d("RustJNI", "Preference Name: $prefName")
  }
}