package com.astream.app

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import kotlinx.coroutines.*
import org.java_websocket.client.WebSocketClient
import org.java_websocket.handshake.ServerHandshake
import org.webrtc.*
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.net.URI

class MainActivity : AppCompatActivity() {
    private val TAG = "AStream"
    private val PERMISSION_REQUEST_CODE = 1
    private val REQUIRED_PERMISSIONS = arrayOf(
        Manifest.permission.RECORD_AUDIO,
        Manifest.permission.INTERNET
    )

    private var peerConnectionFactory: PeerConnectionFactory? = null
    private var peerConnection: PeerConnection? = null
    private var audioSource: AudioSource? = null
    private var localAudioTrack: AudioTrack? = null
    private var pcIpAddress: String = ""
    private var webSocketClient: WebSocketClient? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        if (!hasPermissions()) {
            requestPermissions()
        } else {
            initializeWebRTC()
            setupUI()
            discoverPC()
        }
    }

    private fun hasPermissions(): Boolean {
        return REQUIRED_PERMISSIONS.all {
            ContextCompat.checkSelfPermission(this, it) == PackageManager.PERMISSION_GRANTED
        }
    }

    private fun requestPermissions() {
        ActivityCompat.requestPermissions(this, REQUIRED_PERMISSIONS, PERMISSION_REQUEST_CODE)
    }

    private fun initializeWebRTC() {
        Log.d(TAG, "Initializing WebRTC...")
        val options = PeerConnectionFactory.InitializationOptions.builder(this)
            .createInitializationOptions()
        PeerConnectionFactory.initialize(options)

        peerConnectionFactory = PeerConnectionFactory.builder()
            .setOptions(PeerConnectionFactory.Options())
            .createAudioDeviceModule(this)
            .build()

        Log.d(TAG, "WebRTC initialized")
    }

    private fun setupUI() {
        findViewById<Button>(R.id.btn_connect).setOnClickListener {
            if (pcIpAddress.isNotEmpty()) {
                connectToPC()
            } else {
                Toast.makeText(this, "PC not found. Discovering...", Toast.LENGTH_SHORT).show()
                discoverPC()
            }
        }
    }

    private fun discoverPC() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val socket = DatagramSocket()
                socket.soTimeout = 2000
                socket.broadcast = true

                val discoverMsg = "ASTREAM_DISCOVER".toByteArray()
                val broadcastAddr = InetAddress.getByName("255.255.255.255")
                val packet = DatagramPacket(discoverMsg, discoverMsg.size, broadcastAddr, 5353)
                socket.send(packet)
                Log.d(TAG, "Sent discovery broadcast")

                try {
                    val buf = ByteArray(1024)
                    val response = DatagramPacket(buf, buf.size)
                    socket.receive(response)
                    val msg = String(response.data, 0, response.length)
                    Log.d(TAG, "Discovery response: $msg")

                    if (msg.startsWith("ASTREAM_RESPONSE")) {
                        pcIpAddress = response.address.hostAddress
                        withContext(Dispatchers.Main) {
                            Toast.makeText(this@MainActivity, "Found PC: $pcIpAddress", Toast.LENGTH_SHORT).show()
                        }
                    }
                } catch (e: Exception) {
                    Log.e(TAG, "Discovery timeout or error: ${e.message}")
                }
                socket.close()
            } catch (e: Exception) {
                Log.e(TAG, "Discovery failed: ${e.message}")
            }
        }
    }

    private fun connectToPC() {
        Log.d(TAG, "Connecting to PC at $pcIpAddress:8080")
        Toast.makeText(this, "Connecting to $pcIpAddress...", Toast.LENGTH_SHORT).show()

        val wsUri = URI("ws://$pcIpAddress:8080")
        webSocketClient = object : WebSocketClient(wsUri) {
            override fun onOpen(handshakedata: ServerHandshake?) {
                Log.d(TAG, "WebSocket connected")
                createOffer()
            }

            override fun onMessage(message: String?) {
                Log.d(TAG, "Received: $message")
                message?.let { handleSignalingMessage(it) }
            }

            override fun onClose(code: Int, reason: String?, remote: Boolean) {
                Log.d(TAG, "WebSocket closed: $reason")
            }

            override fun onError(ex: Exception?) {
                Log.e(TAG, "WebSocket error: ${ex?.message}")
            }
        }
        webSocketClient?.connect()
    }

    private fun createOffer() {
        val iceServers = listOf(PeerConnection.IceServer.builder("stun:stun.l.google.com:19302").createIceServer())
        val config = PeerConnection.RTCConfiguration(iceServers)

        peerConnection = peerConnectionFactory?.createPeerConnection(config, object : PeerConnection.Observer {
            override fun onIceCandidate(p0: IceCandidate?) {
                p0?.let {
                    val msg = "{ \"type\": \"IceCandidate\", \"candidate\": \"${it.sdp}\", \"sdpMid\": \"${it.sdpMid}\", \"sdpMLineIndex\": ${it.sdpMLineIndex} }"
                    webSocketClient?.send(msg)
                }
            }

            override fun onAddStream(p0: MediaStream?) {}
            override fun onDataChannel(p0: DataChannel?) {}
            override fun onIceConnectionChange(p0: PeerConnection.IceConnectionState?) {}
            override fun onIceGatheringChange(p0: PeerConnection.IceGatheringState?) {}
            override fun onRemoveStream(p0: MediaStream?) {}
            override fun onRenegotiationNeeded() {}
            override fun onSignalingChange(p0: PeerConnection.SignalingState?) {}
            override fun onTrack(p0: RtpTransceiver?) {}
        })

        val audioConstraints = MediaConstraints()
        audioSource = peerConnectionFactory?.createAudioSource(audioConstraints)
        localAudioTrack = peerConnectionFactory?.createAudioTrack("audio0", audioSource)
        peerConnection?.addTrack(localAudioTrack)

        peerConnection?.createOffer(object : SdpObserver {
            override fun onCreateSuccess(p0: SessionDescription?) {
                p0?.let {
                    peerConnection?.setLocalDescription(this, it)
                    val msg = "{ \"type\": \"Offer\", \"sdp\": \"${it.description}\" }"
                    webSocketClient?.send(msg)
                }
            }
            override fun onSetSuccess() {}
            override fun onCreateFailure(p0: String?) {}
            override fun onSetFailure(p0: String?) {}
        }, MediaConstraints())
    }

    private fun handleSignalingMessage(message: String) {
        // TODO: Parse and handle SDP answer, ICE candidates
        Log.d(TAG, "Handling signaling message")
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == PERMISSION_REQUEST_CODE) {
            if (grantResults.all { it == PackageManager.PERMISSION_GRANTED }) {
                initializeWebRTC()
                setupUI()
                discoverPC()
            } else {
                Log.e(TAG, "Permissions not granted")
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        webSocketClient?.close()
        peerConnection?.dispose()
        peerConnectionFactory?.dispose()
    }
}
