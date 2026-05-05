# AStream (AudioStream)

A cross-platform high-performance audio streaming application for streaming audio between PC and Android with low latency using WebRTC.

## Features

- **Low Latency**: WebRTC-based audio streaming for minimal delay
- **Cross-Platform**: PC (Rust) and Android (Kotlin) support
- **Device Discovery**: Automatic PC discovery on local network via UDP broadcast
- **Bi-directional Audio**: Stream audio from PC to Android or vice versa

## Architecture

```
AStream/
├── pc/          # Rust PC application
│   ├── src/
│   │   ├── main.rs           # Entry point
│   │   ├── signaling.rs      # WebSocket signaling server
│   │   ├── webrtc_handler.rs # WebRTC peer connection management
│   │   ├── audio.rs         # Audio capture/playback (stub)
│   │   └── discovery.rs     # UDP device discovery
│   └── Cargo.toml
│
├── android/     # Android application
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── java/com/astream/app/MainActivity.kt
│   │   │   ├── res/layout/activity_main.xml
│   │   │   └── AndroidManifest.xml
│   │   └── build.gradle
│   ├── build.gradle
│   └── settings.gradle
│
└── .github/workflows/  # CI/CD
    ├── rust-ci.yml
    └── android-ci.yml
```

## Building

### PC (Rust)

```bash
cd pc
cargo build --release
cargo run --release
```

**Note**: Currently uses stub audio module. To enable real audio:
1. Install dependencies: `sudo apt-get install pkg-config libasound2-dev`
2. Uncomment `cpal = "0.15"` in `pc/Cargo.toml`

### Android

Open the `android/` directory in Android Studio and build, or use Gradle:

```bash
cd android
./gradlew build
```

## Usage

1. Start the PC application - it will start the signaling server (ws://0.0.0.0:8080) and discovery service
2. Launch the Android app
3. The app will automatically discover the PC on the local network
4. Tap "Connect to PC" to establish WebRTC connection
5. Audio streaming begins!

## Technology Stack

- **PC**: Rust + WebRTC + cpal (audio) + tokio (async runtime)
- **Android**: Kotlin + Google WebRTC SDK
- **Protocol**: WebRTC (for audio) + WebSocket (for signaling) + UDP (for discovery)

## CI/CD

GitHub Actions workflows automatically build and test both PC and Android applications on every push to main branch.

## License

MIT
