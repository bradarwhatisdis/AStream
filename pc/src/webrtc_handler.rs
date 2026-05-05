use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::RTCPeerConnection;

pub async fn create_peer_connection() -> Result<(RTCPeerConnection, webrtc::data_channel::RTCDataChannel), webrtc::error::Error> {
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

    peer_connection.on_peer_connection_state_change(Box::new(|state: RTCPeerConnectionState| {
        println!("Peer Connection State changed: {}", state);
        Box::pin(async {})
    }));

    Ok((peer_connection, webrtc::data_channel::RTCDataChannel::default()))
}
