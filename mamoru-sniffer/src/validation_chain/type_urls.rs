/// Required to sign a transaction for each message type
use crate::validation_chain::proto::validation_chain::{
    MsgRegisterDaemon, MsgRegisterSniffer, MsgReportIncident, MsgSubscribeDaemons,
    MsgUnregisterSniffer,
};
use cosmrs::proto::traits::TypeUrl;

impl TypeUrl for MsgRegisterSniffer {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgRegisterSniffer";
}

impl TypeUrl for MsgUnregisterSniffer {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgUnregisterSniffer";
}

impl TypeUrl for MsgSubscribeDaemons {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgSubscribeDaemons";
}

impl TypeUrl for MsgReportIncident {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgReportIncident";
}

impl TypeUrl for MsgRegisterDaemon {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgRegisterDaemon";
}
