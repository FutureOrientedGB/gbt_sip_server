use rsip::{
    self,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipRequestHandler;

impl SipRequestHandler {
    pub fn via(ip: &String, port: u16, branch: &String) -> rsip::headers::Via {
        rsip::typed::Via {
            version: rsip::Version::V2,
            transport: rsip::Transport::Udp,
            uri: rsip::Uri {
                host_with_port: (ip.clone(), port).into(),
                ..Default::default()
            },
            params: vec![
                rsip::Param::Other(rsip::param::OtherParam::new("rport"), None),
                rsip::param::Branch::new(branch).into(),
            ],
        }
        .into()
    }

    pub fn branch_get(request: &rsip::Request) -> String {
        if let Ok(v) = request.via_header() {
            if let Ok(tv) = v.typed() {
                if let Some(branch) = tv.branch() {
                    return branch.to_string();
                }
            }
        }
        return String::new();
    }
}
