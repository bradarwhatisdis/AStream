use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::track::track_local::track_local_static_sample::TrackLocalStaticSample;
use webrtc::track::track_local::TrackLocal;
use webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability;
use tokio::sync::mpsc;

pub struct WebRTCManager {
    pub peer_connection: Option<RTCPeerConnection>,
    pub audio_track: Option<tokio::sync::mpsc::UnboundedSender<Vec<u8>>>,
}

impl WebRTCManager {
    pub fn new() -> Self {
        Self {
            peer_connection: None,
            audio_track: None,
        }
    }

    pub async fn create_offer(&mut self) -> Result<String, webrtc::error::Error> {
        let (peer_connection, audio_sender) = Self::create_peer_connection_with_audio().await?;
        
        let offer = peer_connection.create_offer(None).await?;
        let offer_json = serde_json::to_string(&offer).unwrap_or_default();
        
        peer_connection.set_local_description(offer).await?;
        
        self.peer_connection = Some(peer_connection);
        self.audio_track = Some(audio_sender);
        
        Ok(offer_json)
    }

    pub async fn handle_answer(&mut self, answer_json: &str) -> Result<(), webrtc::error::Error> {
        if let Some(pc) = &self.peer_connection {
            let answer: webrtc::peer_connection::sdp::session_description::RTCSessionDescription = 
                serde_json::from_str(answer_json).unwrap();
            pc.set_remote_description(answer).await?;
        }
        Ok(())
    }

    async fn create_peer_connection_with_audio() -> Result<(RTCPeerConnection, mpsc::UnboundedSender<Vec<u8>>), webrtc::error::Error> {
        let mut media_engine = MediaEngine::default();
        media_engine.register_default_codecs()?;

        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut media_engine)?;

        let api = APIBuilder::new()
            .with_media_engine(media_engine)
            .with_interceptor_registry(registry)
            .build();

        let config = RTCConfiguration::default();

        let peer_connection = api.new_peer_connection(config).await?;

        let (audio_tx, mut audio_rx) = mpsc::unbounded_channel::<Vec<u8>>();

        peer_connection.on_peer_connection_state_change(Box::new(|state: RTCPeerConnectionState| {
            println!("Peer Connection State: {}", state);
            Box::pin(async {})
        }));

        // Handle incoming audio track (from Android)
        peer_connection.on_track(Box::new(|track, _, _| {
            let codec = track.codec();
            println!("Received track: {:?}", codec);
            Box::pin(async {})
        }));

        Ok((peer_connection, audio_tx))
    }
}
