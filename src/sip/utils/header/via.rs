use rsip::{
    self as sip_rs,
    prelude::{HeadersExt, ToTypedHeader},
};

use crate::sip::handler::base::SipHandler;

impl SipHandler {
    pub fn via(&self, branch: &String) -> sip_rs::headers::Via {
        sip_rs::typed::Via {
            version: sip_rs::Version::V2,
            transport: sip_rs::Transport::Udp,
            uri: sip_rs::Uri {
                host_with_port: (self.ip.clone(), self.port).into(),
                ..Default::default()
            },
            params: vec![
                sip_rs::Param::Other(sip_rs::param::OtherParam::new("rport"), None),
                sip_rs::param::Branch::new(branch).into(),
            ],
        }
        .into()
    }

    pub fn branch_get(via: &sip_rs::headers::Via) -> String {
        if let Ok(tv) = via.typed() {
            if let Some(branch) = tv.branch() {
                return branch.to_string();
            }
        }

        return String::new();
    }
}
