/// Required to sign a transaction for each message type
use crate::validation_chain::proto::validation_chain::{
    MsgCreateDaemonMetadata, MsgCreateDaemonMetadataResponse, MsgMarkSnifferStatistic,
    MsgMarkSnifferStatisticResponse, MsgRegisterDaemon, MsgRegisterDaemonResponse,
    MsgRegisterSniffer, MsgRegisterSnifferResponse, MsgReportIncident, MsgReportIncidentResponse,
    MsgSubscribeDaemons, MsgSubscribeDaemonsResponse, MsgUnregisterSniffer,
    MsgUnregisterSnifferResponse,
};
use cosmrs::proto::traits::TypeUrl;

impl TypeUrl for MsgRegisterSniffer {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgRegisterSniffer";
}

impl TypeUrl for MsgRegisterSnifferResponse {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgRegisterSnifferResponse";
}

impl TypeUrl for MsgUnregisterSniffer {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgUnregisterSniffer";
}

impl TypeUrl for MsgUnregisterSnifferResponse {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgUnregisterSnifferResponse";
}

impl TypeUrl for MsgSubscribeDaemons {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgSubscribeDaemons";
}

impl TypeUrl for MsgSubscribeDaemonsResponse {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgSubscribeDaemonsResponse";
}

impl TypeUrl for MsgReportIncident {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgReportIncident";
}

impl TypeUrl for MsgReportIncidentResponse {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgReportIncidentResponse";
}

impl TypeUrl for MsgRegisterDaemon {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgRegisterDaemon";
}

impl TypeUrl for MsgRegisterDaemonResponse {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgRegisterDaemonResponse";
}

impl TypeUrl for MsgCreateDaemonMetadata {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgCreateDaemonMetadata";
}

impl TypeUrl for MsgCreateDaemonMetadataResponse {
    const TYPE_URL: &'static str =
        "/validationchain.validationchain.MsgCreateDaemonMetadataResponse";
}

impl TypeUrl for MsgMarkSnifferStatistic {
    const TYPE_URL: &'static str = "/validationchain.validationchain.MsgMarkSnifferStatistic";
}
impl TypeUrl for MsgMarkSnifferStatisticResponse {
    const TYPE_URL: &'static str =
        "/validationchain.validationchain.MsgMarkSnifferStatisticResponse";
}
