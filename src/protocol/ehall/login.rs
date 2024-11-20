use crate::protocol::ehall::EhallProtocolItem;
#[cfg(feature = "cxlib_protocol_integrated")]
use cxlib_protocol::ProtocolItemTrait;
use serde::Deserialize;
use ureq::Agent;

pub fn is_logged_in(agent: &Agent) -> bool {
    agent
        .get(EhallProtocolItem::UserFavoriteApps.get().as_str())
        .call()
        .is_ok_and(|r| {
            #[derive(Deserialize)]
            struct Tmp {
                #[serde(rename = "hasLogin")]
                has_login: bool,
            }
            let Tmp { has_login } = r.into_json().expect("failed to deserialize hasLogin");
            has_login
        })
}
