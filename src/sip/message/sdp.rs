use std::str::FromStr;

use sdp_rs;

use vec1::vec1;

pub enum SdpSessionType {
    Play,
    Playback,
}

impl ToString for SdpSessionType {
    fn to_string(&self) -> String {
        match &self {
            &Self::Play => String::from("Play"),
            &Self::Playback => String::from("Playback"),
        }
    }
}

pub fn generate_media_sdp(
    media_server_ip: &String,
    media_server_port: u16,
    device_gb_code: &String,
    session_type: SdpSessionType,
    start_ts: u64,
    stop_ts: u64,
) -> String {
    // media description
    let media_desc = sdp_rs::MediaDescription {
        media: sdp_rs::lines::Media {
            media: sdp_rs::lines::media::MediaType::Video,
            port: media_server_port,
            num_of_ports: None,
            proto: sdp_rs::lines::media::ProtoType::RtpAvp,
            fmt: String::from("96 97 98 99"),
        },
        attributes: vec![
            sdp_rs::lines::Attribute::Rtpmap(sdp_rs::lines::attribute::Rtpmap {
                payload_type: 96,
                encoding_name: String::from("PS"),
                clock_rate: 90000,
                encoding_params: None,
            }),
            sdp_rs::lines::Attribute::Rtpmap(sdp_rs::lines::attribute::Rtpmap {
                payload_type: 97,
                encoding_name: String::from("MPEG4"),
                clock_rate: 90000,
                encoding_params: None,
            }),
            sdp_rs::lines::Attribute::Rtpmap(sdp_rs::lines::attribute::Rtpmap {
                payload_type: 98,
                encoding_name: String::from("H264"),
                clock_rate: 90000,
                encoding_params: None,
            }),
            sdp_rs::lines::Attribute::Rtpmap(sdp_rs::lines::attribute::Rtpmap {
                payload_type: 99,
                encoding_name: String::from("HEVC"),
                clock_rate: 90000,
                encoding_params: None,
            }),
            sdp_rs::lines::Attribute::Recvonly {},
            sdp_rs::lines::Attribute::Other {
                0: String::from("streamMode"),
                1: Some(String::from("MAIN")),
            },
        ],
        info: None,
        connections: vec![],
        bandwidths: vec![],
        key: None,
    };

    let session_desc = sdp_rs::SessionDescription {
        version: sdp_rs::lines::Version::V0,
        origin: sdp_rs::lines::Origin {
            username: device_gb_code.clone(),
            sess_id: String::from("0"),
            sess_version: String::from("0"),
            nettype: sdp_rs::lines::common::Nettype::In,
            addrtype: sdp_rs::lines::common::Addrtype::Ip4,
            unicast_address: std::net::IpAddr::V4(
                std::net::Ipv4Addr::from_str(media_server_ip).unwrap(),
            ),
        },
        session_name: sdp_rs::lines::SessionName::new(session_type.to_string()),
        session_info: None,
        uri: None,
        times: vec1![
            sdp_rs::Time {
                active: sdp_rs::lines::Active{start: start_ts, stop: stop_ts},
                repeat: vec![],
                zone: None,
            }
        ],
        media_descriptions: vec![media_desc],
        attributes: vec![sdp_rs::lines::Attribute::Other {
            0: String::from("y"),
            1: Some(String::from("0100000001")),
        }],
        emails: vec![],
        phones: vec![],
        connection: None,
        bandwidths: vec![],
        key: None,
       
    };

    session_desc.to_string()
}
