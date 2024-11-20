use crate::protocol::ehall::EhallProtocolItem;
#[cfg(feature = "cxlib_protocol_integrated")]
use cxlib_protocol::ProtocolItemTrait;
use ureq::Agent;

pub fn use_app(agent: &Agent, app_id: &str) -> Result<ureq::Response, Box<ureq::Error>> {
    Ok(agent
        .get(&format!("{}?appId={app_id}", EhallProtocolItem::AppShow))
        .set(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8",
        )
        .call()?)
}

pub fn get_app_list(agent: &Agent, search_key: &str) -> Result<ureq::Response, Box<ureq::Error>> {
    Ok(agent
        .get(&format!(
            "{}?{}&{}&{}&{}&{}",
            EhallProtocolItem::ServiceSearchCustom,
            format_args!("searchKey={search_key}"),
            "pageNumber=1",
            "pageSize=150",
            "sortKey=recentUseCount",
            "orderKey=desc",
        ))
        .call()?)
}
