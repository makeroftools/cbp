use async_graphql::*;

#[derive(SimpleObject)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    pub protocol: Option<String>,
    pub auth: Option<ProxyAuth>,
}

#[derive(SimpleObject)]
pub struct ConfigurationRestApi {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub base_path: Option<String>,
    pub timeout: u64,
    pub keep_alive: bool,
    pub compression: bool,
    pub retries: u32,
    pub backoff: u64,
    pub proxy: Option<ProxyConfig>,
    pub custom_headers: Option<HashMap<String,
    pub agent: Option<HttpAgent>,
    pub private_key: Option<PrivateKey>,
    pub private_key_passphrase: Option<String>,
    pub time_unit: Option<TimeUnit>,
    pub client: Client,
    pub user_agent: String,
    pub signature_gen: SignatureGenerator,
}

#[derive(SimpleObject)]
pub struct ConfigurationWebsocketApi {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub ws_url: Option<String>,
    pub timeout: u64,
    pub reconnect_delay: u64,
    pub WebsocketMode: :Single")]
    pub mode: WebsocketMode,
    pub agent: Option<AgentConnector>,
    pub private_key: Option<PrivateKey>,
    pub private_key_passphrase: Option<String>,
    pub time_unit: Option<TimeUnit>,
    pub auto_session_relogon: bool,
    pub user_agent: String,
    pub signature_gen: SignatureGenerator,
}

#[derive(SimpleObject)]
pub struct ConfigurationWebsocketStreams {
    pub ws_url: Option<String>,
    pub reconnect_delay: u64,
    pub WebsocketMode: :Single")]
    pub mode: WebsocketMode,
    pub agent: Option<AgentConnector>,
    pub time_unit: Option<TimeUnit>,
    pub user_agent: String,
}

#[derive(SimpleObject)]
pub struct RestApiRateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: Interval,
    pub interval_num: u32,
    pub count: u32,
    pub retry_after: Option<u32>,
}

#[derive(SimpleObject)]
pub struct WebsocketApiRateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: Interval,
    pub interval_num: u32,
    pub limit: u32,
    pub count: u32,
}

#[derive(SimpleObject)]
pub struct WebsocketApiConnectConfig {
    pub mode: Option<WebsocketMode>,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsConnectConfig {
    pub streams: Vec<String>,
    pub mode: Option<WebsocketMode>,
}

#[derive(SimpleObject)]
pub struct Subscription {
    pub handle: JoinHandle<()>,
}

#[derive(SimpleObject)]
pub struct WebsocketEventEmitter {
    pub subscribers: Arc<std::sync::Mutex<Vec<UnboundedSender<WebsocketEvent>>>>,
}

#[derive(SimpleObject)]
pub struct PendingRequest {
    pub completion: oneshot::Sender<Result<Value,
}

#[derive(SimpleObject)]
pub struct WebsocketSessionLogonReq {
    pub method: String,
    pub payload: BTreeMap<String,
    pub options: WebsocketMessageSendOptions,
}

#[derive(SimpleObject)]
pub struct WebsocketConnectionState {
    pub reconnection_pending: bool,
    pub renewal_pending: bool,
    pub close_initiated: bool,
    pub pending_requests: HashMap<String,
    pub pending_subscriptions: VecDeque<String>,
    pub stream_callbacks: HashMap<String,
    pub is_session_logged_on: bool,
    pub session_logon_req: Option<WebsocketSessionLogonReq>,
    pub handler: Option<Arc<dyn WebsocketHandler>>,
    pub ws_write_tx: Option<UnboundedSender<Message>>,
}

#[derive(SimpleObject)]
pub struct WebsocketConnection {
    pub id: String,
    pub drain_notify: Notify,
    pub state: Mutex<WebsocketConnectionState>,
}

#[derive(SimpleObject)]
pub struct ReconnectEntry {
    pub connection_id: String,
    pub url: String,
    pub is_renewal: bool,
}

#[derive(SimpleObject)]
pub struct WebsocketCommon {
    pub events: WebsocketEventEmitter,
    pub mode: WebsocketMode,
    pub round_robin_index: AtomicUsize,
    pub connection_pool: Vec<Arc<WebsocketConnection>>,
    pub reconnect_tx: Sender<ReconnectEntry>,
    pub renewal_tx: Sender<(String,
    pub reconnect_delay: usize,
    pub agent: Option<AgentConnector>,
    pub user_agent: Option<String>,
}

#[derive(SimpleObject)]
pub struct WebsocketMessageSendOptions {
    pub with_api_key: bool,
    pub is_signed: bool,
    pub is_session_logon: Option<bool>,
    pub is_session_logout: Option<bool>,
}

#[derive(SimpleObject)]
pub struct WebsocketApi {
    pub common: Arc<WebsocketCommon>,
    pub configuration: ConfigurationWebsocketApi,
    pub is_connecting: Arc<Mutex<bool>>,
    pub stream_callbacks: Mutex<HashMap<String,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub common: Arc<WebsocketCommon>,
    pub is_connecting: Mutex<bool>,
    pub connection_streams: Mutex<HashMap<String,
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct DummyHandler {
    pub url: String,
}

#[derive(SimpleObject)]
pub struct DummyHandler {
    pub called: Arc<Mutex<bool>>,
    pub opened_url: Arc<Mutex<Option<String>>>,
}

#[derive(SimpleObject)]
pub struct DummyHandler {
    pub called_with: Arc<Mutex<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct SignatureGenerator {
    pub api_secret: Option<String>,
    pub private_key: Option<PrivateKey>,
    pub private_key_passphrase: Option<String>,
    pub raw_key_data: OnceCell<String>,
    pub key_object: OnceCell<PKey<openssl::pkey::Private>>,
    pub ed25519_signing_key: OnceCell<SigningKey>,
}

#[derive(SimpleObject)]
pub struct Dummy {
    pub foo: String,
}

#[derive(SimpleObject)]
pub struct TestResponse {
    pub message: String,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub pay_api_client: PayApiClient,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryResponseDataInnerReceiverInfo {
    pub Option: :is_none")]
    pub name: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub binance_id: Option<String>,
    pub Option: :is_none")]
    pub account_id: Option<String>,
    pub Option: :is_none")]
    pub country_code: Option<String>,
    pub Option: :is_none")]
    pub phone_number: Option<String>,
    pub Option: :is_none")]
    pub mobile_code: Option<String>,
    pub Option: :is_none")]
    pub extend: Option<Box<models::GetPayTradeHistoryResponseDataInnerReceiverInfoExtend>>,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::GetPayTradeHistoryResponseDataInner>>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryResponseDataInnerReceiverInfoExtend {
    pub Option: :is_none")]
    pub institution_name: Option<String>,
    pub Option: :is_none")]
    pub card_number: Option<String>,
    pub Option: :is_none")]
    pub digital_wallet_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryResponseDataInnerFundsDetailInnerWalletAssetCostInner {
    pub Option: :is_none")]
    pub param_1: Option<String>,
    pub Option: :is_none")]
    pub param_2: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryResponseDataInner {
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub transaction_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub currency: Option<String>,
    pub Option: :is_none")]
    pub wallet_type: Option<i64>,
    pub Option: :is_none")]
    pub wallet_types: Option<Vec<i64>>,
    pub Option: :is_none")]
    pub funds_detail: Option<Vec<models::GetPayTradeHistoryResponseDataInnerFundsDetailInner>>,
    pub Option: :is_none")]
    pub payer_info: Option<Box<models::GetPayTradeHistoryResponseDataInnerPayerInfo>>,
    pub Option: :is_none")]
    pub receiver_info: Option<Box<models::GetPayTradeHistoryResponseDataInnerReceiverInfo>>,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryResponseDataInnerFundsDetailInner {
    pub Option: :is_none")]
    pub currency: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub wallet_asset_cost: Option<
        Vec<models::GetPayTradeHistoryResponseDataInnerFundsDetailInnerWalletAssetCostInner>,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryResponseDataInnerPayerInfo {
    pub Option: :is_none")]
    pub name: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub binance_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct PayApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetPayTradeHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockPayApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub convert_api_client: ConvertApiClient,
    pub market_data_api_client: MarketDataApiClient,
    pub portfolio_margin_endpoints_api_client: PortfolioMarginEndpointsApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_data_streams_api_client: UserDataStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketApi {
    pub websocket_api_base: Arc<WebsocketApiBase>,
    pub account_api_client: AccountApiClient,
    pub market_data_api_client: MarketDataApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_data_streams_api_client: UserDataStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketApiHandle {
    pub configuration: ConfigurationWebsocketApi,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub websocket_streams_base: Arc<WebsocketStreamsBase>,
    pub websocket_market_streams_api_client: WebsocketMarketStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsHandle {
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct AccountUpdateA {
    pub Option: :is_none")]
    pub m: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<Vec<models::AccountUpdateABInner>>,
    pub Option: :is_none")]
    pub p_uppercase: Option<Vec<models::AccountUpdateAPInner>>,
}

#[derive(SimpleObject)]
pub struct AllMarketMiniTickersStreamResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountConfigUpdateAc {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub l: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ConditionalOrderTriggerReject {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub or: Option<Box<models::ConditionalOrderTriggerRejectOr>>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::KlineCandlestickStreamsResponseK>>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolBookTickerStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct MultiAssetsModeAssetIndexResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub g: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub g_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketLiquidationOrderStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<Box<models::AllMarketLiquidationOrderStreamsResponseO>>,
}

#[derive(SimpleObject)]
pub struct DiffBookDepthStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub pu: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub a: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub ct: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::ContinuousContractKlineCandlestickStreamsResponseK>>,
}

#[derive(SimpleObject)]
pub struct GridUpdate {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub gu: Option<Box<models::GridUpdateGu>>,
}

#[derive(SimpleObject)]
pub struct CompositeIndexSymbolInformationStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub c_uppercase: Option<String>,
    pub Option: :is_none")]
    pub c: Option<Vec<models::CompositeIndexSymbolInformationStreamsResponseCInner>>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateABInner {
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub wb: Option<String>,
    pub Option: :is_none")]
    pub cw: Option<String>,
    pub Option: :is_none")]
    pub bc: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeLite {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<i64>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickStreamsResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct ConditionalOrderTriggerRejectOr {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub r: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountConfigUpdateAi {
    pub Option: :is_none")]
    pub j: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ContractInfoStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub ct: Option<String>,
    pub Option: :is_none")]
    pub dt: Option<i64>,
    pub Option: :is_none")]
    pub ot: Option<i64>,
    pub Option: :is_none")]
    pub cs: Option<String>,
    pub Option: :is_none")]
    pub bks: Option<Vec<models::ContractInfoStreamResponseBksInner>>,
}

#[derive(SimpleObject)]
pub struct AccountConfigUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ac: Option<Box<models::AccountConfigUpdateAc>>,
    pub Option: :is_none")]
    pub ai: Option<Box<models::AccountConfigUpdateAi>>,
}

#[derive(SimpleObject)]
pub struct Listenkeyexpired {
    pub Option: :is_none")]
    pub e_uppercase: Option<String>,
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllBookTickersStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct CompositeIndexSymbolInformationStreamsResponseCInner {
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub w_uppercase: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
}

#[derive(SimpleObject)]
pub struct LiquidationOrderStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<Box<models::AllMarketLiquidationOrderStreamsResponseO>>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolTickerStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginCallPInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub pa: Option<String>,
    pub Option: :is_none")]
    pub mt: Option<String>,
    pub Option: :is_none")]
    pub iw: Option<String>,
    pub Option: :is_none")]
    pub mp: Option<String>,
    pub Option: :is_none")]
    pub up: Option<String>,
    pub Option: :is_none")]
    pub mm: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<Box<models::AccountUpdateA>>,
}

#[derive(SimpleObject)]
pub struct MarkPriceStreamForAllMarketResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AlgoUpdate {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<Box<models::AlgoUpdateO>>,
}

#[derive(SimpleObject)]
pub struct MarginCall {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub cw: Option<String>,
    pub Option: :is_none")]
    pub p: Option<Vec<models::MarginCallPInner>>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateAPInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub pa: Option<String>,
    pub Option: :is_none")]
    pub ep: Option<String>,
    pub Option: :is_none")]
    pub bep: Option<String>,
    pub Option: :is_none")]
    pub cr: Option<String>,
    pub Option: :is_none")]
    pub up: Option<String>,
    pub Option: :is_none")]
    pub mt: Option<String>,
    pub Option: :is_none")]
    pub iw: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketLiquidationOrderStreamsResponseO {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub ap: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StrategyUpdateSu {
    pub Option: :is_none")]
    pub si: Option<i64>,
    pub Option: :is_none")]
    pub st: Option<String>,
    pub Option: :is_none")]
    pub ss: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ut: Option<i64>,
    pub Option: :is_none")]
    pub c: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllMarketTickersStreamsResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AggregateTradeStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ContractInfoStreamResponseBksInner {
    pub Option: :is_none")]
    pub bs: Option<i64>,
    pub Option: :is_none")]
    pub bnf: Option<i64>,
    pub Option: :is_none")]
    pub bnc: Option<i64>,
    pub Option: :is_none")]
    pub mmr: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub cf: Option<i64>,
    pub Option: :is_none")]
    pub mi: Option<i64>,
    pub Option: :is_none")]
    pub ma: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AlgoUpdateO {
    pub Option: :is_none")]
    pub caid: Option<String>,
    pub Option: :is_none")]
    pub aid: Option<i64>,
    pub Option: :is_none")]
    pub at: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub ai: Option<String>,
    pub Option: :is_none")]
    pub ap: Option<String>,
    pub Option: :is_none")]
    pub aq: Option<String>,
    pub Option: :is_none")]
    pub act: Option<String>,
    pub Option: :is_none")]
    pub tp: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub wt: Option<String>,
    pub Option: :is_none")]
    pub pm: Option<String>,
    pub Option: :is_none")]
    pub cp: Option<bool>,
    pub Option: :is_none")]
    pub p_p: Option<bool>,
    pub Option: :is_none")]
    pub r_uppercase: Option<bool>,
    pub Option: :is_none")]
    pub tt: Option<i64>,
    pub Option: :is_none")]
    pub gtd: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub pu: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub a: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<Box<models::OrderTradeUpdateO>>,
}

#[derive(SimpleObject)]
pub struct GridUpdateGu {
    pub Option: :is_none")]
    pub si: Option<i64>,
    pub Option: :is_none")]
    pub st: Option<String>,
    pub Option: :is_none")]
    pub ss: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub up: Option<String>,
    pub Option: :is_none")]
    pub uq: Option<String>,
    pub Option: :is_none")]
    pub uf: Option<String>,
    pub Option: :is_none")]
    pub mp: Option<String>,
    pub Option: :is_none")]
    pub ut: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdateO {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub ap: Option<String>,
    pub Option: :is_none")]
    pub sp: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub r_uppercase: Option<bool>,
    pub Option: :is_none")]
    pub wt: Option<String>,
    pub Option: :is_none")]
    pub ot: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub cp: Option<bool>,
    pub Option: :is_none")]
    pub ap_uppercase: Option<String>,
    pub Option: :is_none")]
    pub cr: Option<String>,
    pub Option: :is_none")]
    pub p_p: Option<bool>,
    pub Option: :is_none")]
    pub si: Option<i64>,
    pub Option: :is_none")]
    pub ss: Option<i64>,
    pub Option: :is_none")]
    pub rp: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub pm: Option<String>,
    pub Option: :is_none")]
    pub gtd: Option<i64>,
    pub Option: :is_none")]
    pub er: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolMiniTickerStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
}

#[derive(SimpleObject)]
pub struct StrategyUpdate {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub su: Option<Box<models::StrategyUpdateSu>>,
}

#[derive(SimpleObject)]
pub struct MarkPriceStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WebsocketMarketStreamsApiClient {
    pub websocket_streams_base: Arc<WebsocketStreams>,
}

#[derive(SimpleObject)]
pub struct AggregateTradeStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllBookTickersStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketLiquidationOrderStreamsParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketMiniTickersStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketTickersStreamsParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct CompositeIndexSymbolInformationStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickStreamsParams {
    pub pair: String,
    pub contract_type: String,
    pub interval: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ContractInfoStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct DiffBookDepthStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolBookTickerStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolMiniTickerStreamParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolTickerStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsParams {
    pub symbol: String,
    pub interval: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct LiquidationOrderStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceStreamParams {
    pub symbol: String,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceStreamForAllMarketParams {
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct MultiAssetsModeAssetIndexParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthStreamsParams {
    pub symbol: String,
    pub levels: i64,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderBookResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerResponse1 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::SymbolPriceTickerResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::SymbolOrderBookTickerResponse1RateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct PositionInformationV2ResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub margin_asset: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub adl: Option<i64>,
    pub Option: :is_none")]
    pub bid_notional: Option<String>,
    pub Option: :is_none")]
    pub ask_notional: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::FuturesAccountBalanceV2ResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationV2ResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct CloseUserDataStreamResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<serde_json::Value>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::SymbolOrderBookTickerResponse1RateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2ResponseResultAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub margin_available: Option<bool>,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamResponseResult {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::NewOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::ModifyOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::KeepaliveUserDataStreamResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::SymbolOrderBookTickerResponse1RateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct NewAlgoOrderResponseResult {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub trigger_price: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_quantity: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponseResult {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerResponse2 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::SymbolPriceTickerResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::SymbolOrderBookTickerResponse1RateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct PositionInformationResponseResultInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
    pub Option: :is_none")]
    pub margin_type: Option<String>,
    pub Option: :is_none")]
    pub is_auto_add_margin: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub max_notional_value: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponseResult {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationV2Response {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::PositionInformationV2ResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationV2ResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2Response {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::AccountInformationV2ResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationV2ResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderResponseResult {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
}

#[derive(SimpleObject)]
pub struct OrderBookResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderBookResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::OrderBookResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2ResponseResultPositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::StartUserDataStreamResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::SymbolOrderBookTickerResponse1RateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct QueryOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::QueryOrderResponseResult>>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceV2Response {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::FuturesAccountBalanceV2ResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationV2ResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseResult {
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub multi_assets_margin: Option<bool>,
    pub Option: :is_none")]
    pub trade_group_id: Option<i64>,
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_maint_margin: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub total_cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::AccountInformationResponseResultAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::AccountInformationResponseResultPositionsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseResultPositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub isolated: Option<bool>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub max_notional: Option<String>,
    pub Option: :is_none")]
    pub bid_notional: Option<String>,
    pub Option: :is_none")]
    pub ask_notional: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelOrderResponseResult {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::AccountInformationResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationV2ResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::CancelAlgoOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CancelAlgoOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerResponse1 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::SymbolOrderBookTickerResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::SymbolOrderBookTickerResponse1RateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2ResponseResult {
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_maint_margin: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub total_cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::AccountInformationV2ResponseResultAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::AccountInformationV2ResponseResultPositionsInner>>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderResponseResult {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub trigger_price: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_quantity: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::PositionInformationResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationV2ResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceV2ResponseResultInner {
    pub Option: :is_none")]
    pub account_alias: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub balance: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub margin_available: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseResultAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub margin_available: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewAlgoOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::NewAlgoOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CancelAlgoOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerResponse1RateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderResponseResult {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerResponse1Result {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderBookResponseResult {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub asks: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerResponse1Result {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::CancelOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CancelAlgoOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::ModifyOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::ModifyOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerResponse2 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::SymbolOrderBookTickerResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::SymbolOrderBookTickerResponse1RateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2ResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct AccountInformationParams {
    pub id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2Params {
    pub id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceParams {
    pub id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceV2Params {
    pub id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderParams {
    pub id: Option<String>,
    pub algoid: Option<i64>,
    pub clientalgoid: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelOrderParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderParams {
    pub symbol: String,
    pub side: ModifyOrderSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub price_match: Option<ModifyOrderPriceMatchEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewAlgoOrderParams {
    pub algo_type: String,
    pub symbol: String,
    pub side: NewAlgoOrderSideEnum,
    pub type: String,
    pub id: Option<String>,
    pub position_side: Option<NewAlgoOrderPositionSideEnum>,
    pub time_in_force: Option<NewAlgoOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub trigger_price: Option<rust_decimal::Decimal>,
    pub by: "`MARK_PRICE`",
    pub working_type: Option<NewAlgoOrderWorkingTypeEnum>,
    pub price_match: Option<NewAlgoOrderPriceMatchEnum>,
    pub close_position: Option<String>,
    pub price_protect: Option<String>,
    pub reduce_only: Option<String>,
    pub activation_price: Option<rust_decimal::Decimal>,
    pub callback_rate: Option<rust_decimal::Decimal>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct NewOrderParams {
    pub symbol: String,
    pub side: NewOrderSideEnum,
    pub type: String,
    pub id: Option<String>,
    pub position_side: Option<NewOrderPositionSideEnum>,
    pub time_in_force: Option<NewOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct PositionInformationParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationV2Params {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct OrderBookParams {
    pub symbol: String,
    pub id: Option<String>,
    pub limits: [5,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamsApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct CloseUserDataStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetIncomeHistoryResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub income_type: Option<String>,
    pub Option: :is_none")]
    pub income: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub info: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub trade_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUserRateLimitResponseInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PlaceMultipleOrdersResponseInner {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct UsersForceOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV3ResponsePositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInnerAmendment {
    pub Option: :is_none")]
    pub price: Option<Box<models::GetOrderModifyHistoryResponseInnerAmendmentPrice>>,
    pub Option: :is_none")]
    pub orig_qty: Option<Box<models::GetOrderModifyHistoryResponseInnerAmendmentOrigQty>>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTransactionHistoryDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CompressedAggregateTradesListResponseInner {
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
}

#[derive(SimpleObject)]
pub struct CancelAllOpenOrdersResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInner {
    pub Option: :is_none")]
    pub amendment_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub amendment: Option<Box<models::GetOrderModifyHistoryResponseInnerAmendment>>,
}

#[derive(SimpleObject)]
pub struct QueryIndexPriceConstituentsResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub constituents: Option<Vec<models::QueryIndexPriceConstituentsResponseConstituentsInner>>,
}

#[derive(SimpleObject)]
pub struct ToggleBnbBurnOnFuturesTradeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetPositionMarginChangeHistoryResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub delta_type: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
}

#[derive(SimpleObject)]
pub struct TestOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionAdlQuantileEstimationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub adl_quantile: Option<Box<models::PositionAdlQuantileEstimationResponseInnerAdlQuantile>>,
}

#[derive(SimpleObject)]
pub struct NotionalAndLeverageBracketsResponse2 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub notional_coef: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub brackets: Option<Vec<models::NotionalAndLeverageBracketsResponse2BracketsInner>>,
}

#[derive(SimpleObject)]
pub struct AcceptTheOfferedQuoteResponse {
    pub Option: :is_none")]
    pub order_id: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2ResponsePositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub isolated: Option<bool>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub max_notional: Option<String>,
    pub Option: :is_none")]
    pub bid_notional: Option<String>,
    pub Option: :is_none")]
    pub ask_notional: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCurrentPositionModeResponse {
    pub Option: :is_none")]
    pub dual_side_position: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTradeHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseSymbolsInnerFiltersInner {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_price: Option<String>,
    pub Option: :is_none")]
    pub min_price: Option<String>,
    pub Option: :is_none")]
    pub tick_size: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub multiplier_decimal: Option<String>,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamResponse {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct ListAllConvertPairsResponseInner {
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub from_asset_min_amount: Option<String>,
    pub Option: :is_none")]
    pub from_asset_max_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset_min_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset_max_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct CompositeIndexSymbolInformationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub component: Option<String>,
    pub Option: :is_none")]
    pub base_asset_list:
        Option<Vec<models::CompositeIndexSymbolInformationResponseInnerBaseAssetListInner>>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountConfigurationResponse {
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub dual_side_position: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub multi_assets_margin: Option<bool>,
    pub Option: :is_none")]
    pub trade_group_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesOrderHistoryDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV3Response {
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_maint_margin: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub total_cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::AccountInformationV3ResponseAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::AccountInformationV3ResponsePositionsInner>>,
}

#[derive(SimpleObject)]
pub struct OldTradesLookupResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_rpi_trade: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AccountTradeListResponseInner {
    pub Option: :is_none")]
    pub buyer: Option<bool>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub maker: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub realized_pnl: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarkPriceResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub index_price: Option<String>,
    pub Option: :is_none"
    )]
    pub estimated_settle_price: Option<String>,
    pub Option: :is_none")]
    pub last_funding_rate: Option<String>,
    pub Option: :is_none")]
    pub interest_rate: Option<String>,
    pub Option: :is_none")]
    pub next_funding_time: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponseIndicators {
    pub Option: :is_none")]
    pub btcusdt: Option<
        Vec<models::FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsBtcusdtInner>,
    pub Option: :is_none")]
    pub ethusdt: Option<
        Vec<models::FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsBtcusdtInner>,
    pub Option: :is_none")]
    pub account: Option<
        Vec<models::FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsAccountInner>,
}

#[derive(SimpleObject)]
pub struct PositionInformationV3ResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub margin_asset: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub adl: Option<i64>,
    pub Option: :is_none")]
    pub bid_notional: Option<String>,
    pub Option: :is_none")]
    pub ask_notional: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationV2ResponseInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
    pub Option: :is_none")]
    pub margin_type: Option<String>,
    pub Option: :is_none")]
    pub is_auto_add_margin: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub max_notional_value: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2ResponseAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub margin_available: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentOpenOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseSymbolsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub contract_type: Option<String>,
    pub Option: :is_none")]
    pub delivery_date: Option<i64>,
    pub Option: :is_none")]
    pub onboard_date: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub maint_margin_percent: Option<String>,
    pub Option: :is_none"
    )]
    pub required_margin_percent: Option<String>,
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub margin_asset: Option<String>,
    pub Option: :is_none")]
    pub price_precision: Option<i64>,
    pub Option: :is_none")]
    pub quantity_precision: Option<i64>,
    pub Option: :is_none")]
    pub base_asset_precision: Option<i64>,
    pub Option: :is_none")]
    pub quote_precision: Option<i64>,
    pub Option: :is_none")]
    pub underlying_type: Option<String>,
    pub Option: :is_none")]
    pub underlying_sub_type: Option<Vec<String>>,
    pub Option: :is_none")]
    pub settle_plan: Option<i64>,
    pub Option: :is_none")]
    pub trigger_protect: Option<String>,
    pub Option: :is_none")]
    pub filters: Option<Vec<models::ExchangeInformationResponseSymbolsInnerFiltersInner>>,
    pub Option: :is_none")]
    pub order_type: Option<Vec<String>>,
    pub Option: :is_none")]
    pub time_in_force: Option<Vec<String>>,
    pub Option: :is_none")]
    pub liquidation_fee: Option<String>,
    pub Option: :is_none")]
    pub market_take_bound: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryInsuranceFundBalanceSnapshotResponse1AssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MultiAssetsModeAssetIndexResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub index: Option<String>,
    pub Option: :is_none")]
    pub bid_buffer: Option<String>,
    pub Option: :is_none")]
    pub ask_buffer: Option<String>,
    pub Option: :is_none")]
    pub bid_rate: Option<String>,
    pub Option: :is_none")]
    pub ask_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_bid_buffer: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_ask_buffer: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_bid_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_ask_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryInsuranceFundBalanceSnapshotResponse2Inner {
    pub Option: :is_none")]
    pub symbols: Option<Vec<String>>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::QueryInsuranceFundBalanceSnapshotResponse2InnerAssetsInner>>,
}

#[derive(SimpleObject)]
pub struct CurrentAllAlgoOpenOrdersResponseInner {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub actual_order_id: Option<String>,
    pub Option: :is_none")]
    pub actual_price: Option<String>,
    pub Option: :is_none")]
    pub trigger_price: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_quantity: Option<String>,
    pub Option: :is_none")]
    pub tp_trigger_price: Option<String>,
    pub Option: :is_none")]
    pub tp_price: Option<String>,
    pub Option: :is_none")]
    pub sl_trigger_price: Option<String>,
    pub Option: :is_none")]
    pub sl_price: Option<String>,
    pub Option: :is_none")]
    pub tp_order_type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarkPriceResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub index_price: Option<String>,
    pub Option: :is_none"
    )]
    pub estimated_settle_price: Option<String>,
    pub Option: :is_none")]
    pub last_funding_rate: Option<String>,
    pub Option: :is_none")]
    pub interest_rate: Option<String>,
    pub Option: :is_none")]
    pub next_funding_time: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OpenInterestStatisticsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub sum_open_interest: Option<String>,
    pub Option: :is_none"
    )]
    pub sum_open_interest_value: Option<String>,
    pub Option: :is_none"
    )]
    pub cmc_circulating_supply: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<String>,
}

#[derive(SimpleObject)]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsBtcusdtInner {
    pub Option: :is_none")]
    pub is_locked: Option<bool>,
    pub Option: :is_none")]
    pub planned_recover_time: Option<i64>,
    pub Option: :is_none")]
    pub indicator: Option<String>,
    pub Option: :is_none")]
    pub value: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub trigger_value: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct LongShortRatioResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub long_short_ratio: Option<String>,
    pub Option: :is_none")]
    pub long_account: Option<String>,
    pub Option: :is_none")]
    pub short_account: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2Response {
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub fee_burn: Option<bool>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub multi_assets_margin: Option<bool>,
    pub Option: :is_none")]
    pub trade_group_id: Option<i64>,
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_maint_margin: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub total_cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::AccountInformationV2ResponseAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::AccountInformationV2ResponsePositionsInner>>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
}

#[derive(SimpleObject)]
pub struct SendQuoteRequestResponse {
    pub Option: :is_none")]
    pub quote_id: Option<String>,
    pub Option: :is_none")]
    pub ratio: Option<String>,
    pub Option: :is_none")]
    pub inverse_ratio: Option<String>,
    pub Option: :is_none")]
    pub valid_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryIndexPriceConstituentsResponseConstituentsInner {
    pub Option: :is_none")]
    pub exchange: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub weight: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrPriceChangeStatisticsResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceV2ResponseInner {
    pub Option: :is_none")]
    pub account_alias: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub balance: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub margin_available: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyMultipleOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct AdlRiskResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub adl_risk: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFundingRateInfoResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub adjusted_funding_rate_cap: Option<String>,
    pub Option: :is_none"
    )]
    pub adjusted_funding_rate_floor: Option<String>,
    pub Option: :is_none"
    )]
    pub funding_interval_hours: Option<i64>,
    pub Option: :is_none")]
    pub disclaimer: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInnerAmendmentPrice {
    pub Option: :is_none")]
    pub before: Option<String>,
    pub Option: :is_none")]
    pub after: Option<String>,
}

#[derive(SimpleObject)]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponse {
    pub Option: :is_none")]
    pub indicators:
        Option<Box<models::FuturesTradingQuantitativeRulesIndicatorsResponseIndicators>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderBookResponse {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub asks: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct RecentTradesListResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_rpi_trade: Option<bool>,
}

#[derive(SimpleObject)]
pub struct PositionAdlQuantileEstimationResponseInnerAdlQuantile {
    pub Option: :is_none")]
    pub long: Option<i64>,
    pub Option: :is_none")]
    pub short: Option<i64>,
    pub Option: :is_none")]
    pub hedge: Option<i64>,
    pub Option: :is_none")]
    pub both: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTradeDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTransactionHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct NotionalAndLeverageBracketsResponse1Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub notional_coef: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub brackets: Option<Vec<models::NotionalAndLeverageBracketsResponse1InnerBracketsInner>>,
}

#[derive(SimpleObject)]
pub struct MultiAssetsModeAssetIndexResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub index: Option<String>,
    pub Option: :is_none")]
    pub bid_buffer: Option<String>,
    pub Option: :is_none")]
    pub ask_buffer: Option<String>,
    pub Option: :is_none")]
    pub bid_rate: Option<String>,
    pub Option: :is_none")]
    pub ask_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_bid_buffer: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_ask_buffer: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_bid_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub auto_exchange_ask_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrPriceChangeStatisticsResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioPositionsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub long_short_ratio: Option<String>,
    pub Option: :is_none")]
    pub long_account: Option<String>,
    pub Option: :is_none")]
    pub short_account: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetCurrentMultiAssetsModeResponse {
    pub Option: :is_none")]
    pub multi_assets_margin: Option<bool>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponse {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenInterestResponse {
    pub Option: :is_none")]
    pub open_interest: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesOrderHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolConfigurationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub margin_type: Option<String>,
    pub Option: :is_none")]
    pub is_auto_add_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<i64>,
    pub Option: :is_none")]
    pub max_notional_value: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangeInitialLeverageResponse {
    pub Option: :is_none")]
    pub leverage: Option<i64>,
    pub Option: :is_none")]
    pub max_notional_value: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AutoCancelAllOpenOrdersResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub countdown_time: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponse {
    pub Option: :is_none")]
    pub exchange_filters: Option<Vec<String>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::ExchangeInformationResponseRateLimitsInner>>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::ExchangeInformationResponseAssetsInner>>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<models::ExchangeInformationResponseSymbolsInner>>,
    pub Option: :is_none")]
    pub timezone: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInnerAmendmentOrigQty {
    pub Option: :is_none")]
    pub before: Option<String>,
    pub Option: :is_none")]
    pub after: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllOrdersResponseInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PlaceMultipleOrdersBatchOrdersParameterInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<SideEnum>,
    pub Option: :is_none")]
    pub position_side: Option<PositionSideEnum>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<TimeInForceEnum>,
    pub Option: :is_none")]
    pub quantity: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub reduce_only: Option<String>,
    pub Option: :is_none")]
    pub price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub new_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub activation_price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub callback_rate: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub working_type: Option<WorkingTypeEnum>,
    pub Option: :is_none")]
    pub price_protect: Option<String>,
    pub Option: :is_none")]
    pub new_order_resp_type: Option<NewOrderRespTypeEnum>,
    pub Option: :is_none")]
    pub price_match: Option<PriceMatchEnum>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<SelfTradePreventionModeEnum>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioAccountsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub long_short_ratio: Option<String>,
    pub Option: :is_none")]
    pub long_account: Option<String>,
    pub Option: :is_none")]
    pub short_account: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderStatusResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub ratio: Option<String>,
    pub Option: :is_none")]
    pub inverse_ratio: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerV2Response1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CheckServerTimeResponse {
    pub Option: :is_none")]
    pub server_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMultipleOrdersResponseInner {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct AdlRiskResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub adl_risk: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderResponse {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangeMultiAssetsModeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryAllAlgoOrdersResponseInner {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub actual_order_id: Option<String>,
    pub Option: :is_none")]
    pub actual_price: Option<String>,
    pub Option: :is_none")]
    pub trigger_price: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_quantity: Option<String>,
    pub Option: :is_none")]
    pub tp_trigger_price: Option<String>,
    pub Option: :is_none")]
    pub tp_price: Option<String>,
    pub Option: :is_none")]
    pub sl_trigger_price: Option<String>,
    pub Option: :is_none")]
    pub sl_price: Option<String>,
    pub Option: :is_none")]
    pub tp_order_type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryInsuranceFundBalanceSnapshotResponse2InnerAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TakerBuySellVolumeResponseInner {
    pub Option: :is_none")]
    pub buy_sell_ratio: Option<String>,
    pub Option: :is_none")]
    pub buy_vol: Option<String>,
    pub Option: :is_none")]
    pub sell_vol: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangeMarginTypeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct NotionalAndLeverageBracketsResponse2BracketsInner {
    pub Option: :is_none")]
    pub bracket: Option<i64>,
    pub Option: :is_none")]
    pub initial_leverage: Option<i64>,
    pub Option: :is_none")]
    pub notional_cap: Option<i64>,
    pub Option: :is_none")]
    pub notional_floor: Option<i64>,
    pub Option: :is_none")]
    pub maint_margin_ratio: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub cum: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_available: Option<bool>,
    pub Option: :is_none")]
    pub auto_asset_exchange: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV3ResponseAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangePositionModeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct UserCommissionRateResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub maker_commission_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub taker_commission_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyIsolatedPositionMarginResponse {
    pub Option: :is_none")]
    pub amount: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ClassicPortfolioMarginAccountInformationResponse {
    pub Option: :is_none"
    )]
    pub max_withdraw_amount_usd: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyMultipleOrdersBatchOrdersParameterInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<SideEnum>,
    pub Option: :is_none")]
    pub quantity: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub price_match: Option<PriceMatchEnum>,
    pub Option: :is_none")]
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAlgoOrderResponse {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub actual_order_id: Option<String>,
    pub Option: :is_none")]
    pub actual_price: Option<String>,
    pub Option: :is_none")]
    pub trigger_price: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_quantity: Option<String>,
    pub Option: :is_none")]
    pub tp_trigger_price: Option<String>,
    pub Option: :is_none")]
    pub tp_price: Option<String>,
    pub Option: :is_none")]
    pub sl_trigger_price: Option<String>,
    pub Option: :is_none")]
    pub sl_price: Option<String>,
    pub Option: :is_none")]
    pub tp_order_type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseRateLimitsInner {
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFundingRateHistoryResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub funding_rate: Option<String>,
    pub Option: :is_none")]
    pub funding_time: Option<i64>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerV2Response2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewAlgoOrderResponse {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub trigger_price: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_quantity: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub callback_rate: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuarterlyContractSettlementPriceResponseInner {
    pub Option: :is_none")]
    pub delivery_time: Option<i64>,
    pub Option: :is_none")]
    pub delivery_price: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct QueryInsuranceFundBalanceSnapshotResponse1 {
    pub Option: :is_none")]
    pub symbols: Option<Vec<String>>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::QueryInsuranceFundBalanceSnapshotResponse1AssetsInner>>,
}

#[derive(SimpleObject)]
pub struct BasisResponseInner {
    pub Option: :is_none")]
    pub index_price: Option<String>,
    pub Option: :is_none")]
    pub contract_type: Option<String>,
    pub Option: :is_none")]
    pub basis_rate: Option<String>,
    pub Option: :is_none")]
    pub futures_price: Option<String>,
    pub Option: :is_none"
    )]
    pub annualized_basis_rate: Option<String>,
    pub Option: :is_none")]
    pub basis: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBnbBurnStatusResponse {
    pub Option: :is_none")]
    pub fee_burn: Option<bool>,
}

#[derive(SimpleObject)]
pub struct NotionalAndLeverageBracketsResponse1InnerBracketsInner {
    pub Option: :is_none")]
    pub bracket: Option<i64>,
    pub Option: :is_none")]
    pub initial_leverage: Option<i64>,
    pub Option: :is_none")]
    pub notional_cap: Option<i64>,
    pub Option: :is_none")]
    pub notional_floor: Option<i64>,
    pub Option: :is_none")]
    pub maint_margin_ratio: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub cum: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct CancelOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CompositeIndexSymbolInformationResponseInnerBaseAssetListInner {
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub weight_in_quantity: Option<String>,
    pub Option: :is_none")]
    pub weight_in_percentage: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesTradingQuantitativeRulesIndicatorsResponseIndicatorsAccountInner {
    pub Option: :is_none")]
    pub indicator: Option<String>,
    pub Option: :is_none")]
    pub value: Option<i64>,
    pub Option: :is_none")]
    pub trigger_value: Option<i64>,
    pub Option: :is_none")]
    pub planned_recover_time: Option<i64>,
    pub Option: :is_none")]
    pub is_locked: Option<bool>,
}

#[derive(SimpleObject)]
pub struct CancelAllAlgoOpenOrdersResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountInformationV2Params {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationV3Params {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceV2Params {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceV3Params {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountConfigurationParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesTradingQuantitativeRulesIndicatorsParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBnbBurnStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCurrentMultiAssetsModeParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCurrentPositionModeParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesOrderHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTradeHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTransactionHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesOrderHistoryDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTradeDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTransactionHistoryDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetIncomeHistoryParams {
    pub symbol: Option<String>,
    pub income_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NotionalAndLeverageBracketsParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserRateLimitParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolConfigurationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ToggleBnbBurnOnFuturesTradeParams {
    pub fee_burn: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UserCommissionRateParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountTradeListParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllOrdersParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AutoCancelAllOpenOrdersParams {
    pub symbol: String,
    pub countdown_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderParams {
    pub algoid: Option<i64>,
    pub clientalgoid: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllAlgoOpenOrdersParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllOpenOrdersParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMultipleOrdersParams {
    pub symbol: String,
    pub order_id_list: Option<Vec<i64>>,
    pub orig_client_order_id_list: Option<Vec<String>>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeInitialLeverageParams {
    pub symbol: String,
    pub leverage: int from 1 to 125
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub leverage: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeMarginTypeParams {
    pub symbol: String,
    pub margin_type: ChangeMarginTypeMarginTypeEnum,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeMultiAssetsModeParams {
    pub multi_assets_margin: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangePositionModeParams {
    pub dual_side_position: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CurrentAllAlgoOpenOrdersParams {
    pub algo_type: Option<String>,
    pub symbol: Option<String>,
    pub algo_id: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CurrentAllOpenOrdersParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetPositionMarginChangeHistoryParams {
    pub symbol: String,
    pub 1: Add position margin，2: Reduce position margin
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub type: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyIsolatedPositionMarginParams {
    pub symbol: String,
    pub amount: rust_decimal::Decimal,
    pub type: String,
    pub position_side: Option<ModifyIsolatedPositionMarginPositionSideEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyMultipleOrdersParams {
    pub batch_orders: Vec<models::ModifyMultipleOrdersBatchOrdersParameterInner>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderParams {
    pub symbol: String,
    pub side: ModifyOrderSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub price_match: Option<ModifyOrderPriceMatchEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewAlgoOrderParams {
    pub algo_type: String,
    pub symbol: String,
    pub side: NewAlgoOrderSideEnum,
    pub type: String,
    pub position_side: Option<NewAlgoOrderPositionSideEnum>,
    pub time_in_force: Option<NewAlgoOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub trigger_price: Option<rust_decimal::Decimal>,
    pub by: "`MARK_PRICE`",
    pub working_type: Option<NewAlgoOrderWorkingTypeEnum>,
    pub price_match: Option<NewAlgoOrderPriceMatchEnum>,
    pub close_position: Option<String>,
    pub price_protect: Option<String>,
    pub reduce_only: Option<String>,
    pub activation_price: Option<rust_decimal::Decimal>,
    pub callback_rate: Option<rust_decimal::Decimal>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct NewOrderParams {
    pub symbol: String,
    pub side: NewOrderSideEnum,
    pub type: String,
    pub position_side: Option<NewOrderPositionSideEnum>,
    pub time_in_force: Option<NewOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct PlaceMultipleOrdersParams {
    pub batch_orders: Vec<models::PlaceMultipleOrdersBatchOrdersParameterInner>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionAdlQuantileEstimationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationV2Params {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationV3Params {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAlgoOrderParams {
    pub algo_id: Option<i64>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct QueryAllAlgoOrdersParams {
    pub symbol: String,
    pub algo_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentOpenOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TestOrderParams {
    pub symbol: String,
    pub side: TestOrderSideEnum,
    pub type: String,
    pub position_side: Option<TestOrderPositionSideEnum>,
    pub time_in_force: Option<TestOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct UsersForceOrdersParams {
    pub symbol: Option<String>,
    pub auto_close_type: Option<UsersForceOrdersAutoCloseTypeEnum>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AdlRiskParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct BasisParams {
    pub pair: String,
    pub contract_type: BasisContractTypeEnum,
    pub period: BasisPeriodEnum,
    pub limit: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CompositeIndexSymbolInformationParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct CompressedAggregateTradesListParams {
    pub symbol: String,
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickDataParams {
    pub pair: String,
    pub contract_type: ContinuousContractKlineCandlestickDataContractTypeEnum,
    pub interval: ContinuousContractKlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFundingRateHistoryParams {
    pub symbol: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct IndexPriceKlineCandlestickDataParams {
    pub pair: String,
    pub interval: IndexPriceKlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickDataParams {
    pub symbol: String,
    pub interval: KlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct LongShortRatioParams {
    pub symbol: String,
    pub period: LongShortRatioPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarkPriceParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceKlineCandlestickDataParams {
    pub symbol: String,
    pub interval: MarkPriceKlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MultiAssetsModeAssetIndexParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct OldTradesLookupParams {
    pub symbol: String,
    pub limit: Option<i64>,
    pub from_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OpenInterestParams {
    pub symbol: String,
}

#[derive(SimpleObject)]
pub struct OpenInterestStatisticsParams {
    pub symbol: String,
    pub period: OpenInterestStatisticsPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderBookParams {
    pub symbol: String,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PremiumIndexKlineDataParams {
    pub symbol: String,
    pub interval: PremiumIndexKlineDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuarterlyContractSettlementPriceParams {
    pub pair: String,
}

#[derive(SimpleObject)]
pub struct QueryIndexPriceConstituentsParams {
    pub symbol: String,
}

#[derive(SimpleObject)]
pub struct QueryInsuranceFundBalanceSnapshotParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct RecentTradesListParams {
    pub symbol: String,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerV2Params {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct TakerBuySellVolumeParams {
    pub symbol: String,
    pub period: TakerBuySellVolumePeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrPriceChangeStatisticsParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioAccountsParams {
    pub symbol: String,
    pub period: TopTraderLongShortRatioAccountsPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioPositionsParams {
    pub symbol: String,
    pub period: TopTraderLongShortRatioPositionsPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginEndpointsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct ClassicPortfolioMarginAccountInformationParams {
    pub asset: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockPortfolioMarginEndpointsApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct UserDataStreamsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct MockUserDataStreamsApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct ConvertApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AcceptTheOfferedQuoteParams {
    pub quote_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ListAllConvertPairsParams {
    pub from_asset: Option<String>,
    pub to_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderStatusParams {
    pub order_id: Option<String>,
    pub quote_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct SendQuoteRequestParams {
    pub from_asset: String,
    pub to_asset: String,
    pub from_amount: Option<rust_decimal::Decimal>,
    pub to_amount: Option<rust_decimal::Decimal>,
    pub valid_time: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockConvertApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub market_data_api_client: MarketDataApiClient,
    pub market_maker_block_trade_api_client: MarketMakerBlockTradeApiClient,
    pub market_maker_endpoints_api_client: MarketMakerEndpointsApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_data_streams_api_client: UserDataStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub websocket_streams_base: Arc<WebsocketStreamsBase>,
    pub websocket_market_streams_api_client: WebsocketMarketStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsHandle {
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::KlineCandlestickStreamsResponseK>>,
}

#[derive(SimpleObject)]
pub struct AccountUpdatePInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub mp: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24HourResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub f_uppercase: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub bo: Option<String>,
    pub Option: :is_none")]
    pub ao: Option<String>,
    pub Option: :is_none")]
    pub bq: Option<String>,
    pub Option: :is_none")]
    pub aq: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub t: Option<String>,
    pub Option: :is_none")]
    pub g: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub vo: Option<String>,
    pub Option: :is_none")]
    pub mp: Option<String>,
    pub Option: :is_none")]
    pub hl: Option<String>,
    pub Option: :is_none")]
    pub ll: Option<String>,
    pub Option: :is_none")]
    pub eep: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24HourByUnderlyingAssetAndExpirationDataResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub f_uppercase: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub bo: Option<String>,
    pub Option: :is_none")]
    pub ao: Option<String>,
    pub Option: :is_none")]
    pub bq: Option<String>,
    pub Option: :is_none")]
    pub aq: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub t: Option<String>,
    pub Option: :is_none")]
    pub g: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub vo: Option<String>,
    pub Option: :is_none")]
    pub mp: Option<String>,
    pub Option: :is_none")]
    pub hl: Option<String>,
    pub Option: :is_none")]
    pub ll: Option<String>,
    pub Option: :is_none")]
    pub eep: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub t: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub b: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateGInner {
    pub Option: :is_none")]
    pub ui: Option<String>,
    pub Option: :is_none")]
    pub d: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub t: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub g: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub v: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdateOInnerFiInner {
    pub Option: :is_none")]
    pub t: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenInterestResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateBInner {
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub m: Option<String>,
    pub Option: :is_none")]
    pub u: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m_uppercase: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndexPriceStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewSymbolInfoResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<String>,
    pub Option: :is_none")]
    pub qa: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub unit: Option<i64>,
    pub Option: :is_none")]
    pub mq: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub sp: Option<String>,
    pub Option: :is_none")]
    pub ed: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub b_uppercase: Option<Vec<models::AccountUpdateBInner>>,
    pub Option: :is_none")]
    pub g_uppercase: Option<Vec<models::AccountUpdateGInner>>,
    pub Option: :is_none")]
    pub p_uppercase: Option<Vec<models::AccountUpdatePInner>>,
    pub Option: :is_none")]
    pub uid: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdateOInner {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub oid: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub stp: Option<i64>,
    pub Option: :is_none")]
    pub r: Option<bool>,
    pub Option: :is_none")]
    pub po: Option<bool>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub ec: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub tif: Option<String>,
    pub Option: :is_none")]
    pub oty: Option<String>,
    pub Option: :is_none")]
    pub fi: Option<Vec<models::OrderTradeUpdateOInnerFiInner>>,
}

#[derive(SimpleObject)]
pub struct RiskLevelChange {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub mb: Option<String>,
    pub Option: :is_none")]
    pub mm: Option<String>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub pu: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub a: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<Vec<models::OrderTradeUpdateOInner>>,
}

#[derive(SimpleObject)]
pub struct WebsocketMarketStreamsApiClient {
    pub websocket_streams_base: Arc<WebsocketStreams>,
}

#[derive(SimpleObject)]
pub struct IndexPriceStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsParams {
    pub symbol: String,
    pub interval: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceParams {
    pub underlying_asset: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewSymbolInfoParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenInterestParams {
    pub underlying_asset: String,
    pub expiration_date: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthStreamsParams {
    pub symbol: String,
    pub levels: i64,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24HourParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24HourByUnderlyingAssetAndExpirationDataParams {
    pub underlying_asset: String,
    pub expiration_date: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ResetMarketMakerProtectionConfigResponse {
    pub Option: :is_none")]
    pub underlying_id: Option<i64>,
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none"
    )]
    pub window_time_in_milliseconds: Option<i64>,
    pub Option: :is_none"
    )]
    pub frozen_time_in_milliseconds: Option<i64>,
    pub Option: :is_none")]
    pub qty_limit: Option<String>,
    pub Option: :is_none")]
    pub delta_limit: Option<String>,
    pub Option: :is_none")]
    pub last_trigger_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetAutoCancelAllOpenOrdersResponse {
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none")]
    pub countdown_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PlaceMultipleOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub post_only: Option<bool>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub mmp: Option<bool>,
}

#[derive(SimpleObject)]
pub struct QueryOptionOrderHistoryResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub post_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub reason: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub source: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub mmp: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseOptionAssetsInner {
    pub Option: :is_none")]
    pub name: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountBlockTradeListResponseInnerLegsInner {
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<String>,
    pub Option: :is_none")]
    pub order_price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub order_quantity: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub executed_amount: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub fee: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub order_type: Option<String>,
    pub Option: :is_none")]
    pub order_side: Option<String>,
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
    pub Option: :is_none")]
    pub trade_price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub trade_qty: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub trade_time: Option<i64>,
    pub Option: :is_none")]
    pub liquidity: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct CancelAllOptionOrdersOnSpecificSymbolResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct HistoricalExerciseRecordsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub strike_price: Option<String>,
    pub Option: :is_none")]
    pub real_strike_price: Option<String>,
    pub Option: :is_none")]
    pub expiry_date: Option<i64>,
    pub Option: :is_none")]
    pub strike_result: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseOptionSymbolsInnerFiltersInner {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub min_price: Option<String>,
    pub Option: :is_none")]
    pub max_price: Option<String>,
    pub Option: :is_none")]
    pub tick_size: Option<String>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseOptionSymbolsInner {
    pub Option: :is_none")]
    pub expiry_date: Option<i64>,
    pub Option: :is_none")]
    pub filters: Option<Vec<models::ExchangeInformationResponseOptionSymbolsInnerFiltersInner>>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub strike_price: Option<String>,
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none")]
    pub unit: Option<i64>,
    pub Option: :is_none")]
    pub maker_fee_rate: Option<String>,
    pub Option: :is_none")]
    pub taker_fee_rate: Option<String>,
    pub Option: :is_none")]
    pub liquidation_fee_rate: Option<String>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub min_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub min_maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub create_date: Option<i64>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub post_only: Option<bool>,
    pub Option: :is_none")]
    pub mmp: Option<bool>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct SetAutoCancelAllOpenOrdersResponse {
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none")]
    pub countdown_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllOptionOrdersByUnderlyingResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForOptionTransactionHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OptionAccountInformationResponse {
    pub Option: :is_none")]
    pub asset: Option<Vec<models::OptionAccountInformationResponseAssetInner>>,
    pub Option: :is_none")]
    pub greek: Option<Vec<models::OptionAccountInformationResponseGreekInner>>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub risk_level: Option<String>,
}

#[derive(SimpleObject)]
pub struct OptionPositionInformationResponseInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub reducible_qty: Option<String>,
    pub Option: :is_none")]
    pub mark_value: Option<String>,
    pub Option: :is_none")]
    pub ror: Option<String>,
    pub Option: :is_none")]
    pub unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub strike_price: Option<String>,
    pub Option: :is_none")]
    pub position_cost: Option<String>,
    pub Option: :is_none")]
    pub expiry_date: Option<i64>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct OldTradesLookupResponseInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub side: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountTradeListResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub realized_profit: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub volatility: Option<String>,
    pub Option: :is_none")]
    pub liquidity: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelMultipleOptionOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<i64>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AcceptBlockTradeOrderResponse {
    pub Option: :is_none"
    )]
    pub block_trade_settlement_key: Option<String>,
    pub Option: :is_none")]
    pub expire_time: Option<i64>,
    pub Option: :is_none")]
    pub liquidity: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub legs: Option<Vec<models::AcceptBlockTradeOrderResponseLegsInner>>,
}

#[derive(SimpleObject)]
pub struct QueryBlockTradeOrderResponseInner {
    pub Option: :is_none"
    )]
    pub block_trade_settlement_key: Option<String>,
    pub Option: :is_none")]
    pub expire_time: Option<i64>,
    pub Option: :is_none")]
    pub liquidity: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub legs: Option<Vec<models::ExtendBlockTradeOrderResponseLegsInner>>,
}

#[derive(SimpleObject)]
pub struct OptionMarkPriceResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub bid_iv: Option<String>,
    pub Option: :is_none")]
    pub ask_iv: Option<String>,
    pub Option: :is_none")]
    pub mark_iv: Option<String>,
    pub Option: :is_none")]
    pub delta: Option<String>,
    pub Option: :is_none")]
    pub theta: Option<String>,
    pub Option: :is_none")]
    pub gamma: Option<String>,
    pub Option: :is_none")]
    pub vega: Option<String>,
    pub Option: :is_none")]
    pub high_price_limit: Option<String>,
    pub Option: :is_none")]
    pub low_price_limit: Option<String>,
    pub Option: :is_none")]
    pub risk_free_interest: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderBookResponse {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub asks: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct RecentTradesListResponseInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub side: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponse {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrPriceChangeStatisticsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub open: Option<String>,
    pub Option: :is_none")]
    pub high: Option<String>,
    pub Option: :is_none")]
    pub low: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_trade_id: Option<i64>,
    pub Option: :is_none")]
    pub trade_count: Option<i64>,
    pub Option: :is_none")]
    pub strike_price: Option<String>,
    pub Option: :is_none")]
    pub exercise_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseOptionContractsInner {
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none")]
    pub settle_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndexPriceTickerResponse {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub index_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct RecentBlockTradesListResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub side: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OpenInterestResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub sum_open_interest: Option<String>,
    pub Option: :is_none")]
    pub sum_open_interest_usd: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<String>,
}

#[derive(SimpleObject)]
pub struct AutoCancelAllOpenOrdersResponse {
    pub Option: :is_none")]
    pub underlyings: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct ExtendBlockTradeOrderResponseLegsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
}

#[derive(SimpleObject)]
pub struct SetMarketMakerProtectionConfigResponse {
    pub Option: :is_none")]
    pub underlying_id: Option<i64>,
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none"
    )]
    pub window_time_in_milliseconds: Option<i64>,
    pub Option: :is_none"
    )]
    pub frozen_time_in_milliseconds: Option<i64>,
    pub Option: :is_none")]
    pub qty_limit: Option<String>,
    pub Option: :is_none")]
    pub delta_limit: Option<String>,
    pub Option: :is_none")]
    pub last_trigger_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponse {
    pub Option: :is_none")]
    pub timezone: Option<String>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub option_contracts: Option<Vec<models::ExchangeInformationResponseOptionContractsInner>>,
    pub Option: :is_none")]
    pub option_assets: Option<Vec<models::ExchangeInformationResponseOptionAssetsInner>>,
    pub Option: :is_none")]
    pub option_symbols: Option<Vec<models::ExchangeInformationResponseOptionSymbolsInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::ExchangeInformationResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AcceptBlockTradeOrderResponseLegsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewBlockTradeOrderResponse {
    pub Option: :is_none"
    )]
    pub block_trade_settlement_key: Option<String>,
    pub Option: :is_none")]
    pub expire_time: Option<i64>,
    pub Option: :is_none")]
    pub liquidity: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub legs: Option<Vec<models::ExtendBlockTradeOrderResponseLegsInner>>,
}

#[derive(SimpleObject)]
pub struct CheckServerTimeResponse {
    pub Option: :is_none")]
    pub server_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySingleOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub post_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub source: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub mmp: Option<bool>,
}

#[derive(SimpleObject)]
pub struct OptionAccountInformationResponseGreekInner {
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none")]
    pub delta: Option<String>,
    pub Option: :is_none")]
    pub gamma: Option<String>,
    pub Option: :is_none")]
    pub theta: Option<String>,
    pub Option: :is_none")]
    pub vega: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryBlockTradeDetailsResponse {
    pub Option: :is_none"
    )]
    pub block_trade_settlement_key: Option<String>,
    pub Option: :is_none")]
    pub expire_time: Option<i64>,
    pub Option: :is_none")]
    pub liquidity: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub legs: Option<Vec<models::QueryBlockTradeDetailsResponseLegsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountBlockTradeListResponseInner {
    pub Option: :is_none")]
    pub parent_order_id: Option<String>,
    pub Option: :is_none")]
    pub cross_type: Option<String>,
    pub Option: :is_none")]
    pub legs: Option<Vec<models::AccountBlockTradeListResponseInnerLegsInner>>,
    pub Option: :is_none"
    )]
    pub block_trade_settlement_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct PlaceMultipleOrdersOrdersParameterInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<SideEnum>,
    pub Option: :is_none")]
    pub r#type: Option<TypeEnum>,
    pub Option: :is_none")]
    pub quantity: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub time_in_force: Option<TimeInForceEnum>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub post_only: Option<bool>,
    pub Option: :is_none")]
    pub new_order_resp_type: Option<NewOrderRespTypeEnum>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub is_mmp: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AccountFundingFlowResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub create_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryBlockTradeDetailsResponseLegsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
}

#[derive(SimpleObject)]
pub struct OptionAccountInformationResponseAssetInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub equity: Option<String>,
    pub Option: :is_none")]
    pub available: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub unrealized_pnl: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentOpenOptionOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub post_only: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub mmp: Option<bool>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickDataResponseInner {
    pub Option: :is_none")]
    pub open: Option<String>,
    pub Option: :is_none")]
    pub high: Option<String>,
    pub Option: :is_none")]
    pub low: Option<String>,
    pub Option: :is_none")]
    pub close: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub trade_count: Option<i64>,
    pub Option: :is_none")]
    pub taker_volume: Option<String>,
    pub Option: :is_none")]
    pub taker_amount: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExtendBlockTradeOrderResponse {
    pub Option: :is_none"
    )]
    pub block_trade_settlement_key: Option<String>,
    pub Option: :is_none")]
    pub expire_time: Option<i64>,
    pub Option: :is_none")]
    pub liquidity: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub legs: Option<Vec<models::ExtendBlockTradeOrderResponseLegsInner>>,
}

#[derive(SimpleObject)]
pub struct GetMarketMakerProtectionConfigResponse {
    pub Option: :is_none")]
    pub underlying_id: Option<i64>,
    pub Option: :is_none")]
    pub underlying: Option<String>,
    pub Option: :is_none"
    )]
    pub window_time_in_milliseconds: Option<i64>,
    pub Option: :is_none"
    )]
    pub frozen_time_in_milliseconds: Option<i64>,
    pub Option: :is_none")]
    pub qty_limit: Option<String>,
    pub Option: :is_none")]
    pub delta_limit: Option<String>,
    pub Option: :is_none")]
    pub last_trigger_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OptionMarginAccountInformationResponse {
    pub Option: :is_none")]
    pub asset: Option<Vec<models::OptionMarginAccountInformationResponseAssetInner>>,
    pub Option: :is_none")]
    pub greek: Option<Vec<models::OptionAccountInformationResponseGreekInner>>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelOptionOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub post_only: Option<bool>,
    pub Option: :is_none")]
    pub create_date: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub source: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub mmp: Option<bool>,
}

#[derive(SimpleObject)]
pub struct UserExerciseRecordResponseInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub currency: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub exercise_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub create_date: Option<i64>,
    pub Option: :is_none")]
    pub price_scale: Option<i64>,
    pub Option: :is_none")]
    pub quantity_scale: Option<i64>,
    pub Option: :is_none")]
    pub option_side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct OptionMarginAccountInformationResponseAssetInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub equity: Option<String>,
    pub Option: :is_none")]
    pub available: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub adjusted_equity: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOptionTransactionHistoryDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountFundingFlowParams {
    pub currency: String,
    pub record_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForOptionTransactionHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOptionTransactionHistoryDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OptionAccountInformationParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OptionMarginAccountInformationParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountTradeListParams {
    pub symbol: Option<String>,
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllOptionOrdersByUnderlyingParams {
    pub underlying: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllOptionOrdersOnSpecificSymbolParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMultipleOptionOrdersParams {
    pub symbol: String,
    pub order_ids: Option<Vec<i64>>,
    pub client_order_ids: Option<Vec<String>>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelOptionOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewOrderParams {
    pub symbol: String,
    pub direction: SELL,
    pub side: NewOrderSideEnum,
    pub Type: LIMIT(only support limit)
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub r#type: NewOrderTypeEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: Option<rust_decimal::Decimal>,
    pub time_in_force: Option<NewOrderTimeInForceEnum>,
    pub reduce_only: Option<bool>,
    pub post_only: Option<bool>,
    pub new_order_resp_type: Option<NewOrderNewOrderRespTypeEnum>,
    pub client_order_id: Option<String>,
    pub is_mmp: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OptionPositionInformationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PlaceMultipleOrdersParams {
    pub orders: Vec<models::PlaceMultipleOrdersOrdersParameterInner>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentOpenOptionOrdersParams {
    pub symbol: Option<String>,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOptionOrderHistoryParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySingleOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UserExerciseRecordParams {
    pub symbol: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct HistoricalExerciseRecordsParams {
    pub underlying: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct IndexPriceTickerParams {
    pub underlying: String,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickDataParams {
    pub symbol: String,
    pub interval: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OldTradesLookupParams {
    pub symbol: String,
    pub from_id: Option<i64>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OpenInterestParams {
    pub underlying_asset: String,
    pub expiration: String,
}

#[derive(SimpleObject)]
pub struct OptionMarkPriceParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderBookParams {
    pub symbol: String,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RecentBlockTradesListParams {
    pub symbol: Option<String>,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RecentTradesListParams {
    pub symbol: String,
    pub Default: 100 Max:1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrPriceChangeStatisticsParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketMakerEndpointsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AutoCancelAllOpenOrdersParams {
    pub underlyings: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetAutoCancelAllOpenOrdersParams {
    pub underlying: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetMarketMakerProtectionConfigParams {
    pub underlying: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ResetMarketMakerProtectionConfigParams {
    pub underlying: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetAutoCancelAllOpenOrdersParams {
    pub underlying: String,
    pub countdown_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetMarketMakerProtectionConfigParams {
    pub underlying: Option<String>,
    pub window_time_in_milliseconds: Option<i64>,
    pub frozen_time_in_milliseconds: Option<i64>,
    pub qty_limit: Option<rust_decimal::Decimal>,
    pub delta_limit: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketMakerEndpointsApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketMakerBlockTradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AcceptBlockTradeOrderParams {
    pub block_order_matching_key: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountBlockTradeListParams {
    pub end_time: Option<i64>,
    pub start_time: Option<i64>,
    pub underlying: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelBlockTradeOrderParams {
    pub block_order_matching_key: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExtendBlockTradeOrderParams {
    pub block_order_matching_key: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewBlockTradeOrderParams {
    pub liquidity: String,
    pub example: eapi/v1/block/order/create?orders=[{"symbol":"BTC-210115-35000-C",
}

#[derive(SimpleObject)]
pub struct QueryBlockTradeDetailsParams {
    pub block_order_matching_key: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryBlockTradeOrderParams {
    pub block_order_matching_key: Option<String>,
    pub end_time: Option<i64>,
    pub start_time: Option<i64>,
    pub underlying: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketMakerBlockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct UserDataStreamsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct MockUserDataStreamsApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub market_data_api_client: MarketDataApiClient,
    pub trade_api_client: TradeApiClient,
}

#[derive(SimpleObject)]
pub struct ChangeAutoCompoundStatusResponse {
    pub Option: :is_none")]
    pub position_id: Option<String>,
    pub Option: :is_none")]
    pub auto_compound_plan: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubscribeDualInvestmentProductsResponse {
    pub Option: :is_none")]
    pub position_id: Option<i64>,
    pub Option: :is_none")]
    pub invest_coin: Option<String>,
    pub Option: :is_none")]
    pub exercised_coin: Option<String>,
    pub Option: :is_none")]
    pub subscription_amount: Option<String>,
    pub Option: :is_none")]
    pub duration: Option<i64>,
    pub Option: :is_none")]
    pub auto_compound_plan: Option<String>,
    pub Option: :is_none")]
    pub strike_price: Option<String>,
    pub Option: :is_none")]
    pub settle_date: Option<i64>,
    pub Option: :is_none")]
    pub purchase_status: Option<String>,
    pub Option: :is_none")]
    pub apr: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub purchase_time: Option<i64>,
    pub Option: :is_none")]
    pub option_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetDualInvestmentPositionsResponseListInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub invest_coin: Option<String>,
    pub Option: :is_none")]
    pub exercised_coin: Option<String>,
    pub Option: :is_none")]
    pub subscription_amount: Option<String>,
    pub Option: :is_none")]
    pub strike_price: Option<String>,
    pub Option: :is_none")]
    pub duration: Option<i64>,
    pub Option: :is_none")]
    pub settle_date: Option<i64>,
    pub Option: :is_none")]
    pub purchase_status: Option<String>,
    pub Option: :is_none")]
    pub apr: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub purchase_end_time: Option<i64>,
    pub Option: :is_none")]
    pub option_type: Option<String>,
    pub Option: :is_none")]
    pub auto_compound_plan: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetDualInvestmentProductListResponseListInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub invest_coin: Option<String>,
    pub Option: :is_none")]
    pub exercised_coin: Option<String>,
    pub Option: :is_none")]
    pub strike_price: Option<String>,
    pub Option: :is_none")]
    pub duration: Option<i64>,
    pub Option: :is_none")]
    pub settle_date: Option<i64>,
    pub Option: :is_none")]
    pub purchase_decimal: Option<i64>,
    pub Option: :is_none")]
    pub purchase_end_time: Option<i64>,
    pub Option: :is_none")]
    pub can_purchase: Option<bool>,
    pub Option: :is_none")]
    pub apr: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub min_amount: Option<String>,
    pub Option: :is_none")]
    pub max_amount: Option<String>,
    pub Option: :is_none")]
    pub create_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub option_type: Option<String>,
    pub Option: :is_none"
    )]
    pub is_auto_compound_enable: Option<bool>,
    pub Option: :is_none"
    )]
    pub auto_compound_plan_list: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct GetDualInvestmentPositionsResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub list: Option<Vec<models::GetDualInvestmentPositionsResponseListInner>>,
}

#[derive(SimpleObject)]
pub struct GetDualInvestmentProductListResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub list: Option<Vec<models::GetDualInvestmentProductListResponseListInner>>,
}

#[derive(SimpleObject)]
pub struct CheckDualInvestmentAccountsResponse {
    pub Option: :is_none")]
    pub total_amount_in_btc: Option<String>,
    pub Option: :is_none")]
    pub total_amount_in_usdt: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct ChangeAutoCompoundStatusParams {
    pub position_id: String,
    pub auto_compound_plan: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CheckDualInvestmentAccountsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDualInvestmentPositionsParams {
    pub status: Option<String>,
    pub Default: 10,
    pub Maximum: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page_size: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page_index: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeDualInvestmentProductsParams {
    pub id: String,
    pub order_id: String,
    pub deposit_amount: rust_decimal::Decimal,
    pub auto_compound_plan: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetDualInvestmentProductListParams {
    pub option_type: String,
    pub input: `optionType`:CALL,
    pub input: `optionType`:PUT,
    pub exercised_coin: String,
    pub input: `optionType`:CALL,
    pub input: `optionType`:PUT,
    pub invest_coin: String,
    pub Default: 10,
    pub Maximum: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page_size: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page_index: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub nft_api_client: NftApiClient,
}

#[derive(SimpleObject)]
pub struct GetNftWithdrawHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub list: Option<Vec<models::GetNftWithdrawHistoryResponseListInner>>,
}

#[derive(SimpleObject)]
pub struct GetNftAssetResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub list: Option<Vec<models::GetNftAssetResponseListInner>>,
}

#[derive(SimpleObject)]
pub struct GetNftTransactionHistoryResponseListInnerTokensInner {
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub token_id: Option<String>,
    pub Option: :is_none")]
    pub contract_address: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetNftTransactionHistoryResponseListInner {
    pub Option: :is_none")]
    pub order_no: Option<String>,
    pub Option: :is_none")]
    pub tokens: Option<Vec<models::GetNftTransactionHistoryResponseListInnerTokensInner>>,
    pub Option: :is_none")]
    pub trade_time: Option<i64>,
    pub Option: :is_none")]
    pub trade_amount: Option<String>,
    pub Option: :is_none")]
    pub trade_currency: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetNftAssetResponseListInner {
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub contract_address: Option<String>,
    pub Option: :is_none")]
    pub token_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetNftTransactionHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub list: Option<Vec<models::GetNftTransactionHistoryResponseListInner>>,
}

#[derive(SimpleObject)]
pub struct GetNftDepositHistoryResponseListInner {
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub contract_adrress: Option<String>,
    pub Option: :is_none")]
    pub token_id: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetNftWithdrawHistoryResponseListInner {
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub contract_adrress: Option<String>,
    pub Option: :is_none")]
    pub token_id: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub fee: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub fee_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetNftDepositHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub list: Option<Vec<models::GetNftDepositHistoryResponseListInner>>,
}

#[derive(SimpleObject)]
pub struct NftApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetNftAssetParams {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetNftDepositHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetNftTransactionHistoryParams {
    pub 0: purchase order,
    pub 1: sell order,
    pub 2: royalty income,
    pub 3: primary market order,
    pub 4: mint fee
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub order_type: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetNftWithdrawHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockNftApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub market_data_api_client: MarketDataApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_data_streams_api_client: UserDataStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub websocket_streams_base: Arc<WebsocketStreamsBase>,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsHandle {
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct Openorderloss {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o_uppercase: Option<Vec<models::OpenorderlossOInner>>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateA {
    pub Option: :is_none")]
    pub m: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<Vec<models::AccountUpdateABInner>>,
    pub Option: :is_none")]
    pub p_uppercase: Option<Vec<models::AccountUpdateAPInner>>,
}

#[derive(SimpleObject)]
pub struct ConditionalOrderTradeUpdate {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub fs: Option<String>,
    pub Option: :is_none")]
    pub so: Option<Box<models::ConditionalOrderTradeUpdateSo>>,
}

#[derive(SimpleObject)]
pub struct AccountConfigUpdateAc {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub l: Option<i64>,
}

#[derive(SimpleObject)]
pub struct Balanceupdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ConditionalOrderTradeUpdateSo {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub si: Option<i64>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub st: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub sp: Option<String>,
    pub Option: :is_none")]
    pub os: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ut: Option<i64>,
    pub Option: :is_none")]
    pub r_uppercase: Option<bool>,
    pub Option: :is_none")]
    pub wt: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub cp: Option<bool>,
    pub Option: :is_none")]
    pub ap_uppercase: Option<String>,
    pub Option: :is_none")]
    pub cr: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub gtd: Option<i64>,
}

#[derive(SimpleObject)]
pub struct Executionreport {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub d: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<String>,
    pub Option: :is_none")]
    pub g: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<String>,
    pub Option: :is_none")]
    pub n_uppercase: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub v: Option<i64>,
    pub Option: :is_none")]
    pub i_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub w: Option<bool>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub z_uppercase: Option<String>,
    pub Option: :is_none")]
    pub y_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub d_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub j: Option<i64>,
    pub Option: :is_none")]
    pub j_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub w_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenorderlossOInner {
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateABInner {
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub wb: Option<String>,
    pub Option: :is_none")]
    pub cw: Option<String>,
    pub Option: :is_none")]
    pub bc: Option<String>,
}

#[derive(SimpleObject)]
pub struct OutboundaccountpositionBInner {
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountConfigUpdate {
    pub Option: :is_none")]
    pub fs: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ac: Option<Box<models::AccountConfigUpdateAc>>,
}

#[derive(SimpleObject)]
pub struct Listenkeyexpired {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountUpdate {
    pub Option: :is_none")]
    pub fs: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub a: Option<Box<models::AccountUpdateA>>,
}

#[derive(SimpleObject)]
pub struct Liabilitychange {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub t: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateAPInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub pa: Option<String>,
    pub Option: :is_none")]
    pub ep: Option<String>,
    pub Option: :is_none")]
    pub cr: Option<String>,
    pub Option: :is_none")]
    pub up: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub bep: Option<String>,
}

#[derive(SimpleObject)]
pub struct Outboundaccountposition {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub b_uppercase: Option<Vec<models::OutboundaccountpositionBInner>>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdate {
    pub Option: :is_none")]
    pub fs: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub o: Option<Box<models::OrderTradeUpdateO>>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdateO {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub ap: Option<String>,
    pub Option: :is_none")]
    pub sp: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub r_uppercase: Option<bool>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub rp: Option<String>,
    pub Option: :is_none")]
    pub st: Option<String>,
    pub Option: :is_none")]
    pub si: Option<i64>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub gtd: Option<i64>,
}

#[derive(SimpleObject)]
pub struct Risklevelchange {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub eq: Option<String>,
    pub Option: :is_none")]
    pub ae: Option<String>,
    pub Option: :is_none")]
    pub m: Option<String>,
}

#[derive(SimpleObject)]
pub struct CmPositionAdlQuantileEstimationResponseInnerAdlQuantile {
    pub Option: :is_none")]
    pub long: Option<i64>,
    pub Option: :is_none")]
    pub short: Option<i64>,
    pub Option: :is_none")]
    pub hedge: Option<i64>,
    pub Option: :is_none")]
    pub both: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UmPositionAdlQuantileEstimationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub adl_quantile: Option<Box<models::CmPositionAdlQuantileEstimationResponseInnerAdlQuantile>>,
}

#[derive(SimpleObject)]
pub struct ChangeAutoRepayFuturesStatusResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct UmNotionalAndLeverageBracketsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub notional_coef: Option<String>,
    pub Option: :is_none")]
    pub brackets: Option<Vec<models::UmNotionalAndLeverageBracketsResponseInnerBracketsInner>>,
}

#[derive(SimpleObject)]
pub struct QueryUmOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmAccountDetailResponse {
    pub Option: :is_none")]
    pub assets: Option<Vec<models::GetUmAccountDetailV2ResponseAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::GetUmAccountDetailResponsePositionsInner>>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsAccountInner {
    pub Option: :is_none")]
    pub indicator: Option<String>,
    pub Option: :is_none")]
    pub value: Option<i64>,
    pub Option: :is_none")]
    pub trigger_value: Option<i64>,
    pub Option: :is_none")]
    pub planned_recover_time: Option<i64>,
    pub Option: :is_none")]
    pub is_locked: Option<bool>,
}

#[derive(SimpleObject)]
pub struct QueryUserRateLimitResponseInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelUmConditionalOrderResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountTradeListResponseInner {
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub is_best_match: Option<bool>,
    pub Option: :is_none")]
    pub is_buyer: Option<bool>,
    pub Option: :is_none")]
    pub is_maker: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetMarginBorrowLoanInterestHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetMarginBorrowLoanInterestHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginMaxBorrowResponse {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub borrow_limit: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetCmAccountDetailResponse {
    pub Option: :is_none")]
    pub assets: Option<Vec<models::GetCmAccountDetailResponseAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::GetCmAccountDetailResponsePositionsInner>>,
}

#[derive(SimpleObject)]
pub struct CmNotionalAndLeverageBracketsResponseInnerBracketsInner {
    pub Option: :is_none")]
    pub bracket: Option<i64>,
    pub Option: :is_none")]
    pub initial_leverage: Option<i64>,
    pub Option: :is_none")]
    pub qty_cap: Option<i64>,
    pub Option: :is_none")]
    pub qty_floor: Option<i64>,
    pub Option: :is_none")]
    pub maint_margin_ratio: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub cum: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct QueryMarginLoanRecordResponseRowsInner {
    pub Option: :is_none")]
    pub tx_id: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryAllUmConditionalOrdersResponseInner {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCmOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountAllOpenOrdersOnASymbolResponseInnerOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryAllCmConditionalOrdersResponseInner {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUsersUmForceOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOcoResponseInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryMarginAccountsOpenOcoResponseInnerOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct QueryAllCurrentUmOpenConditionalOrdersResponseInner {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmIncomeHistoryResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub income_type: Option<String>,
    pub Option: :is_none")]
    pub income: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub info: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelUmOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsBtcusdtInner {
    pub Option: :is_none")]
    pub is_locked: Option<bool>,
    pub Option: :is_none")]
    pub planned_recover_time: Option<i64>,
    pub Option: :is_none")]
    pub indicator: Option<String>,
    pub Option: :is_none")]
    pub value: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub trigger_value: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct ModifyCmOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RepayFuturesNegativeBalanceResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOcoResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOcoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryMarginAccountsOcoResponseOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponse {
    pub Option: :is_none")]
    pub indicators:
        Option<Box<models::PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicators>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginMaxWithdrawResponse {
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentUmOpenConditionalOrderResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmAccountDetailResponsePositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub max_notional: Option<String>,
    pub Option: :is_none")]
    pub bid_notional: Option<String>,
    pub Option: :is_none")]
    pub ask_notional: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountOcoOrdersResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::CancelMarginAccountOcoOrdersResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::CancelMarginAccountOcoOrdersResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct CmPositionAdlQuantileEstimationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub adl_quantile: Option<Box<models::CmPositionAdlQuantileEstimationResponseInnerAdlQuantile>>,
}

#[derive(SimpleObject)]
pub struct CancelCmConditionalOrderResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
}

#[derive(SimpleObject)]
pub struct NewMarginOrderResponseFillsInner {
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginRepayRecordResponseRowsInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub tx_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCmModifyOrderHistoryResponseInnerAmendment {
    pub Option: :is_none")]
    pub price: Option<Box<models::QueryCmModifyOrderHistoryResponseInnerAmendmentPrice>>,
    pub Option: :is_none")]
    pub orig_qty: Option<Box<models::QueryCmModifyOrderHistoryResponseInnerAmendmentOrigQty>>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmAccountDetailV2Response {
    pub Option: :is_none")]
    pub assets: Option<Vec<models::GetUmAccountDetailV2ResponseAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::GetUmAccountDetailV2ResponsePositionsInner>>,
}

#[derive(SimpleObject)]
pub struct NewUmConditionalOrderResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelAllCmOpenConditionalOrdersResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct FundCollectionByAssetResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmAccountDetailV2ResponsePositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub notional: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetCmAccountDetailResponsePositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FundAutoCollectionResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewCmOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCmModifyOrderHistoryResponseInner {
    pub Option: :is_none")]
    pub amendment_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub amendment: Option<Box<models::QueryCmModifyOrderHistoryResponseInnerAmendment>>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentUmOpenOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUserCommissionRateForCmResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub maker_commission_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub taker_commission_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentMarginOpenOrderResponseInner {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub account_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub prevented_match_id: Option<String>,
    pub Option: :is_none")]
    pub prevented_quantity: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryAllCurrentUmOpenOrdersResponseInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponse {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountOcoOrdersResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangeUmInitialLeverageResponse {
    pub Option: :is_none")]
    pub leverage: Option<i64>,
    pub Option: :is_none")]
    pub max_notional_value: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountAllOpenOrdersOnASymbolResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountOcoOrdersResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForUmFuturesTransactionHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesBnbBurnStatusResponse {
    pub Option: :is_none")]
    pub fee_burn: Option<bool>,
}

#[derive(SimpleObject)]
pub struct NewCmConditionalOrderResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesTransactionDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub s3_link: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountBalanceResponse2 {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub cross_margin_borrowed: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_free: Option<String>,
    pub Option: :is_none"
    )]
    pub cross_margin_interest: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_locked: Option<String>,
    pub Option: :is_none")]
    pub um_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub um_unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub cm_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cm_unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub negative_balance: Option<String>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicators {
    pub Option: :is_none")]
    pub btcusdt: Option<Vec<models::PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsBtcusdtInner>>,
    pub Option: :is_none")]
    pub account: Option<Vec<models::PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponseIndicatorsAccountInner>>,
}

#[derive(SimpleObject)]
pub struct CancelAllUmOpenConditionalOrdersResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUmConditionalOrderHistoryResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginRepayRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryMarginRepayRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUsersCmForceOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeCmPositionModeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct UmAccountTradeListResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub realized_pnl: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub buyer: Option<bool>,
    pub Option: :is_none")]
    pub maker: Option<bool>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_asset: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::MarginAccountNewOcoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::MarginAccountNewOcoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponse {
    pub Option: :is_none")]
    pub uni_mmr: Option<String>,
    pub Option: :is_none")]
    pub account_equity: Option<String>,
    pub Option: :is_none")]
    pub actual_equity: Option<String>,
    pub Option: :is_none"
    )]
    pub account_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub account_maint_margin: Option<String>,
    pub Option: :is_none")]
    pub account_status: Option<String>,
    pub Option: :is_none"
    )]
    pub virtual_max_withdraw_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub total_available_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_margin_open_loss: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeUmPositionModeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUmPositionInformationResponseInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub max_notional_value: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub notional: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewUmOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangeCmInitialLeverageResponse {
    pub Option: :is_none")]
    pub leverage: Option<i64>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUserNegativeBalanceAutoExchangeRecordResponseRowsInnerDetailsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub negative_balance: Option<rust_decimal::Decimal>,
    pub Option: :is_none"
    )]
    pub negative_max_threshold: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CmNotionalAndLeverageBracketsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub brackets: Option<Vec<models::CmNotionalAndLeverageBracketsResponseInnerBracketsInner>>,
}

#[derive(SimpleObject)]
pub struct CancelAllUmOpenOrdersResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelAllCmOpenOrdersResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct UmNotionalAndLeverageBracketsResponseInnerBracketsInner {
    pub Option: :is_none")]
    pub bracket: Option<i64>,
    pub Option: :is_none")]
    pub initial_leverage: Option<i64>,
    pub Option: :is_none")]
    pub notional_cap: Option<i64>,
    pub Option: :is_none")]
    pub notional_floor: Option<i64>,
    pub Option: :is_none")]
    pub maint_margin_ratio: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub cum: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCmAccountDetailResponseAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOcoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCmPositionInformationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub notional_value: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUserNegativeBalanceAutoExchangeRecordResponseRowsInner {
    pub Option: :is_none")]
    pub start_time: Option<i64>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
    pub Option: :is_none")]
    pub details: Option<
        Vec<models::QueryUserNegativeBalanceAutoExchangeRecordResponseRowsInnerDetailsInner>,
}

#[derive(SimpleObject)]
pub struct GetCmCurrentPositionModeResponse {
    pub Option: :is_none")]
    pub dual_side_position: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ToggleBnbBurnOnUmFuturesTradeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUmModifyOrderHistoryResponseInner {
    pub Option: :is_none")]
    pub amendment_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub amendment: Option<Box<models::QueryCmModifyOrderHistoryResponseInnerAmendment>>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct UmFuturesAccountConfigurationResponse {
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub dual_side_position: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub multi_assets_margin: Option<bool>,
    pub Option: :is_none")]
    pub trade_group_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesOrderDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub s3_link: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForUmFuturesOrderHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCmConditionalOrderHistoryResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesTradeDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub s3_link: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmAccountDetailV2ResponseAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub account_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub prevented_match_id: Option<String>,
    pub Option: :is_none")]
    pub prevented_quantity: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUsersMarginForceOrdersResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryUsersMarginForceOrdersResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentCmOpenOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetMarginBorrowLoanInterestHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub tx_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub interest_accured_time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub raw_asset: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub interest_rate: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOcoResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentCmOpenConditionalOrderResponse {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewMarginOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_asset: Option<String>,
    pub Option: :is_none")]
    pub fills: Option<Vec<models::NewMarginOrderResponseFillsInner>>,
}

#[derive(SimpleObject)]
pub struct GetCmIncomeHistoryResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub income_type: Option<String>,
    pub Option: :is_none")]
    pub income: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub info: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCmModifyOrderHistoryResponseInnerAmendmentPrice {
    pub Option: :is_none")]
    pub before: Option<String>,
    pub Option: :is_none")]
    pub after: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountBorrowResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForUmFuturesTradeHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCmModifyOrderHistoryResponseInnerAmendmentOrigQty {
    pub Option: :is_none")]
    pub before: Option<String>,
    pub Option: :is_none")]
    pub after: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOcoResponseInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryMarginAccountsAllOcoResponseInnerOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct CancelCmOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginLoanRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryMarginLoanRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllCurrentCmOpenConditionalOrdersResponseInner {
    pub Option: :is_none"
    )]
    pub new_client_strategy_id: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_status: Option<String>,
    pub Option: :is_none")]
    pub strategy_type: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct CmAccountTradeListResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub realized_pnl: Option<String>,
    pub Option: :is_none")]
    pub margin_asset: Option<String>,
    pub Option: :is_none")]
    pub base_qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<bool>,
    pub Option: :is_none")]
    pub maker: Option<bool>,
}

#[derive(SimpleObject)]
pub struct QueryAllCmOrdersResponseInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserNegativeBalanceAutoExchangeRecordResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryUserNegativeBalanceAutoExchangeRecordResponseRowsInner>>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountAllOpenOrdersOnASymbolResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub orders:
        Option<Vec<models::CancelMarginAccountAllOpenOrdersOnASymbolResponseInnerOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<
        Vec<models::CancelMarginAccountAllOpenOrdersOnASymbolResponseInnerOrderReportsInner>,
}

#[derive(SimpleObject)]
pub struct QueryAllMarginAccountOrdersResponseInner {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub account_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub prevented_match_id: Option<String>,
    pub Option: :is_none")]
    pub prevented_quantity: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountBalanceResponse1Inner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_asset: Option<String>,
    pub Option: :is_none"
    )]
    pub cross_margin_borrowed: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_free: Option<String>,
    pub Option: :is_none"
    )]
    pub cross_margin_interest: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_locked: Option<String>,
    pub Option: :is_none")]
    pub um_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub um_unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub cm_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cm_unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub negative_balance: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUmCurrentPositionModeResponse {
    pub Option: :is_none")]
    pub dual_side_position: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetAutoRepayFuturesStatusResponse {
    pub Option: :is_none")]
    pub auto_repay: Option<bool>,
}

#[derive(SimpleObject)]
pub struct MarginAccountRepayResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUsersMarginForceOrdersResponseRowsInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub updated_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginNegativeBalanceInterestHistoryResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none"
    )]
    pub interest_accured_time: Option<i64>,
    pub Option: :is_none")]
    pub interest_rate: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountRepayDebtResponse {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub specify_repay_assets: Option<Vec<String>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetUserCommissionRateForUmResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub maker_commission_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub taker_commission_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyUmOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_quote: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub good_till_date: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct UmFuturesSymbolConfigurationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub margin_type: Option<String>,
    pub Option: :is_none")]
    pub is_auto_add_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<i64>,
    pub Option: :is_none")]
    pub max_notional_value: Option<String>,
}

#[derive(SimpleObject)]
pub struct BnbTransferResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountBalanceParams {
    pub asset: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct BnbTransferParams {
    pub amount: rust_decimal::Decimal,
    pub transfer_side: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeAutoRepayFuturesStatusParams {
    pub Default: `true`; `false` for turn off the auto-repay futures negative balance function
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub auto_repay: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeCmInitialLeverageParams {
    pub symbol: String,
    pub leverage: int from 1 to 125
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub leverage: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeCmPositionModeParams {
    pub dual_side_position: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeUmInitialLeverageParams {
    pub symbol: String,
    pub leverage: int from 1 to 125
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub leverage: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeUmPositionModeParams {
    pub dual_side_position: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CmNotionalAndLeverageBracketsParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FundAutoCollectionParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FundCollectionByAssetParams {
    pub asset: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetAutoRepayFuturesStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCmAccountDetailParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCmCurrentPositionModeParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCmIncomeHistoryParams {
    pub symbol: Option<String>,
    pub income_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForUmFuturesOrderHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForUmFuturesTradeHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForUmFuturesTransactionHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetMarginBorrowLoanInterestHistoryParams {
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub Default: `false`. Set to `true` for archived data from 6 months ago
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub archived: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmAccountDetailParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmAccountDetailV2Params {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmCurrentPositionModeParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesOrderDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesTradeDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesTransactionDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmIncomeHistoryParams {
    pub symbol: Option<String>,
    pub income_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUserCommissionRateForCmParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUserCommissionRateForUmParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginMaxBorrowParams {
    pub asset: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginUmTradingQuantitativeRulesIndicatorsParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCmPositionInformationParams {
    pub margin_asset: Option<String>,
    pub pair: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginLoanRecordParams {
    pub asset: String,
    pub tx_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub Default: `false`. Set to `true` for archived data from 6 months ago
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub archived: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginMaxWithdrawParams {
    pub asset: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginRepayRecordParams {
    pub asset: String,
    pub tx_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub Default: `false`. Set to `true` for archived data from 6 months ago
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub archived: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginNegativeBalanceInterestHistoryParams {
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUmPositionInformationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserNegativeBalanceAutoExchangeRecordParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserRateLimitParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RepayFuturesNegativeBalanceParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UmFuturesAccountConfigurationParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UmFuturesSymbolConfigurationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UmNotionalAndLeverageBracketsParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CancelAllCmOpenConditionalOrdersParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllCmOpenOrdersParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllUmOpenConditionalOrdersParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllUmOpenOrdersParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelCmConditionalOrderParams {
    pub symbol: String,
    pub strategy_id: Option<i64>,
    pub new_client_strategy_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelCmOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountAllOpenOrdersOnASymbolParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountOcoOrdersParams {
    pub symbol: String,
    pub order_list_id: Option<i64>,
    pub list_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMarginAccountOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelUmConditionalOrderParams {
    pub symbol: String,
    pub strategy_id: Option<i64>,
    pub new_client_strategy_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelUmOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CmAccountTradeListParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CmPositionAdlQuantileEstimationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUmFuturesBnbBurnStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountBorrowParams {
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoParams {
    pub symbol: String,
    pub side: MarginAccountNewOcoSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub stop_price: rust_decimal::Decimal,
    pub list_client_order_id: Option<String>,
    pub limit_client_order_id: Option<String>,
    pub limit_iceberg_qty: Option<rust_decimal::Decimal>,
    pub stop_client_order_id: Option<String>,
    pub stop_limit_price: Option<rust_decimal::Decimal>,
    pub stop_iceberg_qty: Option<rust_decimal::Decimal>,
    pub stop_limit_time_in_force: Option<MarginAccountNewOcoStopLimitTimeInForceEnum>,
    pub new_order_resp_type: Option<MarginAccountNewOcoNewOrderRespTypeEnum>,
    pub side_effect_type: Option<MarginAccountNewOcoSideEffectTypeEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountRepayParams {
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountRepayDebtParams {
    pub asset: String,
    pub amount: Option<String>,
    pub specify_repay_assets: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountTradeListParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyCmOrderParams {
    pub symbol: String,
    pub side: ModifyCmOrderSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub price_match: Option<ModifyCmOrderPriceMatchEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyUmOrderParams {
    pub symbol: String,
    pub side: ModifyUmOrderSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub price_match: Option<ModifyUmOrderPriceMatchEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewCmConditionalOrderParams {
    pub symbol: String,
    pub side: NewCmConditionalOrderSideEnum,
    pub strategy_type: NewCmConditionalOrderStrategyTypeEnum,
    pub position_side: Option<NewCmConditionalOrderPositionSideEnum>,
    pub time_in_force: Option<NewCmConditionalOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub by: "`MARK_PRICE`",
    pub working_type: Option<NewCmConditionalOrderWorkingTypeEnum>,
    pub price_protect: Option<String>,
    pub new_client_strategy_id: Option<String>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub activation_price: Option<rust_decimal::Decimal>,
    pub callback_rate: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewCmOrderParams {
    pub symbol: String,
    pub side: NewCmOrderSideEnum,
    pub type: NewCmOrderTypeEnum,
    pub position_side: Option<NewCmOrderPositionSideEnum>,
    pub time_in_force: Option<NewCmOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub price_match: Option<NewCmOrderPriceMatchEnum>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<NewCmOrderNewOrderRespTypeEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewMarginOrderParams {
    pub symbol: String,
    pub side: NewMarginOrderSideEnum,
    pub type: NewMarginOrderTypeEnum,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<NewMarginOrderNewOrderRespTypeEnum>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub side_effect_type: Option<NewMarginOrderSideEffectTypeEnum>,
    pub time_in_force: Option<NewMarginOrderTimeInForceEnum>,
    pub self_trade_prevention_mode: Option<NewMarginOrderSelfTradePreventionModeEnum>,
    pub auto_repay_at_cancel: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewUmConditionalOrderParams {
    pub symbol: String,
    pub side: NewUmConditionalOrderSideEnum,
    pub strategy_type: NewUmConditionalOrderStrategyTypeEnum,
    pub position_side: Option<NewUmConditionalOrderPositionSideEnum>,
    pub time_in_force: Option<NewUmConditionalOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub by: "`MARK_PRICE`",
    pub working_type: Option<NewUmConditionalOrderWorkingTypeEnum>,
    pub price_protect: Option<String>,
    pub new_client_strategy_id: Option<String>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub activation_price: Option<rust_decimal::Decimal>,
    pub callback_rate: Option<rust_decimal::Decimal>,
    pub price_match: Option<NewUmConditionalOrderPriceMatchEnum>,
    pub self_trade_prevention_mode: Option<NewUmConditionalOrderSelfTradePreventionModeEnum>,
    pub good_till_date: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewUmOrderParams {
    pub symbol: String,
    pub side: NewUmOrderSideEnum,
    pub type: NewUmOrderTypeEnum,
    pub position_side: Option<NewUmOrderPositionSideEnum>,
    pub time_in_force: Option<NewUmOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<NewUmOrderNewOrderRespTypeEnum>,
    pub price_match: Option<NewUmOrderPriceMatchEnum>,
    pub self_trade_prevention_mode: Option<NewUmOrderSelfTradePreventionModeEnum>,
    pub good_till_date: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllCmConditionalOrdersParams {
    pub symbol: Option<String>,
    pub strategy_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllCmOrdersParams {
    pub symbol: String,
    pub pair: Option<String>,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllCurrentCmOpenConditionalOrdersParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllCurrentCmOpenOrdersParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllCurrentUmOpenConditionalOrdersParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllCurrentUmOpenOrdersParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllMarginAccountOrdersParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllUmConditionalOrdersParams {
    pub symbol: Option<String>,
    pub strategy_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryAllUmOrdersParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCmConditionalOrderHistoryParams {
    pub symbol: String,
    pub strategy_id: Option<i64>,
    pub new_client_strategy_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCmModifyOrderHistoryParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCmOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentCmOpenConditionalOrderParams {
    pub symbol: String,
    pub strategy_id: Option<i64>,
    pub new_client_strategy_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentCmOpenOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentMarginOpenOrderParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentUmOpenConditionalOrderParams {
    pub symbol: String,
    pub strategy_id: Option<i64>,
    pub new_client_strategy_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentUmOpenOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOcoParams {
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOcoParams {
    pub order_list_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOcoParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUmConditionalOrderHistoryParams {
    pub symbol: String,
    pub strategy_id: Option<i64>,
    pub new_client_strategy_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUmModifyOrderHistoryParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUmOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUsersCmForceOrdersParams {
    pub symbol: Option<String>,
    pub auto_close_type: Option<QueryUsersCmForceOrdersAutoCloseTypeEnum>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUsersMarginForceOrdersParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUsersUmForceOrdersParams {
    pub symbol: Option<String>,
    pub auto_close_type: Option<QueryUsersUmForceOrdersAutoCloseTypeEnum>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ToggleBnbBurnOnUmFuturesTradeParams {
    pub fee_burn: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UmAccountTradeListParams {
    pub symbol: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UmPositionAdlQuantileEstimationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct UserDataStreamsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct MockUserDataStreamsApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub rebate_api_client: RebateApiClient,
}

#[derive(SimpleObject)]
pub struct GetSpotRebateHistoryRecordsResponseDataDataInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSpotRebateHistoryRecordsResponse {
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::GetSpotRebateHistoryRecordsResponseData>>,
}

#[derive(SimpleObject)]
pub struct GetSpotRebateHistoryRecordsResponseData {
    pub Option: :is_none")]
    pub page: Option<i64>,
    pub Option: :is_none")]
    pub total_records: Option<i64>,
    pub Option: :is_none")]
    pub total_page_num: Option<i64>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::GetSpotRebateHistoryRecordsResponseDataDataInner>>,
}

#[derive(SimpleObject)]
pub struct RebateApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetSpotRebateHistoryRecordsParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockRebateApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub mining_api_client: MiningApiClient,
}

#[derive(SimpleObject)]
pub struct HashrateResaleDetailResponseData {
    pub Option: :is_none"
    )]
    pub profit_transfer_details:
        Option<Vec<models::HashrateResaleDetailResponseDataProfitTransferDetailsInner>>,
    pub Option: :is_none")]
    pub total_num: Option<i64>,
    pub Option: :is_none")]
    pub page_size: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EarningsListResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::EarningsListResponseData>>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleListResponseData {
    pub Option: :is_none")]
    pub config_details: Option<Vec<models::HashrateResaleListResponseDataConfigDetailsInner>>,
    pub Option: :is_none")]
    pub total_num: Option<i64>,
    pub Option: :is_none")]
    pub page_size: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelHashrateResaleConfigurationResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<bool>,
}

#[derive(SimpleObject)]
pub struct RequestForMinerListResponseData {
    pub Option: :is_none")]
    pub worker_datas: Option<Vec<models::RequestForMinerListResponseDataWorkerDatasInner>>,
    pub Option: :is_none")]
    pub total_num: Option<i64>,
    pub Option: :is_none")]
    pub page_size: Option<i64>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleDetailResponseDataProfitTransferDetailsInner {
    pub Option: :is_none")]
    pub pool_username: Option<String>,
    pub Option: :is_none")]
    pub to_pool_username: Option<String>,
    pub Option: :is_none")]
    pub algo_name: Option<String>,
    pub Option: :is_none")]
    pub hash_rate: Option<i64>,
    pub Option: :is_none")]
    pub day: Option<i64>,
    pub Option: :is_none")]
    pub amount: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub coin_name: Option<String>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleListResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::HashrateResaleListResponseData>>,
}

#[derive(SimpleObject)]
pub struct RequestForDetailMinerListResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::RequestForDetailMinerListResponseDataInner>>,
}

#[derive(SimpleObject)]
pub struct AcquiringCoinnameResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::AcquiringCoinnameResponseDataInner>>,
}

#[derive(SimpleObject)]
pub struct RequestForDetailMinerListResponseDataInner {
    pub Option: :is_none")]
    pub worker_name: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub hashrate_datas:
        Option<Vec<models::RequestForDetailMinerListResponseDataInnerHashrateDatasInner>>,
}

#[derive(SimpleObject)]
pub struct EarningsListResponseData {
    pub Option: :is_none")]
    pub account_profits: Option<Vec<models::EarningsListResponseDataAccountProfitsInner>>,
    pub Option: :is_none")]
    pub total_num: Option<i64>,
    pub Option: :is_none")]
    pub page_size: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AcquiringAlgorithmResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::AcquiringAlgorithmResponseDataInner>>,
}

#[derive(SimpleObject)]
pub struct StatisticListResponseDataProfitToday {
    pub Option: :is_none")]
    pub btc: Option<String>,
    pub Option: :is_none")]
    pub bsv: Option<String>,
    pub Option: :is_none")]
    pub bch: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountListResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::AccountListResponseDataInner>>,
}

#[derive(SimpleObject)]
pub struct RequestForMinerListResponseDataWorkerDatasInner {
    pub Option: :is_none")]
    pub worker_id: Option<String>,
    pub Option: :is_none")]
    pub worker_name: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub hash_rate: Option<i64>,
    pub Option: :is_none")]
    pub day_hash_rate: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub reject_rate: Option<i64>,
    pub Option: :is_none")]
    pub last_share_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleListResponseDataConfigDetailsInner {
    pub Option: :is_none")]
    pub config_id: Option<i64>,
    pub Option: :is_none")]
    pub pool_username: Option<String>,
    pub Option: :is_none")]
    pub to_pool_username: Option<String>,
    pub Option: :is_none")]
    pub algo_name: Option<String>,
    pub Option: :is_none")]
    pub hash_rate: Option<i64>,
    pub Option: :is_none")]
    pub start_day: Option<i64>,
    pub Option: :is_none")]
    pub end_day: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RequestForDetailMinerListResponseDataInnerHashrateDatasInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub hashrate: Option<String>,
    pub Option: :is_none")]
    pub reject: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MiningAccountEarningResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::MiningAccountEarningResponseData>>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleRequestResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExtraBonusListResponseData {
    pub Option: :is_none")]
    pub other_profits: Option<Vec<models::ExtraBonusListResponseDataOtherProfitsInner>>,
    pub Option: :is_none")]
    pub total_num: Option<i64>,
    pub Option: :is_none")]
    pub page_size: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EarningsListResponseDataAccountProfitsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub hash_transfer: Option<i64>,
    pub Option: :is_none")]
    pub transfer_amount: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub day_hash_rate: Option<i64>,
    pub Option: :is_none")]
    pub profit_amount: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub coin_name: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleDetailResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::HashrateResaleDetailResponseData>>,
}

#[derive(SimpleObject)]
pub struct AccountListResponseDataInnerListInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub hashrate: Option<String>,
    pub Option: :is_none")]
    pub reject: Option<String>,
}

#[derive(SimpleObject)]
pub struct MiningAccountEarningResponseDataAccountProfitsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub coin_name: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub puid: Option<i64>,
    pub Option: :is_none")]
    pub sub_name: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct AcquiringAlgorithmResponseDataInner {
    pub Option: :is_none")]
    pub algo_name: Option<String>,
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub pool_index: Option<i64>,
    pub Option: :is_none")]
    pub unit: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExtraBonusListResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::ExtraBonusListResponseData>>,
}

#[derive(SimpleObject)]
pub struct AcquiringCoinnameResponseDataInner {
    pub Option: :is_none")]
    pub coin_name: Option<String>,
    pub Option: :is_none")]
    pub coin_id: Option<i64>,
    pub Option: :is_none")]
    pub pool_index: Option<i64>,
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub algo_name: Option<String>,
}

#[derive(SimpleObject)]
pub struct RequestForMinerListResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::RequestForMinerListResponseData>>,
}

#[derive(SimpleObject)]
pub struct StatisticListResponseData {
    pub Option: :is_none")]
    pub fifteen_min_hash_rate: Option<String>,
    pub Option: :is_none")]
    pub day_hash_rate: Option<String>,
    pub Option: :is_none")]
    pub valid_num: Option<i64>,
    pub Option: :is_none")]
    pub invalid_num: Option<i64>,
    pub Option: :is_none")]
    pub profit_today: Option<Box<models::StatisticListResponseDataProfitToday>>,
    pub Option: :is_none")]
    pub profit_yesterday: Option<Box<models::StatisticListResponseDataProfitToday>>,
    pub Option: :is_none")]
    pub user_name: Option<String>,
    pub Option: :is_none")]
    pub unit: Option<String>,
    pub Option: :is_none")]
    pub algo: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountListResponseDataInner {
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub user_name: Option<String>,
    pub Option: :is_none")]
    pub list: Option<Vec<models::AccountListResponseDataInnerListInner>>,
}

#[derive(SimpleObject)]
pub struct MiningAccountEarningResponseData {
    pub Option: :is_none")]
    pub account_profits: Option<Vec<models::MiningAccountEarningResponseDataAccountProfitsInner>>,
    pub Option: :is_none")]
    pub total_num: Option<i64>,
    pub Option: :is_none")]
    pub page_size: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExtraBonusListResponseDataOtherProfitsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub coin_name: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub profit_amount: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub status: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StatisticListResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::StatisticListResponseData>>,
}

#[derive(SimpleObject)]
pub struct MiningApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountListParams {
    pub algo: String,
    pub user_name: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelHashrateResaleConfigurationParams {
    pub config_id: i64,
    pub user_name: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EarningsListParams {
    pub algo: String,
    pub user_name: String,
    pub coin: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExtraBonusListParams {
    pub algo: String,
    pub user_name: String,
    pub coin: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleDetailParams {
    pub config_id: i64,
    pub user_name: String,
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleListParams {
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct HashrateResaleRequestParams {
    pub user_name: String,
    pub algo: String,
    pub end_date: i64,
    pub start_date: i64,
    pub to_pool_user: String,
    pub hash_rate: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MiningAccountEarningParams {
    pub algo: String,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub page_index: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RequestForDetailMinerListParams {
    pub algo: String,
    pub user_name: String,
    pub worker_name: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RequestForMinerListParams {
    pub algo: String,
    pub user_name: String,
    pub page_index: Option<i64>,
    pub sort: Option<i64>,
    pub 1: miner name,
    pub 2: real-time computing power,
    pub 3: daily average computing power,
    pub 4: real-time rejection rate,
    pub 5: last submission time
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub sort_column: Option<i64>,
    pub worker_status: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StatisticListParams {
    pub algo: String,
    pub user_name: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMiningApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub future_copy_trading_api_client: FutureCopyTradingApiClient,
}

#[derive(SimpleObject)]
pub struct GetFuturesLeadTradingSymbolWhitelistResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::GetFuturesLeadTradingSymbolWhitelistResponseDataInner>>,
}

#[derive(SimpleObject)]
pub struct GetFuturesLeadTraderStatusResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::GetFuturesLeadTraderStatusResponseData>>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetFuturesLeadTraderStatusResponseData {
    pub Option: :is_none")]
    pub is_lead_trader: Option<bool>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesLeadTradingSymbolWhitelistResponseDataInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct FutureCopyTradingApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetFuturesLeadTraderStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesLeadTradingSymbolWhitelistParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockFutureCopyTradingApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub c2_c_api_client: C2CApiClient,
}

#[derive(SimpleObject)]
pub struct GetC2CTradeHistoryResponseDataInner {
    pub Option: :is_none")]
    pub order_number: Option<String>,
    pub Option: :is_none")]
    pub adv_no: Option<String>,
    pub Option: :is_none")]
    pub trade_type: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub fiat: Option<String>,
    pub Option: :is_none")]
    pub fiat_symbol: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub total_price: Option<String>,
    pub Option: :is_none")]
    pub unit_price: Option<String>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none"
    )]
    pub counter_part_nick_name: Option<String>,
    pub Option: :is_none")]
    pub advertisement_role: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetC2CTradeHistoryResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::GetC2CTradeHistoryResponseDataInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct C2CApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetC2CTradeHistoryParams {
    pub trade_type: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
    pub page: Option<i64>,
    pub rows: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockC2CApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub flexible_rate_api_client: FlexibleRateApiClient,
    pub stable_rate_api_client: StableRateApiClient,
}

#[derive(SimpleObject)]
pub struct CheckCollateralRepayRateStableRateResponse {
    pub Option: :is_none")]
    pub loanl_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub repay_amount: Option<String>,
    pub Option: :is_none")]
    pub rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLoanLtvAdjustmentHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLoanLtvAdjustmentHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanLtvAdjustmentHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub direction: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub pre_ltv: Option<String>,
    pub Option: :is_none")]
    pub after_ltv: Option<String>,
    pub Option: :is_none")]
    pub adjust_time: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanRepaymentHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLoanRepaymentHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanBorrowHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub initial_loan_amount: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none"
    )]
    pub initial_collateral_amount: Option<String>,
    pub Option: :is_none")]
    pub borrow_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct FlexibleLoanBorrowResponse {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub loan_amount: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_amount: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanAssetsDataResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none"
    )]
    pub flexible_interest_rate: Option<String>,
    pub Option: :is_none")]
    pub flexible_min_limit: Option<String>,
    pub Option: :is_none")]
    pub flexible_max_limit: Option<String>,
}

#[derive(SimpleObject)]
pub struct CheckCollateralRepayRateResponse {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanLiquidationHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleLoanLiquidationHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanRepaymentHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub repay_amount: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_return: Option<String>,
    pub Option: :is_none")]
    pub repay_status: Option<String>,
    pub Option: :is_none")]
    pub repay_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanRepaymentHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub repay_amount: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_used: Option<String>,
    pub Option: :is_none")]
    pub collateral_return: Option<String>,
    pub Option: :is_none")]
    pub repay_type: Option<String>,
    pub Option: :is_none")]
    pub repay_status: Option<String>,
    pub Option: :is_none")]
    pub repay_time: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanCollateralAssetsDataResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleLoanCollateralAssetsDataResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FlexibleLoanAdjustLtvResponse {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub direction: Option<String>,
    pub Option: :is_none")]
    pub adjustment_amount: Option<String>,
    pub Option: :is_none")]
    pub current_ltv: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct FlexibleLoanRepayResponse {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub remaining_debt: Option<String>,
    pub Option: :is_none"
    )]
    pub remaining_collateral: Option<String>,
    pub Option: :is_none")]
    pub full_repayment: Option<bool>,
    pub Option: :is_none")]
    pub current_ltv: Option<String>,
    pub Option: :is_none")]
    pub repay_status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanLiquidationHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub liquidation_debt: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none"
    )]
    pub liquidation_collateral_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub return_collateral_amount: Option<String>,
    pub Option: :is_none")]
    pub liquidation_fee: Option<String>,
    pub Option: :is_none"
    )]
    pub liquidation_starting_price: Option<String>,
    pub Option: :is_none"
    )]
    pub liquidation_starting_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanLtvAdjustmentHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleLoanLtvAdjustmentHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanRepaymentHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleLoanRepaymentHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanBorrowHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLoanBorrowHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCryptoLoansIncomeHistoryResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLoanBorrowHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub initial_loan_amount: Option<String>,
    pub Option: :is_none")]
    pub hourly_interest_rate: Option<String>,
    pub Option: :is_none")]
    pub loan_term: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none"
    )]
    pub initial_collateral_amount: Option<String>,
    pub Option: :is_none")]
    pub borrow_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanCollateralAssetsDataResponseRowsInner {
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub initial_ltv: Option<String>,
    pub Option: :is_none")]
    pub margin_call_ltv: Option<String>,
    pub Option: :is_none")]
    pub liquidation_ltv: Option<String>,
    pub Option: :is_none")]
    pub max_limit: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanOngoingOrdersResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleLoanOngoingOrdersResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanAssetsDataResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleLoanAssetsDataResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanLtvAdjustmentHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub direction: Option<String>,
    pub Option: :is_none")]
    pub collateral_amount: Option<String>,
    pub Option: :is_none")]
    pub pre_ltv: Option<String>,
    pub Option: :is_none")]
    pub after_ltv: Option<String>,
    pub Option: :is_none")]
    pub adjust_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanOngoingOrdersResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub total_debt: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub collateral_amount: Option<String>,
    pub Option: :is_none")]
    pub current_ltv: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanBorrowHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleLoanBorrowHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StableRateApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CheckCollateralRepayRateStableRateParams {
    pub loan_coin: String,
    pub collateral_coin: String,
    pub repay_amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCryptoLoansIncomeHistoryParams {
    pub asset: Option<String>,
    pub type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanBorrowHistoryParams {
    pub order_id: Option<i64>,
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanLtvAdjustmentHistoryParams {
    pub order_id: Option<i64>,
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanRepaymentHistoryParams {
    pub order_id: Option<i64>,
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockStableRateApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct FlexibleRateApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CheckCollateralRepayRateParams {
    pub loan_coin: String,
    pub collateral_coin: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FlexibleLoanAdjustLtvParams {
    pub loan_coin: String,
    pub collateral_coin: String,
    pub adjustment_amount: rust_decimal::Decimal,
    pub direction: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FlexibleLoanBorrowParams {
    pub loan_coin: String,
    pub collateral_coin: String,
    pub loan_amount: Option<rust_decimal::Decimal>,
    pub collateral_amount: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FlexibleLoanRepayParams {
    pub loan_coin: String,
    pub collateral_coin: String,
    pub repay_amount: rust_decimal::Decimal,
    pub Default: TRUE. TRUE: Return extra collateral to spot account; FALSE: Keep extra collateral in the order,
    pub collateral_return: Option<bool>,
    pub Default: FALSE. TRUE: Full repayment; FALSE: Partial repayment,
    pub full_repayment: Option<bool>,
    pub Default: 1. 1: Repayment with loan asset; 2: Repayment with collateral
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub repayment_type: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanAssetsDataParams {
    pub loan_coin: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanBorrowHistoryParams {
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanCollateralAssetsDataParams {
    pub collateral_coin: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanLiquidationHistoryParams {
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanLtvAdjustmentHistoryParams {
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanOngoingOrdersParams {
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleLoanRepaymentHistoryParams {
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub default: 1; max: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10; max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockFlexibleRateApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub fiat_api_client: FiatApiClient,
}

#[derive(SimpleObject)]
pub struct GetFiatPaymentsHistoryResponseDataInner {
    pub Option: :is_none")]
    pub order_no: Option<String>,
    pub Option: :is_none")]
    pub source_amount: Option<String>,
    pub Option: :is_none")]
    pub fiat_currency: Option<String>,
    pub Option: :is_none")]
    pub obtain_amount: Option<String>,
    pub Option: :is_none")]
    pub crypto_currency: Option<String>,
    pub Option: :is_none")]
    pub total_fee: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub payment_method: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFiatPaymentsHistoryResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::GetFiatPaymentsHistoryResponseDataInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetFiatDepositWithdrawHistoryResponseDataInner {
    pub Option: :is_none")]
    pub order_no: Option<String>,
    pub Option: :is_none")]
    pub fiat_currency: Option<String>,
    pub Option: :is_none")]
    pub indicated_amount: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub total_fee: Option<String>,
    pub Option: :is_none")]
    pub method: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFiatDepositWithdrawHistoryResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::GetFiatDepositWithdrawHistoryResponseDataInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct FiatApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetFiatDepositWithdrawHistoryParams {
    pub transaction_type: String,
    pub begin_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub rows: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFiatPaymentsHistoryParams {
    pub transaction_type: String,
    pub begin_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub rows: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockFiatApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub borrow_repay_api_client: BorrowRepayApiClient,
    pub market_data_api_client: MarketDataApiClient,
    pub risk_data_stream_api_client: RiskDataStreamApiClient,
    pub trade_api_client: TradeApiClient,
    pub transfer_api_client: TransferApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub websocket_streams_base: Arc<WebsocketStreamsBase>,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsHandle {
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct Balanceupdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginLevelStatusChange {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
}

#[derive(SimpleObject)]
pub struct Executionreport {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub f_uppercase: Option<String>,
    pub Option: :is_none")]
    pub g: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<String>,
    pub Option: :is_none")]
    pub n_uppercase: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub i_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub w: Option<bool>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub m_uppercase: Option<bool>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub z_uppercase: Option<String>,
    pub Option: :is_none")]
    pub y_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub d_uppercase: Option<String>,
    pub Option: :is_none")]
    pub j: Option<String>,
    pub Option: :is_none")]
    pub j_uppercase: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub u: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<String>,
    pub Option: :is_none")]
    pub cs: Option<String>,
    pub Option: :is_none")]
    pub pl: Option<String>,
    pub Option: :is_none")]
    pub p_l: Option<String>,
    pub Option: :is_none")]
    pub p_y: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub k: Option<String>,
    pub Option: :is_none")]
    pub u_s: Option<bool>,
}

#[derive(SimpleObject)]
pub struct Liststatus {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub g: Option<i64>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub c_uppercase: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o_uppercase: Option<Vec<models::ListstatusOInner>>,
}

#[derive(SimpleObject)]
pub struct OutboundaccountpositionBInner {
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
}

#[derive(SimpleObject)]
pub struct ListstatusOInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub c: Option<String>,
}

#[derive(SimpleObject)]
pub struct UserLiabilityChange {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub t: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
}

#[derive(SimpleObject)]
pub struct Listenkeyexpired {
    pub Option: :is_none")]
    pub e_uppercase: Option<String>,
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct Outboundaccountposition {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub b_uppercase: Option<Vec<models::OutboundaccountpositionBInner>>,
}

#[derive(SimpleObject)]
pub struct GetDelistScheduleResponseInner {
    pub Option: :is_none")]
    pub delist_time: Option<i64>,
    pub Option: :is_none")]
    pub cross_margin_assets: Option<Vec<String>>,
    pub Option: :is_none"
    )]
    pub isolated_margin_symbols: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct CreateSpecialKeyResponse {
    pub Option: :is_none")]
    pub api_key: Option<String>,
    pub Option: :is_none")]
    pub secret_key: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryLiabilityCoinLeverageBracketInCrossMarginProModeResponseInnerBracketsInner {
    pub Option: :is_none")]
    pub leverage: Option<i64>,
    pub Option: :is_none")]
    pub max_debt: Option<rust_decimal::Decimal>,
    pub Option: :is_none"
    )]
    pub maintenance_margin_rate: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub initial_margin_rate: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub fast_num: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct QueryBorrowRepayRecordsInMarginAccountResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryBorrowRepayRecordsInMarginAccountResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginPriceindexResponse {
    pub Option: :is_none")]
    pub calc_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct DisableIsolatedMarginAccountResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginAccountInfoResponseAssetsInnerQuoteAsset {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub borrow_enabled: Option<bool>,
    pub Option: :is_none")]
    pub borrowed: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub net_asset: Option<String>,
    pub Option: :is_none")]
    pub net_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub repay_enabled: Option<bool>,
    pub Option: :is_none")]
    pub total_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOrdersResponseInner {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOcoResponseInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryMarginAccountsOpenOcoResponseInnerOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelOcoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::MarginAccountCancelOcoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::MarginAccountCancelOcoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct QuerySpecialKeyListResponseInner {
    pub Option: :is_none")]
    pub api_name: Option<String>,
    pub Option: :is_none")]
    pub api_key: Option<String>,
    pub Option: :is_none")]
    pub ip: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub permission_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_amount: Option<i64>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_asset: Option<String>,
    pub Option: :is_none")]
    pub fills: Option<Vec<models::MarginAccountNewOrderResponseFillsInner>>,
}

#[derive(SimpleObject)]
pub struct GetCrossMarginTransferHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetCrossMarginTransferHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSmallLiabilityExchangeHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetSmallLiabilityExchangeHistoryResponseRowsInner>>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtocoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::MarginAccountNewOtocoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::MarginAccountNewOtocoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOcoResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QuerySpecialKeyResponse {
    pub Option: :is_none")]
    pub api_key: Option<String>,
    pub Option: :is_none")]
    pub ip: Option<String>,
    pub Option: :is_none")]
    pub api_name: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub permission_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOcoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryMarginAccountsOcoResponseOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct QueryLiabilityCoinLeverageBracketInCrossMarginProModeResponseInner {
    pub Option: :is_none")]
    pub asset_names: Option<Vec<String>>,
    pub Option: :is_none")]
    pub rank: Option<i64>,
    pub Option: :is_none")]
    pub brackets: Option<
        Vec<
            models::QueryLiabilityCoinLeverageBracketInCrossMarginProModeResponseInnerBracketsInner,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelOcoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginAccountInfoResponseAssetsInnerBaseAsset {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub borrow_enabled: Option<bool>,
    pub Option: :is_none")]
    pub borrowed: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub net_asset: Option<String>,
    pub Option: :is_none")]
    pub net_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub repay_enabled: Option<bool>,
    pub Option: :is_none")]
    pub total_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAvailableInventoryResponseAssets {
    pub Option: :is_none")]
    pub matic: Option<String>,
    pub Option: :is_none")]
    pub stpt: Option<String>,
    pub Option: :is_none")]
    pub tvk: Option<String>,
    pub Option: :is_none")]
    pub shib: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetAllMarginAssetsResponseInner {
    pub Option: :is_none")]
    pub asset_full_name: Option<String>,
    pub Option: :is_none")]
    pub asset_name: Option<String>,
    pub Option: :is_none")]
    pub is_borrowable: Option<bool>,
    pub Option: :is_none")]
    pub is_mortgageable: Option<bool>,
    pub Option: :is_none")]
    pub user_min_borrow: Option<String>,
    pub Option: :is_none")]
    pub user_min_repay: Option<String>,
    pub Option: :is_none")]
    pub delist_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginAccountInfoResponse {
    pub Option: :is_none")]
    pub assets: Option<Vec<models::QueryIsolatedMarginAccountInfoResponseAssetsInner>>,
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetAllCrossMarginPairsResponseInner {
    pub Option: :is_none")]
    pub base: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub is_buy_allowed: Option<bool>,
    pub Option: :is_none")]
    pub is_margin_trade: Option<bool>,
    pub Option: :is_none")]
    pub is_sell_allowed: Option<bool>,
    pub Option: :is_none")]
    pub quote: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub delist_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFutureHourlyInterestRateResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none"
    )]
    pub next_hourly_interest_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct AdjustCrossMarginMaxLeverageResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct QueryCrossMarginFeeDataResponseInner {
    pub Option: :is_none")]
    pub vip_level: Option<i64>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub transfer_in: Option<bool>,
    pub Option: :is_none")]
    pub borrowable: Option<bool>,
    pub Option: :is_none")]
    pub daily_interest: Option<String>,
    pub Option: :is_none")]
    pub yearly_interest: Option<String>,
    pub Option: :is_none")]
    pub borrow_limit: Option<String>,
    pub Option: :is_none")]
    pub marginable_pairs: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtocoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryBorrowRepayRecordsInMarginAccountResponseRowsInner {
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub isolated_symbol: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub tx_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSmallLiabilityExchangeCoinListResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
    pub Option: :is_none")]
    pub liability_asset: Option<String>,
    pub Option: :is_none")]
    pub liability_qty: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsTradeListResponseInner {
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub is_best_match: Option<bool>,
    pub Option: :is_none")]
    pub is_buyer: Option<bool>,
    pub Option: :is_none")]
    pub is_maker: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetForceLiquidationRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetForceLiquidationRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelOcoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginTierDataResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub tier: Option<i64>,
    pub Option: :is_none")]
    pub effective_multiple: Option<String>,
    pub Option: :is_none")]
    pub initial_risk_ratio: Option<String>,
    pub Option: :is_none"
    )]
    pub liquidation_risk_ratio: Option<String>,
    pub Option: :is_none"
    )]
    pub base_asset_max_borrowable: Option<String>,
    pub Option: :is_none"
    )]
    pub quote_asset_max_borrowable: Option<String>,
}

#[derive(SimpleObject)]
pub struct CrossMarginCollateralRatioResponseInner {
    pub Option: :is_none")]
    pub collaterals: Option<Vec<models::CrossMarginCollateralRatioResponseInnerCollateralsInner>>,
    pub Option: :is_none")]
    pub asset_names: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct QueryCrossIsolatedMarginCapitalFlowResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfMarginAccountResponse {
    pub Option: :is_none")]
    pub normal_bar: Option<String>,
    pub Option: :is_none")]
    pub margin_call_bar: Option<String>,
    pub Option: :is_none"
    )]
    pub force_liquidation_bar: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAvailableInventoryResponse {
    pub Option: :is_none")]
    pub assets: Option<Box<models::QueryMarginAvailableInventoryResponseAssets>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponse {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryEnabledIsolatedMarginAccountLimitResponse {
    pub Option: :is_none")]
    pub enabled_account: Option<i64>,
    pub Option: :is_none")]
    pub max_account: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCrossMarginTransferHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub tx_id: Option<i64>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub trans_from: Option<String>,
    pub Option: :is_none")]
    pub trans_to: Option<String>,
    pub Option: :is_none")]
    pub from_symbol: Option<String>,
    pub Option: :is_none")]
    pub to_symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentMarginOrderCountUsageResponseInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginFeeDataResponseInner {
    pub Option: :is_none")]
    pub vip_level: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::QueryIsolatedMarginFeeDataResponseInnerDataInner>>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOrdersResponseInner {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountBorrowRepayResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CrossMarginCollateralRatioResponseInnerCollateralsInner {
    pub Option: :is_none")]
    pub min_usd_value: Option<String>,
    pub Option: :is_none")]
    pub max_usd_value: Option<String>,
    pub Option: :is_none")]
    pub discount_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub margin_buy_borrow_asset: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::MarginAccountNewOcoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::MarginAccountNewOcoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolResponseInnerOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMaxTransferOutAmountResponse {
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOcoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtocoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOrderResponseFillsInner {
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCrossMarginAccountDetailsResponseUserAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub borrowed: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub net_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetListScheduleResponseInner {
    pub Option: :is_none")]
    pub list_time: Option<i64>,
    pub Option: :is_none")]
    pub cross_margin_assets: Option<Vec<String>>,
    pub Option: :is_none"
    )]
    pub isolated_margin_symbols: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOcoResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetForceLiquidationRecordResponseRowsInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub updated_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginAccountInfoResponseAssetsInner {
    pub Option: :is_none")]
    pub base_asset: Option<Box<models::QueryIsolatedMarginAccountInfoResponseAssetsInnerBaseAsset>>,
    pub Option: :is_none")]
    pub quote_asset:
        Option<Box<models::QueryIsolatedMarginAccountInfoResponseAssetsInnerQuoteAsset>>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub isolated_created: Option<bool>,
    pub Option: :is_none")]
    pub enabled: Option<bool>,
    pub Option: :is_none")]
    pub margin_level: Option<String>,
    pub Option: :is_none")]
    pub margin_level_status: Option<String>,
    pub Option: :is_none")]
    pub margin_ratio: Option<String>,
    pub Option: :is_none")]
    pub index_price: Option<String>,
    pub Option: :is_none")]
    pub liquidate_price: Option<String>,
    pub Option: :is_none")]
    pub liquidate_rate: Option<String>,
    pub Option: :is_none")]
    pub trade_enabled: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetSmallLiabilityExchangeHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub target_asset: Option<String>,
    pub Option: :is_none")]
    pub target_amount: Option<String>,
    pub Option: :is_none")]
    pub biz_type: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOcoResponseInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryMarginAccountsAllOcoResponseInnerOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct GetAllIsolatedMarginSymbolResponseInner {
    pub Option: :is_none")]
    pub base: Option<String>,
    pub Option: :is_none")]
    pub is_buy_allowed: Option<bool>,
    pub Option: :is_none")]
    pub is_margin_trade: Option<bool>,
    pub Option: :is_none")]
    pub is_sell_allowed: Option<bool>,
    pub Option: :is_none")]
    pub quote: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginFeeDataResponseInnerDataInner {
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub daily_interest: Option<String>,
    pub Option: :is_none")]
    pub borrow_limit: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLimitPricePairsResponse {
    pub Option: :is_none")]
    pub cross_margin_symbols: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct EnableIsolatedMarginAccountResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetInterestHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetInterestHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginManualLiquidationResponse {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
    pub Option: :is_none")]
    pub liability_asset: Option<String>,
    pub Option: :is_none")]
    pub liability_qty: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct GetBnbBurnStatusResponse {
    pub Option: :is_none")]
    pub spot_bnb_burn: Option<bool>,
    pub Option: :is_none")]
    pub interest_bnb_burn: Option<bool>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::MarginAccountNewOtoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::MarginAccountNewOtoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub orders:
        Option<Vec<models::MarginAccountCancelAllOpenOrdersOnASymbolResponseInnerOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<
        Vec<models::MarginAccountCancelAllOpenOrdersOnASymbolResponseInnerOrderReportsInner>,
}

#[derive(SimpleObject)]
pub struct QueryMaxBorrowResponse {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub borrow_limit: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginInterestRateHistoryResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub daily_interest_rate: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
    pub Option: :is_none")]
    pub vip_level: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub is_isolated: Option<bool>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCrossMarginAccountDetailsResponse {
    pub Option: :is_none")]
    pub created: Option<bool>,
    pub Option: :is_none")]
    pub borrow_enabled: Option<bool>,
    pub Option: :is_none")]
    pub margin_level: Option<String>,
    pub Option: :is_none"
    )]
    pub collateral_margin_level: Option<String>,
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_collateral_value_in_usdt: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_loss_in_usdt: Option<String>,
    pub Option: :is_none")]
    pub trade_enabled: Option<bool>,
    pub Option: :is_none")]
    pub transfer_in_enabled: Option<bool>,
    pub Option: :is_none")]
    pub transfer_out_enabled: Option<bool>,
    pub Option: :is_none")]
    pub account_type: Option<String>,
    pub Option: :is_none")]
    pub user_assets: Option<Vec<models::QueryCrossMarginAccountDetailsResponseUserAssetsInner>>,
}

#[derive(SimpleObject)]
pub struct GetInterestHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub tx_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub interest_accured_time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub raw_asset: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub interest_rate: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub isolated_symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AdjustCrossMarginMaxLeverageParams {
    pub Example: maxLeverage = 5 or 3 for Cross Margin Classic; maxLeverage=10 for Cross Margin Pro 10x leverage or 20x if compliance allows.
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub max_leverage: i64,
}

#[derive(SimpleObject)]
pub struct DisableIsolatedMarginAccountParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EnableIsolatedMarginAccountParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBnbBurnStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfMarginAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCrossIsolatedMarginCapitalFlowParams {
    pub asset: Option<String>,
    pub symbol: Option<String>,
    pub Type: `ROLL_IN`,
    pub type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub Value: 500; Max Value: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCrossMarginAccountDetailsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCrossMarginFeeDataParams {
    pub vip_level: Option<i64>,
    pub coin: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryEnabledIsolatedMarginAccountLimitParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginAccountInfoParams {
    pub symbols: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginFeeDataParams {
    pub vip_level: Option<i64>,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CreateSpecialKeyParams {
    pub api_name: String,
    pub symbol: Option<String>,
    pub ip: Option<String>,
    pub public_key: Option<String>,
    pub permission_mode: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DeleteSpecialKeyParams {
    pub api_name: Option<String>,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EditIpForSpecialKeyParams {
    pub ip: String,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetForceLiquidationRecordParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub isolated_symbol: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSmallLiabilityExchangeCoinListParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSmallLiabilityExchangeHistoryParams {
    pub Default: 1
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub current: i64,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub size: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelAllOpenOrdersOnASymbolParams {
    pub symbol: String,
    pub is_isolated: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelOcoParams {
    pub symbol: String,
    pub is_isolated: Option<String>,
    pub order_list_id: Option<i64>,
    pub list_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountCancelOrderParams {
    pub symbol: String,
    pub is_isolated: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOcoParams {
    pub symbol: String,
    pub side: MarginAccountNewOcoSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub stop_price: rust_decimal::Decimal,
    pub is_isolated: Option<String>,
    pub list_client_order_id: Option<String>,
    pub limit_client_order_id: Option<String>,
    pub limit_iceberg_qty: Option<rust_decimal::Decimal>,
    pub stop_client_order_id: Option<String>,
    pub stop_limit_price: Option<rust_decimal::Decimal>,
    pub stop_iceberg_qty: Option<rust_decimal::Decimal>,
    pub stop_limit_time_in_force: Option<String>,
    pub new_order_resp_type: Option<MarginAccountNewOcoNewOrderRespTypeEnum>,
    pub https: //www.binance.com/en/support/faq/how-to-use-the-sideeffecttype-parameter-with-the-margin-order-endpoints-f9fc51cda1984bf08b95e0d96c4570bc)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub side_effect_type: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub auto_repay_at_cancel: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOrderParams {
    pub symbol: String,
    pub side: MarginAccountNewOrderSideEnum,
    pub type: String,
    pub is_isolated: Option<String>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub new_order_resp_type: Option<MarginAccountNewOrderNewOrderRespTypeEnum>,
    pub https: //www.binance.com/en/support/faq/how-to-use-the-sideeffecttype-parameter-with-the-margin-order-endpoints-f9fc51cda1984bf08b95e0d96c4570bc)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub side_effect_type: Option<String>,
    pub time_in_force: Option<MarginAccountNewOrderTimeInForceEnum>,
    pub self_trade_prevention_mode: Option<String>,
    pub auto_repay_at_cancel: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtoParams {
    pub symbol: String,
    pub values: `LIMIT`,
    pub working_type: String,
    pub working_side: String,
    pub working_price: rust_decimal::Decimal,
    pub working_quantity: rust_decimal::Decimal,
    pub working_iceberg_qty: rust_decimal::Decimal,
    pub values: [Order Types](https://developers.binance.com/docs/binance-spot-api-docs/enums#order-types-ordertypes-type) Note that `MARKET` orders using `quoteOrderQty` are not supported.
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub pending_type: String,
    pub pending_side: String,
    pub pending_quantity: rust_decimal::Decimal,
    pub is_isolated: Option<String>,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<MarginAccountNewOtoNewOrderRespTypeEnum>,
    pub https: //www.binance.com/en/support/faq/how-to-use-the-sideeffecttype-parameter-with-the-margin-order-endpoints-f9fc51cda1984bf08b95e0d96c4570bc)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub side_effect_type: Option<String>,
    pub self_trade_prevention_mode: Option<String>,
    pub auto_repay_at_cancel: Option<bool>,
    pub working_client_order_id: Option<String>,
    pub working_time_in_force: Option<String>,
    pub pending_client_order_id: Option<String>,
    pub pending_price: Option<rust_decimal::Decimal>,
    pub pending_stop_price: Option<rust_decimal::Decimal>,
    pub pending_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_time_in_force: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginAccountNewOtocoParams {
    pub symbol: String,
    pub values: `LIMIT`,
    pub working_type: String,
    pub working_side: String,
    pub working_price: rust_decimal::Decimal,
    pub working_quantity: rust_decimal::Decimal,
    pub pending_side: String,
    pub pending_quantity: rust_decimal::Decimal,
    pub values: `LIMIT_MAKER`,
    pub pending_above_type: String,
    pub is_isolated: Option<String>,
    pub https: //www.binance.com/en/support/faq/how-to-use-the-sideeffecttype-parameter-with-the-margin-order-endpoints-f9fc51cda1984bf08b95e0d96c4570bc)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub side_effect_type: Option<String>,
    pub auto_repay_at_cancel: Option<bool>,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<MarginAccountNewOtocoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<String>,
    pub working_client_order_id: Option<String>,
    pub working_iceberg_qty: Option<rust_decimal::Decimal>,
    pub working_time_in_force: Option<String>,
    pub pending_above_client_order_id: Option<String>,
    pub pending_above_price: Option<rust_decimal::Decimal>,
    pub pending_above_stop_price: Option<rust_decimal::Decimal>,
    pub pending_above_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_above_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_above_time_in_force: Option<String>,
    pub values: `LIMIT_MAKER`,
    pub pending_below_type: Option<String>,
    pub pending_below_client_order_id: Option<String>,
    pub pending_below_price: Option<rust_decimal::Decimal>,
    pub pending_below_stop_price: Option<rust_decimal::Decimal>,
    pub pending_below_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_below_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_below_time_in_force: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginManualLiquidationParams {
    pub type: String,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentMarginOrderCountUsageParams {
    pub is_isolated: Option<String>,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOcoParams {
    pub is_isolated: Option<String>,
    pub symbol: Option<String>,
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Value: 500; Max Value: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsAllOrdersParams {
    pub symbol: String,
    pub is_isolated: Option<String>,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Value: 500; Max Value: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOcoParams {
    pub is_isolated: Option<String>,
    pub symbol: Option<String>,
    pub order_list_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOcoParams {
    pub is_isolated: Option<String>,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOpenOrdersParams {
    pub symbol: Option<String>,
    pub is_isolated: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsOrderParams {
    pub symbol: String,
    pub is_isolated: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAccountsTradeListParams {
    pub symbol: String,
    pub is_isolated: Option<String>,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub Value: 500; Max Value: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySpecialKeyParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySpecialKeyListParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SmallLiabilityExchangeParams {
    pub Example: assetNames = BTC,
    pub asset_names: Vec<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetAllCrossMarginPairsParams {
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetAllIsolatedMarginSymbolParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetAllMarginAssetsParams {
    pub asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetDelistScheduleParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetListScheduleParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIsolatedMarginTierDataParams {
    pub symbol: String,
    pub tier: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginAvailableInventoryParams {
    pub type: String,
}

#[derive(SimpleObject)]
pub struct QueryMarginPriceindexParams {
    pub symbol: String,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct BorrowRepayApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetFutureHourlyInterestRateParams {
    pub assets: String,
    pub is_isolated: bool,
}

#[derive(SimpleObject)]
pub struct GetInterestHistoryParams {
    pub asset: Option<String>,
    pub isolated_symbol: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginAccountBorrowRepayParams {
    pub asset: String,
    pub is_isolated: String,
    pub symbol: String,
    pub amount: String,
    pub type: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryBorrowRepayRecordsInMarginAccountParams {
    pub type: String,
    pub asset: Option<String>,
    pub isolated_symbol: Option<String>,
    pub tx_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMarginInterestRateHistoryParams {
    pub asset: String,
    pub vip_level: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMaxBorrowParams {
    pub asset: String,
    pub isolated_symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockBorrowRepayApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RiskDataStreamApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamParams {
    pub listen_key: String,
}

#[derive(SimpleObject)]
pub struct MockRiskDataStreamApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TransferApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetCrossMarginTransferHistoryParams {
    pub asset: Option<String>,
    pub Type: `ROLL_IN`,
    pub type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub isolated_symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryMaxTransferOutAmountParams {
    pub asset: String,
    pub isolated_symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTransferApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub market_data_api_client: MarketDataApiClient,
    pub trade_api_client: TradeApiClient,
}

#[derive(SimpleObject)]
pub struct PlaceLimitOrderResponse {
    pub Option: :is_none")]
    pub quote_id: Option<String>,
    pub Option: :is_none")]
    pub ratio: Option<String>,
    pub Option: :is_none")]
    pub inverse_ratio: Option<String>,
    pub Option: :is_none")]
    pub valid_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct ListAllConvertPairsResponseInner {
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub from_asset_min_amount: Option<String>,
    pub Option: :is_none")]
    pub from_asset_max_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset_min_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset_max_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetConvertTradeHistoryResponse {
    pub Option: :is_none")]
    pub list: Option<Vec<models::GetConvertTradeHistoryResponseListInner>>,
    pub Option: :is_none")]
    pub start_time: Option<i64>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub more_data: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetConvertTradeHistoryResponseListInner {
    pub Option: :is_none")]
    pub quote_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub ratio: Option<String>,
    pub Option: :is_none")]
    pub inverse_ratio: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SendQuoteRequestResponse {
    pub Option: :is_none")]
    pub quote_id: Option<String>,
    pub Option: :is_none")]
    pub ratio: Option<String>,
    pub Option: :is_none")]
    pub inverse_ratio: Option<String>,
    pub Option: :is_none")]
    pub valid_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryLimitOpenOrdersResponseListInner {
    pub Option: :is_none")]
    pub quote_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub ratio: Option<String>,
    pub Option: :is_none")]
    pub inverse_ratio: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub expired_timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderQuantityPrecisionPerAssetResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub fraction: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryLimitOpenOrdersResponse {
    pub Option: :is_none")]
    pub list: Option<Vec<models::QueryLimitOpenOrdersResponseListInner>>,
}

#[derive(SimpleObject)]
pub struct CancelLimitOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderStatusResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub ratio: Option<String>,
    pub Option: :is_none")]
    pub inverse_ratio: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AcceptQuoteResponse {
    pub Option: :is_none")]
    pub order_id: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AcceptQuoteParams {
    pub quote_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelLimitOrderParams {
    pub order_id: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetConvertTradeHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderStatusParams {
    pub order_id: Option<String>,
    pub quote_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct PlaceLimitOrderParams {
    pub base_asset: String,
    pub quote_asset: String,
    pub limit_price: rust_decimal::Decimal,
    pub side: String,
    pub expired_type: String,
    pub base_amount: Option<rust_decimal::Decimal>,
    pub quote_amount: Option<rust_decimal::Decimal>,
    pub wallet_type: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryLimitOpenOrdersParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SendQuoteRequestParams {
    pub from_asset: String,
    pub to_asset: String,
    pub from_amount: Option<rust_decimal::Decimal>,
    pub to_amount: Option<rust_decimal::Decimal>,
    pub wallet_type: Option<String>,
    pub valid_time: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct ListAllConvertPairsParams {
    pub from_asset: Option<String>,
    pub to_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryOrderQuantityPrecisionPerAssetParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_management_api_client: AccountManagementApiClient,
    pub api_management_api_client: ApiManagementApiClient,
    pub asset_management_api_client: AssetManagementApiClient,
    pub managed_sub_account_api_client: ManagedSubAccountApiClient,
}

#[derive(SimpleObject)]
pub struct QueryUniversalTransferHistoryResponseResultInner {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub from_email: Option<String>,
    pub Option: :is_none")]
    pub to_email: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub create_time_stamp: Option<i64>,
    pub Option: :is_none")]
    pub from_account_type: Option<String>,
    pub Option: :is_none")]
    pub to_account_type: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_tran_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetIpRestrictionForASubAccountApiKeyResponse {
    pub Option: :is_none")]
    pub ip_restrict: Option<String>,
    pub Option: :is_none")]
    pub ip_list: Option<Vec<String>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub api_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct MovePositionForSubAccountResponse {
    pub Option: :is_none")]
    pub move_position_orders:
        Option<Vec<models::MovePositionForSubAccountResponseMovePositionOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountV2ResponseDeliveryAccountRespAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountAssetsAssetManagementResponseBalancesInner {
    pub Option: :is_none")]
    pub freeze: Option<String>,
    pub Option: :is_none")]
    pub withdrawing: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataPositionInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
}

#[derive(SimpleObject)]
pub struct WithdrawlAssetsFromTheManagedSubAccountResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountResponse {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub assets: Option<
        Vec<models::GetDetailOnSubAccountsFuturesAccountV2ResponseFutureAccountRespAssetsInner>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryResp {
    pub Option: :is_none")]
    pub total_margin_balance_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_unrealized_profit_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance_of_btc: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub sub_account_list: Option<Vec<models::GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryRespSubAccountListInner>>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<i64>,
    pub Option: :is_none")]
    pub wallet_balance: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountAssetsAssetManagementResponse {
    pub Option: :is_none")]
    pub balances: Option<Vec<models::QuerySubAccountAssetsAssetManagementResponseBalancesInner>>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountSpotAssetsSummaryResponseSpotSubUserAssetBtcVoListInner {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub total_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountV2Response {
    pub Option: :is_none"
    )]
    pub future_account_summary_resp: Option<
        Box<models::GetSummaryOfSubAccountsFuturesAccountV2ResponseFutureAccountSummaryResp>,
    pub Option: :is_none"
    )]
    pub delivery_account_summary_resp: Option<
        Box<models::GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryResp>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerData {
    pub Option: :is_none")]
    pub assets: Option<Vec<models::QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataAssetsInner>>,
    pub Option: :is_none")]
    pub position: Option<Vec<models::QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataPositionInner>>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
}

#[derive(SimpleObject)]
pub struct CreateAVirtualSubAccountResponse {
    pub Option: :is_none")]
    pub email: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountAssetDetailsResponseInner {
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub name: Option<String>,
    pub Option: :is_none")]
    pub total_balance: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub in_order: Option<String>,
    pub Option: :is_none")]
    pub btc_value: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetManagedSubAccountDepositAddressResponse {
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub tag: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountListResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none"
    )]
    pub manager_sub_user_info_vo_list:
        Option<Vec<models::QueryManagedSubAccountListResponseManagerSubUserInfoVoListInner>>,
}

#[derive(SimpleObject)]
pub struct DeleteIpListForASubAccountApiKeyResponse {
    pub Option: :is_none")]
    pub ip_restrict: Option<String>,
    pub Option: :is_none")]
    pub ip_list: Option<Vec<String>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub api_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct EnableOptionsForSubAccountResponse {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub is_e_options_enabled: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountV2ResponseFutureAccountSummaryResp {
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none")]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub sub_account_list: Option<Vec<models::GetSummaryOfSubAccountsFuturesAccountV2ResponseFutureAccountSummaryRespSubAccountListInner>>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountAssetsResponse {
    pub Option: :is_none")]
    pub balances: Option<Vec<models::QuerySubAccountAssetsResponseBalancesInner>>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountV2ResponseFutureAccountRespAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotResponseSnapshotVosInnerData {
    pub Option: :is_none")]
    pub balances: Option<
        Vec<models::QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataBalancesInner>,
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub margin_level: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub user_assets: Option<
        Vec<models::QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataUserAssetsInner>,
    pub Option: :is_none")]
    pub assets:
        Option<Vec<models::QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataAssetsInner>>,
    pub Option: :is_none")]
    pub position: Option<
        Vec<models::QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataPositionInner>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountV2Response {
    pub Option: :is_none")]
    pub future_account_resp:
        Option<Box<models::GetDetailOnSubAccountsFuturesAccountV2ResponseFutureAccountResp>>,
    pub Option: :is_none"
    )]
    pub delivery_account_resp:
        Option<Box<models::GetDetailOnSubAccountsFuturesAccountV2ResponseDeliveryAccountResp>>,
}

#[derive(SimpleObject)]
pub struct GetSubAccountDepositAddressResponse {
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub tag: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountMarginAssetDetailsResponse {
    pub Option: :is_none")]
    pub margin_level: Option<String>,
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub user_assets:
        Option<Vec<models::QueryManagedSubAccountMarginAssetDetailsResponseUserAssetsInner>>,
}

#[derive(SimpleObject)]
pub struct GetMovePositionHistoryForSubAccountResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none"
    )]
    pub future_move_position_order_vo_list: Option<
        Vec<models::GetMovePositionHistoryForSubAccountResponseFutureMovePositionOrderVoListInner>,
}

#[derive(SimpleObject)]
pub struct AddIpRestrictionForSubAccountApiKeyResponse {
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub ip_list: Option<Vec<String>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub api_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub snapshot_vos: Option<Vec<models::QueryManagedSubAccountSnapshotResponseSnapshotVosInner>>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountAssetsResponseBalancesInner {
    pub Option: :is_none")]
    pub freeze: Option<i64>,
    pub Option: :is_none")]
    pub withdrawing: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub locked: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DepositAssetsIntoTheManagedSubAccountResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TransferToMasterResponse {
    pub Option: :is_none")]
    pub txn_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountV2ResponseFutureAccountResp {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub assets: Option<
        Vec<models::GetDetailOnSubAccountsFuturesAccountV2ResponseFutureAccountRespAssetsInner>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountV2ResponseDeliveryAccountSummaryRespSubAccountListInner {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountFuturesAssetTransferHistoryResponseTransfersInner {
    pub Option: :is_none")]
    pub from: Option<String>,
    pub Option: :is_none")]
    pub to: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesTransferForSubAccountResponse {
    pub Option: :is_none")]
    pub txn_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarginTransferForSubAccountResponse {
    pub Option: :is_none")]
    pub txn_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotResponseSnapshotVosInner {
    pub Option: :is_none")]
    pub data: Option<Box<models::QueryManagedSubAccountSnapshotResponseSnapshotVosInnerData>>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsMarginAccountResponse {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub margin_level: Option<String>,
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub margin_trade_coeff_vo:
        Option<Box<models::GetDetailOnSubAccountsMarginAccountResponseMarginTradeCoeffVo>>,
    pub Option: :is_none"
    )]
    pub margin_user_asset_vo_list:
        Option<Vec<models::GetDetailOnSubAccountsMarginAccountResponseMarginUserAssetVoListInner>>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsMarginAccountResponseMarginTradeCoeffVo {
    pub Option: :is_none"
    )]
    pub force_liquidation_bar: Option<String>,
    pub Option: :is_none")]
    pub margin_call_bar: Option<String>,
    pub Option: :is_none")]
    pub normal_bar: Option<String>,
}

#[derive(SimpleObject)]
pub struct EnableFuturesForSubAccountResponse {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub is_futures_enabled: Option<bool>,
}

#[derive(SimpleObject)]
pub struct UniversalTransferResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub client_tran_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataBalancesInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSubAccountsStatusOnMarginOrFuturesResponseInner {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub is_sub_user_enabled: Option<bool>,
    pub Option: :is_none")]
    pub is_user_active: Option<bool>,
    pub Option: :is_none")]
    pub insert_time: Option<i64>,
    pub Option: :is_none")]
    pub is_margin_enabled: Option<bool>,
    pub Option: :is_none")]
    pub is_future_enabled: Option<bool>,
    pub Option: :is_none")]
    pub mobile: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountTransactionStatisticsResponse {
    pub Option: :is_none")]
    pub recent30_btc_total: Option<String>,
    pub Option: :is_none"
    )]
    pub recent30_btc_futures_total: Option<String>,
    pub Option: :is_none"
    )]
    pub recent30_btc_margin_total: Option<String>,
    pub Option: :is_none")]
    pub recent30_busd_total: Option<String>,
    pub Option: :is_none"
    )]
    pub recent30_busd_futures_total: Option<String>,
    pub Option: :is_none"
    )]
    pub recent30_busd_margin_total: Option<String>,
    pub Option: :is_none")]
    pub trade_info_vos:
        Option<Vec<models::QuerySubAccountTransactionStatisticsResponseTradeInfoVosInner>>,
}

#[derive(SimpleObject)]
pub struct GetSubAccountDepositHistoryResponseInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub address_tag: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub insert_time: Option<i64>,
    pub Option: :is_none")]
    pub transfer_type: Option<i64>,
    pub Option: :is_none")]
    pub confirm_times: Option<String>,
    pub Option: :is_none")]
    pub unlock_confirm: Option<i64>,
    pub Option: :is_none")]
    pub wallet_type: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountTransferLogMasterAccountInvestorResponse {
    pub Option: :is_none")]
    pub manager_sub_transfer_history_vos: Option<Vec<models::QueryManagedSubAccountTransferLogMasterAccountInvestorResponseManagerSubTransferHistoryVosInner>>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountListResponseSubAccountsInner {
    pub Option: :is_none")]
    pub sub_user_id: Option<i64>,
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub remark: Option<String>,
    pub Option: :is_none")]
    pub is_freeze: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub is_managed_sub_account: Option<bool>,
    pub Option: :is_none"
    )]
    pub is_asset_management_sub_account: Option<bool>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountSpotAssetsSummaryResponse {
    pub Option: :is_none")]
    pub total_count: Option<i64>,
    pub Option: :is_none"
    )]
    pub master_account_total_asset: Option<String>,
    pub Option: :is_none"
    )]
    pub spot_sub_user_asset_btc_vo_list:
        Option<Vec<models::QuerySubAccountSpotAssetsSummaryResponseSpotSubUserAssetBtcVoListInner>>,
}

#[derive(SimpleObject)]
pub struct SubAccountTransferHistoryResponseInner {
    pub Option: :is_none")]
    pub counter_party: Option<String>,
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub from_account_type: Option<String>,
    pub Option: :is_none")]
    pub to_account_type: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesPositionRiskOfSubAccountV2ResponseFuturePositionRiskVosInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub max_notional: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub position_amount: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUniversalTransferHistoryResponse {
    pub Option: :is_none")]
    pub result: Option<Vec<models::QueryUniversalTransferHistoryResponseResultInner>>,
    pub Option: :is_none")]
    pub total_count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountTransferLogSubAccountTradingResponse {
    pub Option: :is_none")]
    pub manager_sub_transfer_history_vos: Option<Vec<models::QueryManagedSubAccountTransferLogMasterAccountInvestorResponseManagerSubTransferHistoryVosInner>>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountTransactionStatisticsResponseTradeInfoVosInner {
    pub Option: :is_none")]
    pub user_id: Option<i64>,
    pub Option: :is_none")]
    pub btc: Option<i64>,
    pub Option: :is_none")]
    pub btc_futures: Option<i64>,
    pub Option: :is_none")]
    pub btc_margin: Option<i64>,
    pub Option: :is_none")]
    pub busd: Option<i64>,
    pub Option: :is_none")]
    pub busd_futures: Option<i64>,
    pub Option: :is_none")]
    pub busd_margin: Option<i64>,
    pub Option: :is_none")]
    pub date: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotResponseSnapshotVosInnerDataUserAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub borrowed: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub net_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountListResponse {
    pub Option: :is_none")]
    pub sub_accounts: Option<Vec<models::QuerySubAccountListResponseSubAccountsInner>>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountTransferLogMasterAccountTradingResponse {
    pub Option: :is_none")]
    pub manager_sub_transfer_history_vos: Option<Vec<models::QueryManagedSubAccountTransferLogMasterAccountInvestorResponseManagerSubTransferHistoryVosInner>>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountV2ResponseDeliveryAccountResp {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub assets: Option<
        Vec<models::GetDetailOnSubAccountsFuturesAccountV2ResponseDeliveryAccountRespAssetsInner>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountFuturesAssetTransferHistoryResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub futures_type: Option<i64>,
    pub Option: :is_none")]
    pub transfers:
        Option<Vec<models::QuerySubAccountFuturesAssetTransferHistoryResponseTransfersInner>>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerDataPositionInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<i64>,
    pub Option: :is_none")]
    pub mark_price: Option<i64>,
    pub Option: :is_none")]
    pub position_amt: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct TransferToSubAccountOfSameMasterResponse {
    pub Option: :is_none")]
    pub txn_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFuturesPositionRiskOfSubAccountV2ResponseDeliveryPositionRiskVosInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub isolated: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub is_auto_add_margin: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub position_amount: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountFuturesAssetDetailsResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub snapshot_vos:
        Option<Vec<models::QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInner>>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsMarginAccountResponseSubAccountListInner {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFuturesPositionRiskOfSubAccountV2Response {
    pub Option: :is_none"
    )]
    pub future_position_risk_vos:
        Option<Vec<models::GetFuturesPositionRiskOfSubAccountV2ResponseFuturePositionRiskVosInner>>,
    pub Option: :is_none"
    )]
    pub delivery_position_risk_vos: Option<
        Vec<models::GetFuturesPositionRiskOfSubAccountV2ResponseDeliveryPositionRiskVosInner>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountResponse {
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none")]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub sub_account_list: Option<Vec<models::GetSummaryOfSubAccountsFuturesAccountV2ResponseFutureAccountSummaryRespSubAccountListInner>>,
}

#[derive(SimpleObject)]
pub struct MovePositionForSubAccountResponseMovePositionOrdersInner {
    pub Option: :is_none")]
    pub from_user_email: Option<String>,
    pub Option: :is_none")]
    pub to_user_email: Option<String>,
    pub Option: :is_none")]
    pub product_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountMarginAssetDetailsResponseUserAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub borrowed: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub net_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsMarginAccountResponse {
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub sub_account_list:
        Option<Vec<models::GetSummaryOfSubAccountsMarginAccountResponseSubAccountListInner>>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountTransferLogMasterAccountInvestorResponseManagerSubTransferHistoryVosInner {
    pub Option: :is_none")]
    pub from_email: Option<String>,
    pub Option: :is_none")]
    pub from_account_type: Option<String>,
    pub Option: :is_none")]
    pub to_email: Option<String>,
    pub Option: :is_none")]
    pub to_account_type: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub scheduled_data: Option<i64>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInner {
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub data:
        Option<Box<models::QueryManagedSubAccountFuturesAssetDetailsResponseSnapshotVosInnerData>>,
}

#[derive(SimpleObject)]
pub struct MovePositionForSubAccountOrderArgsParameterInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubAccountFuturesAssetTransferResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub txn_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountV2ResponseFutureAccountSummaryRespSubAccountListInner {
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub total_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_maintenance_margin: Option<String>,
    pub Option: :is_none")]
    pub total_margin_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub total_open_order_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetMovePositionHistoryForSubAccountResponseFutureMovePositionOrderVoListInner {
    pub Option: :is_none")]
    pub from_user_email: Option<String>,
    pub Option: :is_none")]
    pub to_user_email: Option<String>,
    pub Option: :is_none")]
    pub product_type: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub quantity: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub time_stamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountSpotAssetTransferHistoryResponseInner {
    pub Option: :is_none")]
    pub from: Option<String>,
    pub Option: :is_none")]
    pub to: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountListResponseManagerSubUserInfoVoListInner {
    pub Option: :is_none")]
    pub root_user_id: Option<i64>,
    pub Option: :is_none")]
    pub managersub_user_id: Option<i64>,
    pub Option: :is_none")]
    pub bind_parent_user_id: Option<i64>,
    pub Option: :is_none")]
    pub email: Option<String>,
    pub Option: :is_none")]
    pub insert_time_stamp: Option<i64>,
    pub Option: :is_none")]
    pub bind_parent_email: Option<String>,
    pub Option: :is_none")]
    pub is_sub_user_enabled: Option<bool>,
    pub Option: :is_none")]
    pub is_user_active: Option<bool>,
    pub Option: :is_none")]
    pub is_margin_enabled: Option<bool>,
    pub Option: :is_none")]
    pub is_future_enabled: Option<bool>,
    pub Option: :is_none"
    )]
    pub is_signed_lvt_risk_agreement: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsMarginAccountResponseMarginUserAssetVoListInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub borrowed: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub net_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountManagementApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CreateAVirtualSubAccountParams {
    pub sub_account_string: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EnableFuturesForSubAccountParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EnableOptionsForSubAccountParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesPositionRiskOfSubAccountParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesPositionRiskOfSubAccountV2Params {
    pub email: String,
    pub 1: USDT-margined Futures，2: Coin-margined Futures
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub futures_type: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSubAccountsStatusOnMarginOrFuturesParams {
    pub email: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountListParams {
    pub email: Option<String>,
    pub is_freeze: Option<String>,
    pub value: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountTransactionStatisticsParams {
    pub email: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountManagementApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct ApiManagementApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AddIpRestrictionForSubAccountApiKeyParams {
    pub email: String,
    pub sub_account_api_key: String,
    pub status: String,
    pub ip_address: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DeleteIpListForASubAccountApiKeyParams {
    pub email: String,
    pub sub_account_api_key: String,
    pub ip_address: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetIpRestrictionForASubAccountApiKeyParams {
    pub email: String,
    pub sub_account_api_key: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockApiManagementApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct ManagedSubAccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct DepositAssetsIntoTheManagedSubAccountParams {
    pub to_email: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetManagedSubAccountDepositAddressParams {
    pub email: String,
    pub coin: String,
    pub network: Option<String>,
    pub amount: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountAssetDetailsParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountFuturesAssetDetailsParams {
    pub email: String,
    pub account_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountListParams {
    pub email: Option<String>,
    pub value: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountMarginAssetDetailsParams {
    pub email: String,
    pub account_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountSnapshotParams {
    pub email: String,
    pub type: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountTransferLogMasterAccountInvestorParams {
    pub email: String,
    pub start_time: i64,
    pub end_time: i64,
    pub page: i64,
    pub Max: 500)
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub limit: i64,
    pub transfers: Option<String>,
    pub transfer_function_account_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountTransferLogMasterAccountTradingParams {
    pub email: String,
    pub start_time: i64,
    pub end_time: i64,
    pub page: i64,
    pub Max: 500)
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub limit: i64,
    pub transfers: Option<String>,
    pub transfer_function_account_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryManagedSubAccountTransferLogSubAccountTradingParams {
    pub start_time: i64,
    pub end_time: i64,
    pub page: i64,
    pub Max: 500)
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub limit: i64,
    pub transfers: Option<String>,
    pub transfer_function_account_type: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WithdrawlAssetsFromTheManagedSubAccountParams {
    pub from_email: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub transfer_date: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockManagedSubAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct AssetManagementApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct FuturesTransferForSubAccountParams {
    pub email: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub 1: transfer from subaccount's  spot account to margin account 2: transfer from subaccount's margin account to its spot account
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub r#type: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsFuturesAccountV2Params {
    pub email: String,
    pub 1: USDT-margined Futures，2: Coin-margined Futures
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub futures_type: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDetailOnSubAccountsMarginAccountParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetMovePositionHistoryForSubAccountParams {
    pub symbol: String,
    pub page: i64,
    pub row: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSubAccountDepositAddressParams {
    pub email: String,
    pub coin: String,
    pub network: Option<String>,
    pub amount: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSubAccountDepositHistoryParams {
    pub email: String,
    pub coin: Option<String>,
    pub 0: pending,
    pub 6: credited but cannot withdraw,
    pub 7: Wrong Deposit,
    pub 8: Waiting User confirm,
    pub 1: success)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub status: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub default: 0
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub offset: Option<i64>,
    pub recv_window: Option<i64>,
    pub tx_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsFuturesAccountV2Params {
    pub 1: USDT-margined Futures，2: Coin-margined Futures
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub futures_type: i64,
    pub value: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSummaryOfSubAccountsMarginAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginTransferForSubAccountParams {
    pub email: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub 1: transfer from subaccount's  spot account to margin account 2: transfer from subaccount's margin account to its spot account
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub r#type: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MovePositionForSubAccountParams {
    pub from_user_email: String,
    pub to_user_email: String,
    pub product_type: String,
    pub array: orderArgs[0].symbol=BTCUSDT orderArgs[0].quantity=0.001 orderArgs[0].positionSide=BOTH orderArgs[1].symbol=ETHUSDT orderArgs[1].quantity=0.01 orderArgs[1].positionSide=BOTH
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub order_args: Vec<models::MovePositionForSubAccountOrderArgsParameterInner>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountAssetsParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountAssetsAssetManagementParams {
    pub email: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountFuturesAssetTransferHistoryParams {
    pub email: String,
    pub 1: USDT-margined Futures，2: Coin-margined Futures
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub futures_type: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub value: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountSpotAssetTransferHistoryParams {
    pub from_email: Option<String>,
    pub to_email: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub value: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubAccountSpotAssetsSummaryParams {
    pub email: Option<String>,
    pub value: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUniversalTransferHistoryParams {
    pub from_email: Option<String>,
    pub to_email: Option<String>,
    pub client_tran_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub value: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub page: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubAccountFuturesAssetTransferParams {
    pub from_email: String,
    pub to_email: String,
    pub 1: USDT-margined Futures，2: Coin-margined Futures
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub futures_type: i64,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubAccountTransferHistoryParams {
    pub asset: Option<String>,
    pub 1: transfer in,
    pub 2: transfer out
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub type: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub value: 1,
    pub value: 200
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub return_fail_history: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TransferToMasterParams {
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TransferToSubAccountOfSameMasterParams {
    pub to_email: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UniversalTransferParams {
    pub from_account_type: String,
    pub to_account_type: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub from_email: Option<String>,
    pub to_email: Option<String>,
    pub client_tran_id: Option<String>,
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAssetManagementApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub market_data_api_client: MarketDataApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_information_api_client: UserInformationApiClient,
}

#[derive(SimpleObject)]
pub struct GetLoanableAssetsDataResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLoanableAssetsDataResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBorrowInterestRateResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none"
    )]
    pub flexible_daily_interest_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub flexible_yearly_interest_rate: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CheckVipLoanCollateralAccountResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::CheckVipLoanCollateralAccountResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanableAssetsDataResponseRowsInner {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none"
    )]
    pub _flexible_daily_interest_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub _flexible_yearly_interest_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub _30d_daily_interest_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub _30d_yearly_interest_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub _60d_daily_interest_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub _60d_yearly_interest_rate: Option<String>,
    pub Option: :is_none")]
    pub min_limit: Option<String>,
    pub Option: :is_none")]
    pub max_limit: Option<String>,
    pub Option: :is_none")]
    pub vip_level: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryApplicationStatusResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryApplicationStatusResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetVipLoanOngoingOrdersResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetVipLoanOngoingOrdersResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct VipLoanRepayResponse {
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub repay_amount: Option<String>,
    pub Option: :is_none")]
    pub remaining_principal: Option<String>,
    pub Option: :is_none")]
    pub remaining_interest: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub current_ltv: Option<String>,
    pub Option: :is_none")]
    pub repay_status: Option<String>,
}

#[derive(SimpleObject)]
pub struct VipLoanBorrowResponse {
    pub Option: :is_none")]
    pub loan_account_id: Option<String>,
    pub Option: :is_none")]
    pub request_id: Option<String>,
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub is_flexible_rate: Option<String>,
    pub Option: :is_none")]
    pub loan_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub collateral_account_id: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub loan_term: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetCollateralAssetDataResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetCollateralAssetDataResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CheckVipLoanCollateralAccountResponseRowsInner {
    pub Option: :is_none"
    )]
    pub collateral_account_id: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryApplicationStatusResponseRowsInner {
    pub Option: :is_none")]
    pub loan_account_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<String>,
    pub Option: :is_none")]
    pub request_id: Option<String>,
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub loan_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub collateral_account_id: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub loan_term: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub loan_date: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetVipLoanOngoingOrdersResponseRowsInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub total_debt: Option<String>,
    pub Option: :is_none")]
    pub residual_interest: Option<String>,
    pub Option: :is_none"
    )]
    pub collateral_account_id: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_collateral_value_after_haircut: Option<String>,
    pub Option: :is_none"
    )]
    pub locked_collateral_value: Option<String>,
    pub Option: :is_none")]
    pub current_ltv: Option<String>,
    pub Option: :is_none")]
    pub expiration_time: Option<i64>,
    pub Option: :is_none")]
    pub loan_date: Option<String>,
    pub Option: :is_none")]
    pub loan_term: Option<String>,
}

#[derive(SimpleObject)]
pub struct VipLoanRenewResponse {
    pub Option: :is_none")]
    pub loan_account_id: Option<String>,
    pub Option: :is_none")]
    pub loan_coin: Option<String>,
    pub Option: :is_none")]
    pub loan_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub collateral_account_id: Option<String>,
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none")]
    pub loan_term: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetCollateralAssetDataResponseRowsInner {
    pub Option: :is_none")]
    pub collateral_coin: Option<String>,
    pub Option: :is_none"
    )]
    pub _1st_collateral_ratio: Option<String>,
    pub Option: :is_none"
    )]
    pub _1st_collateral_range: Option<String>,
    pub Option: :is_none"
    )]
    pub _2nd_collateral_ratio: Option<String>,
    pub Option: :is_none"
    )]
    pub _2nd_collateral_range: Option<String>,
    pub Option: :is_none"
    )]
    pub _3rd_collateral_ratio: Option<String>,
    pub Option: :is_none"
    )]
    pub _3rd_collateral_range: Option<String>,
    pub Option: :is_none"
    )]
    pub _4th_collateral_ratio: Option<String>,
    pub Option: :is_none"
    )]
    pub _4th_collateral_range: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct VipLoanBorrowParams {
    pub loan_account_id: i64,
    pub loan_coin: String,
    pub loan_amount: rust_decimal::Decimal,
    pub collateral_account_id: String,
    pub collateral_coin: String,
    pub Default: TRUE. TRUE : flexible rate; FALSE: fixed rate
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub is_flexible_rate: bool,
    pub Eg: 30/60 days
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub loan_term: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct VipLoanRenewParams {
    pub order_id: i64,
    pub loan_term: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct VipLoanRepayParams {
    pub order_id: i64,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetBorrowInterestRateParams {
    pub loan_coin: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCollateralAssetDataParams {
    pub collateral_coin: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLoanableAssetsDataParams {
    pub loan_coin: Option<String>,
    pub default: user's vip level
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub vip_level: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct UserInformationApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CheckVipLoanCollateralAccountParams {
    pub order_id: Option<i64>,
    pub collateral_account_id: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetVipLoanOngoingOrdersParams {
    pub order_id: Option<i64>,
    pub collateral_account_id: Option<i64>,
    pub loan_coin: Option<String>,
    pub collateral_coin: Option<String>,
    pub Default: 1,
    pub Max: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryApplicationStatusParams {
    pub Default: 1,
    pub Max: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockUserInformationApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub market_data_api_client: MarketDataApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub websocket_streams_base: Arc<WebsocketStreamsBase>,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsHandle {
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct Risklevelchange {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub eq: Option<String>,
    pub Option: :is_none")]
    pub ae: Option<String>,
    pub Option: :is_none")]
    pub m: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangeAutoRepayFuturesStatusResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub rows:
        Option<Vec<models::QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponseRowsInner>>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginProAccountBalanceResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub total_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_asset: Option<String>,
    pub Option: :is_none"
    )]
    pub cross_margin_borrowed: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_free: Option<String>,
    pub Option: :is_none"
    )]
    pub cross_margin_interest: Option<String>,
    pub Option: :is_none")]
    pub cross_margin_locked: Option<String>,
    pub Option: :is_none")]
    pub um_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub um_unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub cm_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cm_unrealized_pnl: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub negative_balance: Option<String>,
    pub Option: :is_none"
    )]
    pub option_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub option_equity: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub repay_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginCollateralRateResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub collateral_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct RepayFuturesNegativeBalanceResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginProSpanAccountInfoResponse {
    pub Option: :is_none")]
    pub uni_mmr: Option<String>,
    pub Option: :is_none")]
    pub account_equity: Option<String>,
    pub Option: :is_none")]
    pub actual_equity: Option<String>,
    pub Option: :is_none")]
    pub account_maint_margin: Option<String>,
    pub Option: :is_none")]
    pub risk_unit_mm_list:
        Option<Vec<models::GetPortfolioMarginProSpanAccountInfoResponseRiskUnitMmListInner>>,
    pub Option: :is_none")]
    pub margin_mm: Option<String>,
    pub Option: :is_none")]
    pub other_mm: Option<String>,
    pub Option: :is_none")]
    pub account_status: Option<String>,
    pub Option: :is_none")]
    pub account_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginProTieredCollateralRateResponseInnerCollateralInfoInner {
    pub Option: :is_none")]
    pub tier_floor: Option<String>,
    pub Option: :is_none")]
    pub tier_cap: Option<String>,
    pub Option: :is_none")]
    pub collateral_rate: Option<String>,
    pub Option: :is_none")]
    pub cum: Option<String>,
}

#[derive(SimpleObject)]
pub struct FundCollectionByAssetResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginProBankruptcyLoanAmountResponse {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginProNegativeBalanceInterestHistoryResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none"
    )]
    pub interest_accrued_time: Option<i64>,
    pub Option: :is_none")]
    pub interest_rate: Option<String>,
    pub Option: :is_none")]
    pub principal: Option<String>,
}

#[derive(SimpleObject)]
pub struct FundAutoCollectionResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginProSpanAccountInfoResponseRiskUnitMmListInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub uni_maintain_usd: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginProAccountInfoResponse {
    pub Option: :is_none")]
    pub uni_mmr: Option<String>,
    pub Option: :is_none")]
    pub account_equity: Option<String>,
    pub Option: :is_none")]
    pub actual_equity: Option<String>,
    pub Option: :is_none")]
    pub account_maint_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub account_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub total_available_balance: Option<String>,
    pub Option: :is_none")]
    pub account_status: Option<String>,
    pub Option: :is_none")]
    pub account_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct TransferLdusdtRwusdForPortfolioMarginResponse {
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetTransferableEarnAssetBalanceForPortfolioMarginResponse {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginProTieredCollateralRateResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub collateral_info:
        Option<Vec<models::PortfolioMarginProTieredCollateralRateResponseInnerCollateralInfoInner>>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginProBankruptcyLoanRepayResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginAssetLeverageResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetAutoRepayFuturesStatusResponse {
    pub Option: :is_none")]
    pub auto_repay: Option<bool>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginAssetIndexPriceResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub asset_index_price: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct BnbTransferResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct BnbTransferParams {
    pub amount: rust_decimal::Decimal,
    pub transfer_side: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeAutoRepayFuturesStatusParams {
    pub Default: `true`; `false` for turn off the auto-repay futures negative balance function
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub auto_repay: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FundAutoCollectionParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FundCollectionByAssetParams {
    pub asset: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetAutoRepayFuturesStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginProAccountBalanceParams {
    pub asset: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginProAccountInfoParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetPortfolioMarginProSpanAccountInfoParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetTransferableEarnAssetBalanceForPortfolioMarginParams {
    pub asset: String,
    pub transfer_type: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginProBankruptcyLoanRepayParams {
    pub from: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginProBankruptcyLoanAmountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginProBankruptcyLoanRepayHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginProNegativeBalanceInterestHistoryParams {
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 10 Max:100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RepayFuturesNegativeBalanceParams {
    pub from: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TransferLdusdtRwusdForPortfolioMarginParams {
    pub asset: String,
    pub transfer_type: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginProTieredCollateralRateParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryPortfolioMarginAssetIndexPriceParams {
    pub asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub bfusd_api_client: BfusdApiClient,
    pub flexible_locked_api_client: FlexibleLockedApiClient,
    pub rwusd_api_client: RwusdApiClient,
}

#[derive(SimpleObject)]
pub struct GetRateHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub product_id: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdAccountResponse {
    pub Option: :is_none")]
    pub rwusd_amount: Option<String>,
    pub Option: :is_none")]
    pub total_profit: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemLockedProductResponse {
    pub Option: :is_none")]
    pub redeem_id: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetRwusdQuotaDetailsResponseStandardRedemptionQuota {
    pub Option: :is_none")]
    pub left_quota: Option<String>,
    pub Option: :is_none")]
    pub minimum: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub redeem_period: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRateHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBfusdRateHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLockedRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub lock_period: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRedemptionHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub receive_asset: Option<String>,
    pub Option: :is_none")]
    pub receive_amount: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLockedSubscriptionRecordResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<i64>,
    pub Option: :is_none")]
    pub purchase_id: Option<String>,
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub lock_period: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub source_account: Option<String>,
    pub Option: :is_none")]
    pub amt_from_spot: Option<String>,
    pub Option: :is_none")]
    pub amt_from_funding: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetRwusdSubscriptionHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetRwusdSubscriptionHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCollateralRecordResponseRowsInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub product_id: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub product_name: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdSubscriptionHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBfusdSubscriptionHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRedemptionHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBfusdRedemptionHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdQuotaDetailsResponse {
    pub Option: :is_none")]
    pub subscription_quota: Option<Box<models::GetRwusdQuotaDetailsResponseSubscriptionQuota>>,
    pub Option: :is_none"
    )]
    pub fast_redemption_quota: Option<Box<models::GetRwusdQuotaDetailsResponseFastRedemptionQuota>>,
    pub Option: :is_none"
    )]
    pub standard_redemption_quota:
        Option<Box<models::GetRwusdQuotaDetailsResponseStandardRedemptionQuota>>,
    pub Option: :is_none")]
    pub subscribe_enable: Option<bool>,
    pub Option: :is_none")]
    pub redeem_enable: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetFlexiblePersonalLeftQuotaResponse {
    pub Option: :is_none")]
    pub left_personal_quota: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLockedRewardsHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLockedRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnFlexibleProductListResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetSimpleEarnFlexibleProductListResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedProductPositionResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLockedProductPositionResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedPersonalLeftQuotaResponse {
    pub Option: :is_none")]
    pub left_personal_quota: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemBfusdResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub receive_amount: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdAccountResponse {
    pub Option: :is_none")]
    pub bfusd_amount: Option<String>,
    pub Option: :is_none")]
    pub total_profit: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemFlexibleProductResponse {
    pub Option: :is_none")]
    pub redeem_id: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetBfusdQuotaDetailsResponseStandardRedemptionQuota {
    pub Option: :is_none")]
    pub left_quota: Option<String>,
    pub Option: :is_none")]
    pub minimum: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub redeem_period: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleProductPositionResponseRowsInnerTierAnnualPercentageRate {
    pub Option: :is_none")]
    pub param_0_5_btc: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub param_5_10_btc: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct SetLockedProductRedeemOptionResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRewardsHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetRwusdRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleRedemptionRecordResponseRowsInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub redeem_id: Option<i64>,
    pub Option: :is_none")]
    pub dest_account: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleProductPositionResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleProductPositionResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnFlexibleProductListResponseRowsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none"
    )]
    pub latest_annual_percentage_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub tier_annual_percentage_rate:
        Option<Box<models::GetFlexibleProductPositionResponseRowsInnerTierAnnualPercentageRate>>,
    pub Option: :is_none"
    )]
    pub air_drop_percentage_rate: Option<String>,
    pub Option: :is_none")]
    pub can_purchase: Option<bool>,
    pub Option: :is_none")]
    pub can_redeem: Option<bool>,
    pub Option: :is_none")]
    pub is_sold_out: Option<bool>,
    pub Option: :is_none")]
    pub hot: Option<bool>,
    pub Option: :is_none")]
    pub min_purchase_amount: Option<String>,
    pub Option: :is_none")]
    pub product_id: Option<String>,
    pub Option: :is_none"
    )]
    pub subscription_start_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnLockedProductListResponseRowsInnerQuota {
    pub Option: :is_none")]
    pub total_personal_quota: Option<String>,
    pub Option: :is_none")]
    pub minimum: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLockedRedemptionRecordResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<i64>,
    pub Option: :is_none")]
    pub redeem_id: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub lock_period: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub original_amount: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub deliver_date: Option<String>,
    pub Option: :is_none")]
    pub loss_amount: Option<String>,
    pub Option: :is_none")]
    pub is_complete: Option<bool>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub reward_amt: Option<String>,
    pub Option: :is_none")]
    pub extra_reward_asset: Option<String>,
    pub Option: :is_none")]
    pub est_extra_reward_amt: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleSubscriptionRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleSubscriptionRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnLockedProductListResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetSimpleEarnLockedProductListResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCollateralRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetCollateralRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnLockedProductListResponseRowsInnerDetail {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub duration: Option<i64>,
    pub Option: :is_none")]
    pub renewable: Option<bool>,
    pub Option: :is_none")]
    pub is_sold_out: Option<bool>,
    pub Option: :is_none")]
    pub apr: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none"
    )]
    pub subscription_start_time: Option<i64>,
    pub Option: :is_none")]
    pub extra_reward_asset: Option<String>,
    pub Option: :is_none")]
    pub extra_reward_apr: Option<String>,
    pub Option: :is_none")]
    pub boost_reward_asset: Option<String>,
    pub Option: :is_none")]
    pub boost_apr: Option<String>,
    pub Option: :is_none")]
    pub boost_end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRateHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBfusdRateHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubscribeRwusdResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub rwusd_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleRewardsHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdQuotaDetailsResponseFastRedemptionQuota {
    pub Option: :is_none")]
    pub left_quota: Option<String>,
    pub Option: :is_none")]
    pub minimum: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub free_quota: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemRwusdResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub receive_amount: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetFlexibleAutoSubscribeResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleRedemptionRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetFlexibleRedemptionRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub rewards: Option<String>,
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRateHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetRateHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetRwusdQuotaDetailsResponseSubscriptionQuota {
    pub Option: :is_none")]
    pub assets: Option<Vec<String>>,
    pub Option: :is_none")]
    pub left_quota: Option<String>,
    pub Option: :is_none")]
    pub minimum: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBfusdQuotaDetailsResponse {
    pub Option: :is_none"
    )]
    pub fast_redemption_quota: Option<Box<models::GetBfusdQuotaDetailsResponseFastRedemptionQuota>>,
    pub Option: :is_none"
    )]
    pub standard_redemption_quota:
        Option<Box<models::GetBfusdQuotaDetailsResponseStandardRedemptionQuota>>,
    pub Option: :is_none")]
    pub subscribe_enable: Option<bool>,
    pub Option: :is_none")]
    pub redeem_enable: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRewardsHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBfusdRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub rewards_amount: Option<String>,
    pub Option: :is_none")]
    pub rwusd_position: Option<String>,
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetLockedSubscriptionPreviewResponseInner {
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub total_reward_amt: Option<String>,
    pub Option: :is_none")]
    pub extra_reward_asset: Option<String>,
    pub Option: :is_none"
    )]
    pub est_total_extra_reward_amt: Option<String>,
    pub Option: :is_none")]
    pub boost_reward_asset: Option<String>,
    pub Option: :is_none")]
    pub est_daily_reward_amt: Option<String>,
    pub Option: :is_none")]
    pub next_pay: Option<String>,
    pub Option: :is_none")]
    pub next_pay_date: Option<String>,
    pub Option: :is_none")]
    pub value_date: Option<String>,
    pub Option: :is_none")]
    pub rewards_end_date: Option<String>,
    pub Option: :is_none")]
    pub deliver_date: Option<String>,
    pub Option: :is_none"
    )]
    pub next_subscription_date: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub rewards_amount: Option<String>,
    pub Option: :is_none")]
    pub bfusd_position: Option<String>,
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnLockedProductListResponseRowsInner {
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub detail: Option<Box<models::GetSimpleEarnLockedProductListResponseRowsInnerDetail>>,
    pub Option: :is_none")]
    pub quota: Option<Box<models::GetSimpleEarnLockedProductListResponseRowsInnerQuota>>,
}

#[derive(SimpleObject)]
pub struct GetLockedProductPositionResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<i64>,
    pub Option: :is_none")]
    pub parent_position_id: Option<i64>,
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub purchase_time: Option<String>,
    pub Option: :is_none")]
    pub duration: Option<String>,
    pub Option: :is_none")]
    pub accrual_days: Option<String>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub apy: Option<String>,
    pub Option: :is_none")]
    pub reward_amt: Option<String>,
    pub Option: :is_none")]
    pub extra_reward_asset: Option<String>,
    pub Option: :is_none")]
    pub extra_reward_apr: Option<String>,
    pub Option: :is_none")]
    pub est_extra_reward_amt: Option<String>,
    pub Option: :is_none")]
    pub boost_reward_asset: Option<String>,
    pub Option: :is_none")]
    pub boost_apr: Option<String>,
    pub Option: :is_none"
    )]
    pub total_boost_reward_amt: Option<String>,
    pub Option: :is_none")]
    pub next_pay: Option<String>,
    pub Option: :is_none")]
    pub next_pay_date: Option<String>,
    pub Option: :is_none")]
    pub pay_period: Option<String>,
    pub Option: :is_none")]
    pub redeem_amount_early: Option<String>,
    pub Option: :is_none")]
    pub rewards_end_date: Option<String>,
    pub Option: :is_none")]
    pub deliver_date: Option<String>,
    pub Option: :is_none")]
    pub redeem_period: Option<String>,
    pub Option: :is_none")]
    pub redeeming_amt: Option<String>,
    pub Option: :is_none")]
    pub redeem_to: Option<String>,
    pub Option: :is_none"
    )]
    pub partial_amt_deliver_date: Option<String>,
    pub Option: :is_none")]
    pub can_redeem_early: Option<bool>,
    pub Option: :is_none")]
    pub can_fast_redemption: Option<bool>,
    pub Option: :is_none")]
    pub auto_subscribe: Option<bool>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub can_re_stake: Option<bool>,
}

#[derive(SimpleObject)]
pub struct SubscribeLockedProductResponse {
    pub Option: :is_none")]
    pub purchase_id: Option<i64>,
    pub Option: :is_none")]
    pub position_id: Option<String>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetLockedSubscriptionRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLockedSubscriptionRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRedemptionHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub receive_asset: Option<String>,
    pub Option: :is_none")]
    pub receive_amount: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleSubscriptionRecordResponseRowsInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub purchase_id: Option<i64>,
    pub Option: :is_none")]
    pub product_id: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub source_account: Option<String>,
    pub Option: :is_none")]
    pub amt_from_spot: Option<String>,
    pub Option: :is_none")]
    pub amt_from_funding: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBfusdSubscriptionHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub receive_asset: Option<String>,
    pub Option: :is_none")]
    pub receive_amount: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetRwusdSubscriptionHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub receive_asset: Option<String>,
    pub Option: :is_none")]
    pub receive_amount: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct SetLockedAutoSubscribeResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRedemptionHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetRwusdRedemptionHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdQuotaDetailsResponseFastRedemptionQuota {
    pub Option: :is_none")]
    pub left_quota: Option<String>,
    pub Option: :is_none")]
    pub minimum: Option<String>,
    pub Option: :is_none")]
    pub fee: Option<String>,
    pub Option: :is_none")]
    pub free_quota: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleSubscriptionPreviewResponse {
    pub Option: :is_none")]
    pub total_amount: Option<String>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub air_drop_asset: Option<String>,
    pub Option: :is_none"
    )]
    pub est_daily_bonus_rewards: Option<String>,
    pub Option: :is_none"
    )]
    pub est_daily_real_time_rewards: Option<String>,
    pub Option: :is_none"
    )]
    pub est_daily_airdrop_rewards: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRateHistoryResponseRowsInner {
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeFlexibleProductResponse {
    pub Option: :is_none")]
    pub purchase_id: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct SubscribeBfusdResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub bfusd_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleProductPositionResponseRowsInner {
    pub Option: :is_none")]
    pub total_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub tier_annual_percentage_rate:
        Option<Box<models::GetFlexibleProductPositionResponseRowsInnerTierAnnualPercentageRate>>,
    pub Option: :is_none"
    )]
    pub latest_annual_percentage_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub yesterday_airdrop_percentage_rate: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub air_drop_asset: Option<String>,
    pub Option: :is_none")]
    pub can_redeem: Option<bool>,
    pub Option: :is_none")]
    pub collateral_amount: Option<String>,
    pub Option: :is_none")]
    pub product_id: Option<String>,
    pub Option: :is_none"
    )]
    pub yesterday_real_time_rewards: Option<String>,
    pub Option: :is_none"
    )]
    pub cumulative_bonus_rewards: Option<String>,
    pub Option: :is_none"
    )]
    pub cumulative_real_time_rewards: Option<String>,
    pub Option: :is_none"
    )]
    pub cumulative_total_rewards: Option<String>,
    pub Option: :is_none")]
    pub auto_subscribe: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetLockedRedemptionRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetLockedRedemptionRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SimpleAccountResponse {
    pub Option: :is_none")]
    pub total_amount_in_btc: Option<String>,
    pub Option: :is_none")]
    pub total_amount_in_usdt: Option<String>,
    pub Option: :is_none"
    )]
    pub total_flexible_amount_in_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_flexible_amount_in_usdt: Option<String>,
    pub Option: :is_none")]
    pub total_locked_in_btc: Option<String>,
    pub Option: :is_none")]
    pub total_locked_in_usdt: Option<String>,
}

#[derive(SimpleObject)]
pub struct RwusdApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetRwusdAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdQuotaDetailsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRateHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRedemptionHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdRewardsHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRwusdSubscriptionHistoryParams {
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemRwusdParams {
    pub amount: rust_decimal::Decimal,
    pub type: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeRwusdParams {
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockRwusdApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct FlexibleLockedApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetCollateralRecordParams {
    pub product_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexiblePersonalLeftQuotaParams {
    pub product_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleProductPositionParams {
    pub asset: Option<String>,
    pub product_id: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleRedemptionRecordParams {
    pub product_id: Option<String>,
    pub redeem_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleRewardsHistoryParams {
    pub type: String,
    pub product_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleSubscriptionPreviewParams {
    pub product_id: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFlexibleSubscriptionRecordParams {
    pub product_id: Option<String>,
    pub purchase_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedPersonalLeftQuotaParams {
    pub project_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedProductPositionParams {
    pub asset: Option<String>,
    pub position_id: Option<i64>,
    pub project_id: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedRedemptionRecordParams {
    pub position_id: Option<i64>,
    pub redeem_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedRewardsHistoryParams {
    pub position_id: Option<i64>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedSubscriptionPreviewParams {
    pub project_id: String,
    pub amount: rust_decimal::Decimal,
    pub auto_subscribe: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetLockedSubscriptionRecordParams {
    pub purchase_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetRateHistoryParams {
    pub product_id: String,
    pub apr_period: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnFlexibleProductListParams {
    pub asset: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSimpleEarnLockedProductListParams {
    pub asset: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemFlexibleProductParams {
    pub product_id: String,
    pub redeem_all: Option<bool>,
    pub amount: Option<rust_decimal::Decimal>,
    pub dest_account: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemLockedProductParams {
    pub position_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetFlexibleAutoSubscribeParams {
    pub product_id: String,
    pub auto_subscribe: bool,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetLockedAutoSubscribeParams {
    pub position_id: String,
    pub auto_subscribe: bool,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetLockedProductRedeemOptionParams {
    pub position_id: String,
    pub redeem_to: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SimpleAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeFlexibleProductParams {
    pub product_id: String,
    pub amount: rust_decimal::Decimal,
    pub auto_subscribe: Option<bool>,
    pub source_account: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeLockedProductParams {
    pub project_id: String,
    pub amount: rust_decimal::Decimal,
    pub auto_subscribe: Option<bool>,
    pub source_account: Option<String>,
    pub redeem_to: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockFlexibleLockedApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct BfusdApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetBfusdAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdQuotaDetailsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRateHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRedemptionHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdRewardsHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBfusdSubscriptionHistoryParams {
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemBfusdParams {
    pub amount: rust_decimal::Decimal,
    pub type: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeBfusdParams {
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockBfusdApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub general_api_client: GeneralApiClient,
    pub market_api_client: MarketApiClient,
    pub trade_api_client: TradeApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketApi {
    pub websocket_api_base: Arc<WebsocketApiBase>,
    pub account_api_client: AccountApiClient,
    pub auth_api_client: AuthApiClient,
    pub general_api_client: GeneralApiClient,
    pub market_api_client: MarketApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_data_stream_api_client: UserDataStreamApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketApiHandle {
    pub configuration: ConfigurationWebsocketApi,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub websocket_streams_base: Arc<WebsocketStreamsBase>,
    pub web_socket_streams_api_client: WebSocketStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsHandle {
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct LotSizeFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AvgPriceResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DiffBookDepthResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub a: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrderAmendsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_amends: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumAlgoOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_algo_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TrailingDeltaFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub min_trailing_above_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub max_trailing_above_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub min_trailing_below_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub max_trailing_below_delta: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumOrderListsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_lists: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NotionalFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_min_to_market: Option<bool>,
    pub Option: :is_none")]
    pub max_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_max_to_market: Option<bool>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct AllMarketRollingWindowTickerResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllTickerResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MinNotionalFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_to_market: Option<bool>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct TickerResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RateLimits {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RollingWindowTickerResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MiniTickerResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
}

#[derive(SimpleObject)]
pub struct BookTickerResponse {
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct PercentPriceFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub multiplier_exponent: Option<i32>,
    pub Option: :is_none")]
    pub multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthResponse {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub asks: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct MaxNumAlgoOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_algo_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct KlineResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MaxAssetFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub limit: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMiniTickerResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
}

#[derive(SimpleObject)]
pub struct TPlusSellFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AggTradeResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub m_uppercase: Option<bool>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrderListsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_lists: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumIcebergOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub max_num_iceberg_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PriceFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_price: Option<String>,
    pub Option: :is_none")]
    pub max_price: Option<String>,
    pub Option: :is_none")]
    pub tick_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub m_uppercase: Option<bool>,
}

#[derive(SimpleObject)]
pub struct KlineOffsetResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::KlineResponseK>>,
}

#[derive(SimpleObject)]
pub struct IcebergPartsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MaxNumIcebergOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub max_num_iceberg_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PercentPriceBySideFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub multiplier_exponent: Option<i32>,
    pub Option: :is_none")]
    pub bid_multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub bid_multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub ask_multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub ask_multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct MarketLotSizeFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct MaxPositionFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub max_position: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::KlineResponseK>>,
}

#[derive(SimpleObject)]
pub struct WebSocketStreamsApiClient {
    pub websocket_streams_base: Arc<WebsocketStreams>,
}

#[derive(SimpleObject)]
pub struct AggTradeParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketRollingWindowTickerParams {
    pub window_size: AllMarketRollingWindowTickerWindowSizeEnum,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMiniTickerParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllTickerParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AvgPriceParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct BookTickerParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct DiffBookDepthParams {
    pub symbol: String,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineParams {
    pub symbol: String,
    pub interval: KlineIntervalEnum,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineOffsetParams {
    pub symbol: String,
    pub interval: KlineOffsetIntervalEnum,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MiniTickerParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthParams {
    pub symbol: String,
    pub levels: PartialBookDepthLevelsEnum,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct RollingWindowTickerParams {
    pub symbol: String,
    pub window_size: RollingWindowTickerWindowSizeEnum,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct TickerParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradeParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct LotSizeFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderListPlaceResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceResponseResult {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListCancelResponseResultOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListPlaceResponseResultOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct KlinesResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<Vec<models::KlinesItemInner>>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct AvgPriceResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::AvgPriceResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOcoResponseResultOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct TickerPriceResponse1Result {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
}

#[derive(SimpleObject)]
pub struct MyTradesResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer: Option<bool>,
    pub Option: :is_none")]
    pub is_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_best_match: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AccountStatusResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::AccountStatusResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrResponse1Result {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub prev_close_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::AccountCommissionResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct SessionLogonResponseResult {
    pub Option: :is_none")]
    pub api_key: Option<String>,
    pub Option: :is_none")]
    pub authorized_since: Option<i64>,
    pub Option: :is_none")]
    pub connected_since: Option<i64>,
    pub Option: :is_none")]
    pub return_rate_limits: Option<bool>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub user_data_stream: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ListStatusOInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub c: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoResponseResultSymbolsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub base_asset_precision: Option<i64>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub quote_precision: Option<i64>,
    pub Option: :is_none"
    )]
    pub quote_asset_precision: Option<i64>,
    pub Option: :is_none"
    )]
    pub base_commission_precision: Option<i64>,
    pub Option: :is_none"
    )]
    pub quote_commission_precision: Option<i64>,
    pub Option: :is_none")]
    pub order_types: Option<Vec<String>>,
    pub Option: :is_none")]
    pub iceberg_allowed: Option<bool>,
    pub Option: :is_none")]
    pub oco_allowed: Option<bool>,
    pub Option: :is_none")]
    pub oto_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub quote_order_qty_market_allowed: Option<bool>,
    pub Option: :is_none")]
    pub allow_trailing_stop: Option<bool>,
    pub Option: :is_none"
    )]
    pub cancel_replace_allowed: Option<bool>,
    pub Option: :is_none")]
    pub amend_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub peg_instructions_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub is_spot_trading_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub is_margin_trading_allowed: Option<bool>,
    pub Option: :is_none")]
    pub filters: Option<Vec<models::SymbolFilters>>,
    pub Option: :is_none")]
    pub permissions: Option<Vec<String>>,
    pub Option: :is_none")]
    pub permission_sets: Option<Vec<Vec<String>>>,
    pub Option: :is_none"
    )]
    pub default_self_trade_prevention_mode: Option<String>,
    pub Option: :is_none"
    )]
    pub allowed_self_trade_prevention_modes: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct TickerResponse2ResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrderAmendsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_amends: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TradesRecentResponseResultInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_best_match: Option<bool>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponseResult {
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub execution_id: Option<i64>,
    pub Option: :is_none")]
    pub amended_order: Option<Box<models::OrderAmendKeepPriorityResponseResultAmendedOrder>>,
    pub Option: :is_none")]
    pub list_status: Option<Box<models::OrderAmendKeepPriorityResponseResultListStatus>>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumAlgoOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_algo_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderCancelReplaceResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TrailingDeltaFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub min_trailing_above_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub max_trailing_above_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub min_trailing_below_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub max_trailing_below_delta: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumOrderListsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_lists: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponseResultSpecialCommissionForOrder {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
}

#[derive(SimpleObject)]
pub struct NotionalFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_min_to_market: Option<bool>,
    pub Option: :is_none")]
    pub max_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_max_to_market: Option<bool>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseResultSpecialCommission {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrResponse2ResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub prev_close_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListCancelResponseResultOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MyFiltersResponseResult {
    pub Option: :is_none")]
    pub exchange_filters: Option<Vec<models::ExchangeFilters>>,
    pub Option: :is_none")]
    pub symbol_filters: Option<Vec<models::SymbolFilters>>,
    pub Option: :is_none")]
    pub asset_filters: Option<Vec<models::AssetFilters>>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersCancelAllResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::OpenOrdersCancelAllResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseResultCancelResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct TickerBookResponse1Result {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtoResponseResultOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct TimeResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::TimeResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct MinNotionalFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_to_market: Option<bool>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct TickerPriceResponse2ResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponseResultAmendedOrder {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub prevented_qty: Option<String>,
    pub Option: :is_none")]
    pub quote_order_qty: Option<String>,
    pub Option: :is_none")]
    pub cumulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct SorOrderPlaceResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::SorOrderPlaceResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtocoResponseResult {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListPlaceOtocoResponseResultOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListPlaceOtocoResponseResultOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct AllOrdersResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub prevented_match_id: Option<i64>,
    pub Option: :is_none")]
    pub prevented_quantity: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListStatusResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::AllOrderListsResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TickerPriceResponse1 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::TickerPriceResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct UiKlinesResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<Vec<models::KlinesItemInner>>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TickerPriceResponse2 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::TickerPriceResponse2ResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderCancelResponseResult {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub trailing_delta: Option<i64>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_type: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OpenOrdersCancelAllResponseResultInnerOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OpenOrdersCancelAllResponseResultInnerOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct TradesAggregateResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::TradesAggregateResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct MyPreventedMatchesResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::MyPreventedMatchesResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtocoResponseResultOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct EventStreamTerminated {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RateLimits {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExternalLockUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderAmendKeepPriorityResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct MyFiltersResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::MyFiltersResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct SessionStatusResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::SessionStatusResponseResult>>,
}

#[derive(SimpleObject)]
pub struct OrderStatusResponseResult {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub trailing_delta: Option<i64>,
    pub Option: :is_none")]
    pub trailing_time: Option<i64>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_type: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub prevented_match_id: Option<i64>,
    pub Option: :is_none")]
    pub prevented_quantity: Option<String>,
}

#[derive(SimpleObject)]
pub struct SessionLogonResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::SessionLogonResponseResult>>,
}

#[derive(SimpleObject)]
pub struct DepthResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::DepthResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseResultDiscount {
    pub Option: :is_none")]
    pub enabled_for_account: Option<bool>,
    pub Option: :is_none")]
    pub enabled_for_symbol: Option<bool>,
    pub Option: :is_none")]
    pub discount_asset: Option<String>,
    pub Option: :is_none")]
    pub discount: Option<String>,
}

#[derive(SimpleObject)]
pub struct TickerBookResponse2 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::TickerBookResponse2ResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct MyPreventedMatchesResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub prevented_match_id: Option<i64>,
    pub Option: :is_none")]
    pub taker_order_id: Option<i64>,
    pub Option: :is_none")]
    pub maker_symbol: Option<String>,
    pub Option: :is_none")]
    pub maker_order_id: Option<i64>,
    pub Option: :is_none")]
    pub trade_group_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none"
    )]
    pub maker_prevented_quantity: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::ExchangeInfoResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseResult {
    pub Option: :is_none")]
    pub cancel_result: Option<String>,
    pub Option: :is_none")]
    pub new_order_result: Option<String>,
    pub Option: :is_none")]
    pub cancel_response: Option<Box<models::OrderCancelReplaceResponseResultCancelResponse>>,
    pub Option: :is_none")]
    pub new_order_response: Option<Box<models::OrderCancelReplaceResponseResultNewOrderResponse>>,
}

#[derive(SimpleObject)]
pub struct TickerResponse1 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::TickerResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderAmendmentsResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub execution_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub new_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub new_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListCancelResponseResultOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenOrderListsStatusResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::OpenOrderListsStatusResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct PercentPriceFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub multiplier_exponent: Option<i32>,
    pub Option: :is_none")]
    pub multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamSubscribeResponseResult {
    pub Option: :is_none")]
    pub subscription_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountRateLimitsOrdersResponseResultInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListCancelResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderListCancelResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct AccountRateLimitsOrdersResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::AccountRateLimitsOrdersResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct MyAllocationsResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::MyAllocationsResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersCancelAllResponseResultInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtoResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderListPlaceOtoResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct DepthResponseResult {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub asks: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct MaxNumAlgoOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_algo_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoResponseResultSorsInner {
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersStatusResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::OpenOrdersStatusResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponseResultListStatus {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderAmendKeepPriorityResponseResultListStatusOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct TradesRecentResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::TradesRecentResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseResultTaxCommission {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderCancelResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderCancelResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TradesAggregateResponseResultInner {
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub m_uppercase: Option<bool>,
}

#[derive(SimpleObject)]
pub struct MyTradesResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::MyTradesResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrResponse1 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::Ticker24hrResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OutboundAccountPosition {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub b_uppercase: Option<Vec<models::OutboundAccountPositionBInner>>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderTestResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseResultStandardCommission {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MaxAssetFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub limit: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderPlaceResponseResultFillsInner {
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SorOrderPlaceResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none")]
    pub fills: Option<Vec<models::SorOrderPlaceResponseResultInnerFillsInner>>,
    pub Option: :is_none")]
    pub working_floor: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub used_sor: Option<bool>,
}

#[derive(SimpleObject)]
pub struct OrderListCancelResponseResult {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListCancelResponseResultOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListCancelResponseResultOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct TickerResponse2 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::TickerResponse2ResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TickerResponse1Result {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderPlaceResponseResult {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub fills: Option<Vec<models::OrderPlaceResponseResultFillsInner>>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtoResponseResultOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct SessionStatusResponseResult {
    pub Option: :is_none")]
    pub api_key: Option<String>,
    pub Option: :is_none")]
    pub authorized_since: Option<i64>,
    pub Option: :is_none")]
    pub connected_since: Option<i64>,
    pub Option: :is_none")]
    pub return_rate_limits: Option<bool>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub user_data_stream: Option<bool>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamSubscribeSignatureResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::UserDataStreamSubscribeResponseResult>>,
}

#[derive(SimpleObject)]
pub struct BalanceUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub d: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TPlusSellFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllOrderListsResponseResultInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListCancelResponseResultOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct OrderStatusResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderStatusResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponseResultListStatusOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersCancelAllResponseResultInnerOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct SessionLogoutResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::SessionLogoutResponseResult>>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamSubscribeResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::UserDataStreamSubscribeResponseResult>>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrderListsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_lists: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SessionSubscriptionsResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::SessionSubscriptionsResponseResultInner>>,
}

#[derive(SimpleObject)]
pub struct AllOrderListsResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::AllOrderListsResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumIcebergOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub max_num_iceberg_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MyAllocationsResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub allocation_id: Option<i64>,
    pub Option: :is_none")]
    pub allocation_type: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer: Option<bool>,
    pub Option: :is_none")]
    pub is_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_allocator: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AccountStatusResponseResult {
    pub Option: :is_none")]
    pub maker_commission: Option<i64>,
    pub Option: :is_none")]
    pub taker_commission: Option<i64>,
    pub Option: :is_none")]
    pub buyer_commission: Option<i64>,
    pub Option: :is_none")]
    pub seller_commission: Option<i64>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub commission_rates: Option<Box<models::AccountStatusResponseResultCommissionRates>>,
    pub Option: :is_none")]
    pub brokered: Option<bool>,
    pub Option: :is_none"
    )]
    pub require_self_trade_prevention: Option<bool>,
    pub Option: :is_none")]
    pub prevent_sor: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub account_type: Option<String>,
    pub Option: :is_none")]
    pub balances: Option<Vec<models::AccountStatusResponseResultBalancesInner>>,
    pub Option: :is_none")]
    pub permissions: Option<Vec<String>>,
    pub Option: :is_none")]
    pub uid: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOcoResponseResultOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderAmendmentsResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::OrderAmendmentsResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct PriceFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_price: Option<String>,
    pub Option: :is_none")]
    pub max_price: Option<String>,
    pub Option: :is_none")]
    pub tick_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct SorOrderTestResponseResult {
    pub Option: :is_none"
    )]
    pub standard_commission_for_order:
        Option<Box<models::OrderTestResponseResultStandardCommissionForOrder>>,
    pub Option: :is_none"
    )]
    pub tax_commission_for_order:
        Option<Box<models::OrderTestResponseResultStandardCommissionForOrder>>,
    pub Option: :is_none")]
    pub discount: Option<Box<models::OrderTestResponseResultDiscount>>,
}

#[derive(SimpleObject)]
pub struct ExecutionReport {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub f_uppercase: Option<String>,
    pub Option: :is_none")]
    pub g: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<String>,
    pub Option: :is_none")]
    pub n_uppercase: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub v: Option<i64>,
    pub Option: :is_none")]
    pub i_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub w: Option<bool>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub m_uppercase: Option<bool>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub z_uppercase: Option<String>,
    pub Option: :is_none")]
    pub y_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub d: Option<i64>,
    pub Option: :is_none")]
    pub d_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub j: Option<i64>,
    pub Option: :is_none")]
    pub j_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub cs: Option<String>,
    pub Option: :is_none")]
    pub pl: Option<String>,
    pub Option: :is_none")]
    pub p_l: Option<String>,
    pub Option: :is_none")]
    pub p_y: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub k: Option<String>,
    pub Option: :is_none")]
    pub u_s: Option<bool>,
    pub Option: :is_none")]
    pub g_p: Option<String>,
    pub Option: :is_none")]
    pub g_ot: Option<String>,
    pub Option: :is_none")]
    pub g_ov: Option<i64>,
    pub Option: :is_none")]
    pub gp: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountStatusResponseResultBalancesInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceResponseResultOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenOrderListsStatusResponseResultInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct SorOrderTestResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::SorOrderTestResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct IcebergPartsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseResult {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub standard_commission: Option<Box<models::AccountCommissionResponseResultStandardCommission>>,
    pub Option: :is_none")]
    pub special_commission: Option<Box<models::AccountCommissionResponseResultSpecialCommission>>,
    pub Option: :is_none")]
    pub tax_commission: Option<Box<models::AccountCommissionResponseResultTaxCommission>>,
    pub Option: :is_none")]
    pub discount: Option<Box<models::AccountCommissionResponseResultDiscount>>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponseResultDiscount {
    pub Option: :is_none")]
    pub enabled_for_account: Option<bool>,
    pub Option: :is_none")]
    pub enabled_for_symbol: Option<bool>,
    pub Option: :is_none")]
    pub discount_asset: Option<String>,
    pub Option: :is_none")]
    pub discount: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtocoResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderListPlaceOtocoResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrResponse2 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::Ticker24hrResponse2ResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOcoResponseResult {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListPlaceOcoResponseResultOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListPlaceOcoResponseResultOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct MaxNumIcebergOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub max_num_iceberg_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtoResponseResult {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListPlaceOtoResponseResultOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListPlaceOtoResponseResultOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct OrderPlaceResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderPlaceResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TickerTradingDayResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::TickerTradingDayResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct SessionSubscriptionsResponseResultInner {
    pub Option: :is_none")]
    pub subscription_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PingResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<serde_json::Value>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TickerTradingDayResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOcoResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::OrderListPlaceOcoResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct SessionLogoutResponseResult {
    pub Option: :is_none")]
    pub api_key: Option<String>,
    pub Option: :is_none")]
    pub authorized_since: Option<i64>,
    pub Option: :is_none")]
    pub connected_since: Option<i64>,
    pub Option: :is_none")]
    pub return_rate_limits: Option<bool>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub user_data_stream: Option<bool>,
}

#[derive(SimpleObject)]
pub struct SorOrderPlaceResponseResultInnerFillsInner {
    pub Option: :is_none")]
    pub match_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
    pub Option: :is_none")]
    pub alloc_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PercentPriceBySideFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub multiplier_exponent: Option<i32>,
    pub Option: :is_none")]
    pub bid_multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub bid_multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub ask_multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub ask_multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersStatusResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoResponseResult {
    pub Option: :is_none")]
    pub timezone: Option<String>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
    pub Option: :is_none")]
    pub exchange_filters: Option<Vec<models::ExchangeFilters>>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<models::ExchangeInfoResponseResultSymbolsInner>>,
    pub Option: :is_none")]
    pub sors: Option<Vec<models::ExchangeInfoResponseResultSorsInner>>,
}

#[derive(SimpleObject)]
pub struct TimeResponseResult {
    pub Option: :is_none")]
    pub server_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ListStatus {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub g: Option<i64>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub c_uppercase: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o_uppercase: Option<Vec<models::ListStatusOInner>>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponseResult {
    pub Option: :is_none"
    )]
    pub standard_commission_for_order:
        Option<Box<models::OrderTestResponseResultStandardCommissionForOrder>>,
    pub Option: :is_none"
    )]
    pub special_commission_for_order:
        Option<Box<models::OrderTestResponseResultSpecialCommissionForOrder>>,
    pub Option: :is_none"
    )]
    pub tax_commission_for_order:
        Option<Box<models::OrderTestResponseResultStandardCommissionForOrder>>,
    pub Option: :is_none")]
    pub discount: Option<Box<models::OrderTestResponseResultDiscount>>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtocoResponseResultOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct TickerBookResponse1 {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::TickerBookResponse1Result>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct MarketLotSizeFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseResultNewOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct TradesHistoricalResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::TradesHistoricalResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponseResultStandardCommissionForOrder {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersCancelAllResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub trailing_delta: Option<i64>,
    pub Option: :is_none")]
    pub trailing_time: Option<i64>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub strategy_id: Option<i64>,
    pub Option: :is_none")]
    pub strategy_type: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OpenOrdersCancelAllResponseResultInnerOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OpenOrdersCancelAllResponseResultInnerOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamUnsubscribeResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(SimpleObject)]
pub struct MaxPositionFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub max_position: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountStatusResponseResultCommissionRates {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct OutboundAccountPositionBInner {
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
}

#[derive(SimpleObject)]
pub struct AvgPriceResponseResult {
    pub Option: :is_none")]
    pub mins: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllOrdersResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::AllOrdersResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct TradesHistoricalResponseResultInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_best_match: Option<bool>,
}

#[derive(SimpleObject)]
pub struct OpenOrderListsStatusResponseResultInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OpenOrderListsStatusResponseResultInnerOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct TickerBookResponse2ResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountRateLimitsOrdersParams {
    pub id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct AccountStatusParams {
    pub id: Option<String>,
    pub value: false
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub omit_zero_balances: Option<bool>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct AllOrderListsParams {
    pub id: Option<String>,
    pub from_id: Option<i32>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct AllOrdersParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyAllocationsParams {
    pub symbol: String,
    pub id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_allocation_id: Option<i32>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub order_id: Option<i64>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyFiltersParams {
    pub symbol: String,
    pub id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyPreventedMatchesParams {
    pub symbol: String,
    pub id: Option<String>,
    pub prevented_match_id: Option<i64>,
    pub order_id: Option<i64>,
    pub from_prevented_match_id: Option<i64>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyTradesParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i32>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OpenOrderListsStatusParams {
    pub id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersStatusParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderAmendmentsParams {
    pub symbol: String,
    pub order_id: i64,
    pub id: Option<String>,
    pub from_execution_id: Option<i64>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListStatusParams {
    pub id: Option<String>,
    pub orig_client_order_id: Option<String>,
    pub order_list_id: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderStatusParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MarketApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct AvgPriceParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct DepthParams {
    pub symbol: String,
    pub id: Option<String>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub symbol_status: Option<DepthSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct KlinesParams {
    pub symbol: String,
    pub interval: KlinesIntervalEnum,
    pub id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 0 (UTC)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub time_zone: Option<String>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct TickerParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub type: Option<TickerTypeEnum>,
    pub window_size: Option<TickerWindowSizeEnum>,
    pub symbol_status: Option<TickerSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub type: Option<Ticker24hrTypeEnum>,
    pub symbol_status: Option<Ticker24hrSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct TickerBookParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub symbol_status: Option<TickerBookSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct TickerPriceParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub symbol_status: Option<TickerPriceSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct TickerTradingDayParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub Default: 0 (UTC)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub time_zone: Option<String>,
    pub type: Option<TickerTradingDayTypeEnum>,
    pub symbol_status: Option<TickerTradingDaySymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct TradesAggregateParams {
    pub symbol: String,
    pub id: Option<String>,
    pub from_id: Option<i32>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct TradesHistoricalParams {
    pub symbol: String,
    pub id: Option<String>,
    pub from_id: Option<i32>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct TradesRecentParams {
    pub symbol: String,
    pub id: Option<String>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct UiKlinesParams {
    pub symbol: String,
    pub interval: UiKlinesIntervalEnum,
    pub id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 0 (UTC)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub time_zone: Option<String>,
    pub Default: 100; Maximum: 5000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct OpenOrdersCancelAllParams {
    pub symbol: String,
    pub id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityParams {
    pub symbol: String,
    pub new_qty: rust_decimal::Decimal,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderCancelParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub cancel_restrictions: Option<OrderCancelCancelRestrictionsEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceParams {
    pub symbol: String,
    pub cancel_replace_mode: OrderCancelReplaceCancelReplaceModeEnum,
    pub side: OrderCancelReplaceSideEnum,
    pub type: OrderCancelReplaceTypeEnum,
    pub id: Option<String>,
    pub cancel_order_id: Option<i64>,
    pub cancel_orig_client_order_id: Option<String>,
    pub cancel_new_client_order_id: Option<String>,
    pub time_in_force: Option<OrderCancelReplaceTimeInForceEnum>,
    pub price: Option<rust_decimal::Decimal>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<OrderCancelReplaceNewOrderRespTypeEnum>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<rust_decimal::Decimal>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub self_trade_prevention_mode: Option<OrderCancelReplaceSelfTradePreventionModeEnum>,
    pub cancel_restrictions: Option<OrderCancelReplaceCancelRestrictionsEnum>,
    pub order_rate_limit_exceeded_mode: Option<OrderCancelReplaceOrderRateLimitExceededModeEnum>,
    pub peg_price_type: Option<OrderCancelReplacePegPriceTypeEnum>,
    pub max: 100)
    /// See Pegged Orders
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub peg_offset_value: Option<i32>,
    pub peg_offset_type: Option<OrderCancelReplacePegOffsetTypeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListCancelParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_list_id: Option<i32>,
    pub list_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceParams {
    pub symbol: String,
    pub side: OrderListPlaceSideEnum,
    pub price: rust_decimal::Decimal,
    pub quantity: rust_decimal::Decimal,
    pub id: Option<String>,
    pub list_client_order_id: Option<String>,
    pub limit_client_order_id: Option<String>,
    pub limit_iceberg_qty: Option<rust_decimal::Decimal>,
    pub limit_strategy_id: Option<i64>,
    pub limit_strategy_type: Option<i32>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<i32>,
    pub stop_client_order_id: Option<String>,
    pub stop_limit_price: Option<rust_decimal::Decimal>,
    pub stop_limit_time_in_force: Option<OrderListPlaceStopLimitTimeInForceEnum>,
    pub stop_iceberg_qty: Option<rust_decimal::Decimal>,
    pub stop_strategy_id: Option<i64>,
    pub stop_strategy_type: Option<i32>,
    pub new_order_resp_type: Option<OrderListPlaceNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderListPlaceSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOcoParams {
    pub symbol: String,
    pub side: OrderListPlaceOcoSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub above_type: OrderListPlaceOcoAboveTypeEnum,
    pub below_type: OrderListPlaceOcoBelowTypeEnum,
    pub id: Option<String>,
    pub list_client_order_id: Option<String>,
    pub above_client_order_id: Option<String>,
    pub above_iceberg_qty: Option<i64>,
    pub above_price: Option<rust_decimal::Decimal>,
    pub above_stop_price: Option<rust_decimal::Decimal>,
    pub above_trailing_delta: Option<i64>,
    pub above_time_in_force: Option<OrderListPlaceOcoAboveTimeInForceEnum>,
    pub above_strategy_id: Option<i64>,
    pub above_strategy_type: Option<i32>,
    pub above_peg_price_type: Option<OrderListPlaceOcoAbovePegPriceTypeEnum>,
    pub above_peg_offset_type: Option<OrderListPlaceOcoAbovePegOffsetTypeEnum>,
    pub above_peg_offset_value: Option<i32>,
    pub below_client_order_id: Option<String>,
    pub below_iceberg_qty: Option<i64>,
    pub below_price: Option<rust_decimal::Decimal>,
    pub below_stop_price: Option<rust_decimal::Decimal>,
    pub below_trailing_delta: Option<i64>,
    pub below_time_in_force: Option<OrderListPlaceOcoBelowTimeInForceEnum>,
    pub below_strategy_id: Option<i64>,
    pub below_strategy_type: Option<i32>,
    pub below_peg_price_type: Option<OrderListPlaceOcoBelowPegPriceTypeEnum>,
    pub below_peg_offset_type: Option<OrderListPlaceOcoBelowPegOffsetTypeEnum>,
    pub below_peg_offset_value: Option<i32>,
    pub new_order_resp_type: Option<OrderListPlaceOcoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderListPlaceOcoSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtoParams {
    pub symbol: String,
    pub working_type: OrderListPlaceOtoWorkingTypeEnum,
    pub working_side: OrderListPlaceOtoWorkingSideEnum,
    pub working_price: rust_decimal::Decimal,
    pub working_quantity: rust_decimal::Decimal,
    pub pending_type: OrderListPlaceOtoPendingTypeEnum,
    pub pending_side: OrderListPlaceOtoPendingSideEnum,
    pub pending_quantity: rust_decimal::Decimal,
    pub id: Option<String>,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<OrderListPlaceOtoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderListPlaceOtoSelfTradePreventionModeEnum>,
    pub working_client_order_id: Option<String>,
    pub working_iceberg_qty: Option<rust_decimal::Decimal>,
    pub working_time_in_force: Option<OrderListPlaceOtoWorkingTimeInForceEnum>,
    pub working_strategy_id: Option<i64>,
    pub working_strategy_type: Option<i32>,
    pub working_peg_price_type: Option<OrderListPlaceOtoWorkingPegPriceTypeEnum>,
    pub working_peg_offset_type: Option<OrderListPlaceOtoWorkingPegOffsetTypeEnum>,
    pub working_peg_offset_value: Option<i32>,
    pub pending_client_order_id: Option<String>,
    pub pending_price: Option<rust_decimal::Decimal>,
    pub pending_stop_price: Option<rust_decimal::Decimal>,
    pub pending_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_time_in_force: Option<OrderListPlaceOtoPendingTimeInForceEnum>,
    pub pending_strategy_id: Option<i64>,
    pub pending_strategy_type: Option<i32>,
    pub pending_peg_offset_type: Option<OrderListPlaceOtoPendingPegOffsetTypeEnum>,
    pub pending_peg_price_type: Option<OrderListPlaceOtoPendingPegPriceTypeEnum>,
    pub pending_peg_offset_value: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListPlaceOtocoParams {
    pub symbol: String,
    pub working_type: OrderListPlaceOtocoWorkingTypeEnum,
    pub working_side: OrderListPlaceOtocoWorkingSideEnum,
    pub working_price: rust_decimal::Decimal,
    pub working_quantity: rust_decimal::Decimal,
    pub pending_side: OrderListPlaceOtocoPendingSideEnum,
    pub pending_quantity: rust_decimal::Decimal,
    pub pending_above_type: OrderListPlaceOtocoPendingAboveTypeEnum,
    pub id: Option<String>,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<OrderListPlaceOtocoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderListPlaceOtocoSelfTradePreventionModeEnum>,
    pub working_client_order_id: Option<String>,
    pub working_iceberg_qty: Option<rust_decimal::Decimal>,
    pub working_time_in_force: Option<OrderListPlaceOtocoWorkingTimeInForceEnum>,
    pub working_strategy_id: Option<i64>,
    pub working_strategy_type: Option<i32>,
    pub working_peg_price_type: Option<OrderListPlaceOtocoWorkingPegPriceTypeEnum>,
    pub working_peg_offset_type: Option<OrderListPlaceOtocoWorkingPegOffsetTypeEnum>,
    pub working_peg_offset_value: Option<i32>,
    pub pending_above_client_order_id: Option<String>,
    pub pending_above_price: Option<rust_decimal::Decimal>,
    pub pending_above_stop_price: Option<rust_decimal::Decimal>,
    pub pending_above_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_above_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_above_time_in_force: Option<OrderListPlaceOtocoPendingAboveTimeInForceEnum>,
    pub pending_above_strategy_id: Option<i64>,
    pub pending_above_strategy_type: Option<i32>,
    pub pending_above_peg_price_type: Option<OrderListPlaceOtocoPendingAbovePegPriceTypeEnum>,
    pub pending_above_peg_offset_type: Option<OrderListPlaceOtocoPendingAbovePegOffsetTypeEnum>,
    pub pending_above_peg_offset_value: Option<i32>,
    pub pending_below_type: Option<OrderListPlaceOtocoPendingBelowTypeEnum>,
    pub pending_below_client_order_id: Option<String>,
    pub pending_below_price: Option<rust_decimal::Decimal>,
    pub pending_below_stop_price: Option<rust_decimal::Decimal>,
    pub pending_below_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_below_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_below_time_in_force: Option<OrderListPlaceOtocoPendingBelowTimeInForceEnum>,
    pub pending_below_strategy_id: Option<i64>,
    pub pending_below_strategy_type: Option<i32>,
    pub pending_below_peg_price_type: Option<OrderListPlaceOtocoPendingBelowPegPriceTypeEnum>,
    pub pending_below_peg_offset_type: Option<OrderListPlaceOtocoPendingBelowPegOffsetTypeEnum>,
    pub pending_below_peg_offset_value: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderPlaceParams {
    pub symbol: String,
    pub side: OrderPlaceSideEnum,
    pub type: OrderPlaceTypeEnum,
    pub id: Option<String>,
    pub time_in_force: Option<OrderPlaceTimeInForceEnum>,
    pub price: Option<rust_decimal::Decimal>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<OrderPlaceNewOrderRespTypeEnum>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<i32>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub self_trade_prevention_mode: Option<OrderPlaceSelfTradePreventionModeEnum>,
    pub peg_price_type: Option<OrderPlacePegPriceTypeEnum>,
    pub max: 100)
    /// See Pegged Orders
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub peg_offset_value: Option<i32>,
    pub peg_offset_type: Option<OrderPlacePegOffsetTypeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderTestParams {
    pub symbol: String,
    pub side: OrderTestSideEnum,
    pub type: OrderTestTypeEnum,
    pub id: Option<String>,
    pub Default: `false` <br> See [Commissions FAQ](faqs/commission_faq.md#test-order-diferences) to learn more.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub compute_commission_rates: Option<bool>,
    pub time_in_force: Option<OrderTestTimeInForceEnum>,
    pub price: Option<rust_decimal::Decimal>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<OrderTestNewOrderRespTypeEnum>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<i32>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub self_trade_prevention_mode: Option<OrderTestSelfTradePreventionModeEnum>,
    pub peg_price_type: Option<OrderTestPegPriceTypeEnum>,
    pub max: 100)
    /// See Pegged Orders
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub peg_offset_value: Option<i32>,
    pub peg_offset_type: Option<OrderTestPegOffsetTypeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct SorOrderPlaceParams {
    pub symbol: String,
    pub side: SorOrderPlaceSideEnum,
    pub type: SorOrderPlaceTypeEnum,
    pub quantity: rust_decimal::Decimal,
    pub id: Option<String>,
    pub time_in_force: Option<SorOrderPlaceTimeInForceEnum>,
    pub price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<SorOrderPlaceNewOrderRespTypeEnum>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub self_trade_prevention_mode: Option<SorOrderPlaceSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct SorOrderTestParams {
    pub symbol: String,
    pub side: SorOrderTestSideEnum,
    pub type: SorOrderTestTypeEnum,
    pub quantity: rust_decimal::Decimal,
    pub id: Option<String>,
    pub Default: `false` <br> See [Commissions FAQ](faqs/commission_faq.md#test-order-diferences) to learn more.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub compute_commission_rates: Option<bool>,
    pub time_in_force: Option<SorOrderTestTimeInForceEnum>,
    pub price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub new_order_resp_type: Option<SorOrderTestNewOrderRespTypeEnum>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub self_trade_prevention_mode: Option<SorOrderTestSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct SessionSubscriptionsParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamSubscribeParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamSubscribeSignatureParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamUnsubscribeParams {
    pub id: Option<String>,
    pub subscription_id: Option<i32>,
}

#[derive(SimpleObject)]
pub struct AuthApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct SessionLogonParams {
    pub id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct SessionLogoutParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct SessionStatusParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GeneralApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoParams {
    pub id: Option<String>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
    pub show_permission_sets: Option<bool>,
    pub symbol_status: Option<ExchangeInfoSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct PingParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct TimeParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct LotSizeFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TickerTradingDayResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TickerResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AvgPriceResponse {
    pub Option: :is_none")]
    pub mins: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderAmendmentsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub execution_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub new_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub new_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub standard_commission: Option<Box<models::AccountCommissionResponseStandardCommission>>,
    pub Option: :is_none")]
    pub special_commission: Option<Box<models::AccountCommissionResponseSpecialCommission>>,
    pub Option: :is_none")]
    pub tax_commission: Option<Box<models::AccountCommissionResponseTaxCommission>>,
    pub Option: :is_none")]
    pub discount: Option<Box<models::AccountCommissionResponseDiscount>>,
}

#[derive(SimpleObject)]
pub struct TickerTradingDayResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrderAmendsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_amends: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AggTradesResponseInner {
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub m_uppercase: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumAlgoOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_algo_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DeleteOrderListResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponse {
    pub Option: :is_none")]
    pub cancel_result: Option<String>,
    pub Option: :is_none")]
    pub new_order_result: Option<String>,
    pub Option: :is_none")]
    pub cancel_response: Option<Box<models::OrderCancelReplaceResponseCancelResponse>>,
    pub Option: :is_none")]
    pub new_order_response: Option<Box<models::OrderCancelReplaceResponseNewOrderResponse>>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::OrderCancelReplaceResponseData>>,
}

#[derive(SimpleObject)]
pub struct TrailingDeltaFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub min_trailing_above_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub max_trailing_above_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub min_trailing_below_delta: Option<i64>,
    pub Option: :is_none"
    )]
    pub max_trailing_below_delta: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumOrderListsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_lists: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderOcoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListOcoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListOcoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListOcoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct NotionalFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_min_to_market: Option<bool>,
    pub Option: :is_none")]
    pub max_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_max_to_market: Option<bool>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub fills: Option<Vec<models::NewOrderResponseFillsInner>>,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoResponseSymbolsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub base_asset_precision: Option<i64>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub quote_precision: Option<i64>,
    pub Option: :is_none"
    )]
    pub quote_asset_precision: Option<i64>,
    pub Option: :is_none"
    )]
    pub base_commission_precision: Option<i64>,
    pub Option: :is_none"
    )]
    pub quote_commission_precision: Option<i64>,
    pub Option: :is_none")]
    pub order_types: Option<Vec<String>>,
    pub Option: :is_none")]
    pub iceberg_allowed: Option<bool>,
    pub Option: :is_none")]
    pub oco_allowed: Option<bool>,
    pub Option: :is_none")]
    pub oto_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub quote_order_qty_market_allowed: Option<bool>,
    pub Option: :is_none")]
    pub allow_trailing_stop: Option<bool>,
    pub Option: :is_none"
    )]
    pub cancel_replace_allowed: Option<bool>,
    pub Option: :is_none")]
    pub amend_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub peg_instructions_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub is_spot_trading_allowed: Option<bool>,
    pub Option: :is_none"
    )]
    pub is_margin_trading_allowed: Option<bool>,
    pub Option: :is_none")]
    pub filters: Option<Vec<models::SymbolFilters>>,
    pub Option: :is_none")]
    pub permissions: Option<Vec<String>>,
    pub Option: :is_none")]
    pub permission_sets: Option<Vec<Vec<String>>>,
    pub Option: :is_none"
    )]
    pub default_self_trade_prevention_mode: Option<String>,
    pub Option: :is_none"
    )]
    pub allowed_self_trade_prevention_modes: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct TimeResponse {
    pub Option: :is_none")]
    pub server_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MinNotionalFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_notional: Option<String>,
    pub Option: :is_none")]
    pub apply_to_market: Option<bool>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct DeleteOpenOrdersResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct TickerBookTickerResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseCancelResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderOcoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct SorOrderResponseFillsInner {
    pub Option: :is_none")]
    pub match_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
    pub Option: :is_none")]
    pub alloc_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TickerPriceResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetAccountResponseCommissionRates {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListOtoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct HistoricalTradesResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_best_match: Option<bool>,
}

#[derive(SimpleObject)]
pub struct OrderListOcoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponseListStatusOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct RateLimits {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponse {
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub execution_id: Option<i64>,
    pub Option: :is_none")]
    pub amended_order: Option<Box<models::OrderAmendKeepPriorityResponseAmendedOrder>>,
    pub Option: :is_none")]
    pub list_status: Option<Box<models::OrderAmendKeepPriorityResponseListStatus>>,
}

#[derive(SimpleObject)]
pub struct MyFiltersResponse {
    pub Option: :is_none")]
    pub exchange_filters: Option<Vec<models::ExchangeFilters>>,
    pub Option: :is_none")]
    pub symbol_filters: Option<Vec<models::SymbolFilters>>,
    pub Option: :is_none")]
    pub asset_filters: Option<Vec<models::AssetFilters>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
}

#[derive(SimpleObject)]
pub struct OrderListOtocoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct DepthResponse {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub asks: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoResponse {
    pub Option: :is_none")]
    pub timezone: Option<String>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::RateLimits>>,
    pub Option: :is_none")]
    pub exchange_filters: Option<Vec<models::ExchangeFilters>>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<models::ExchangeInfoResponseSymbolsInner>>,
}

#[derive(SimpleObject)]
pub struct TickerResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseStandardCommission {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub prev_close_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PercentPriceFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub multiplier_exponent: Option<i32>,
    pub Option: :is_none")]
    pub multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct AllOrderListResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseData {
    pub Option: :is_none")]
    pub cancel_result: Option<String>,
    pub Option: :is_none")]
    pub new_order_result: Option<String>,
    pub Option: :is_none")]
    pub cancel_response: Option<Box<models::OrderCancelReplaceResponseDataCancelResponse>>,
    pub Option: :is_none")]
    pub new_order_response: Option<Box<models::OrderCancelReplaceResponseDataNewOrderResponse>>,
}

#[derive(SimpleObject)]
pub struct MaxNumAlgoOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_algo_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DeleteOrderListResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::DeleteOrderListResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::DeleteOrderListResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct GetAccountResponse {
    pub Option: :is_none")]
    pub maker_commission: Option<i64>,
    pub Option: :is_none")]
    pub taker_commission: Option<i64>,
    pub Option: :is_none")]
    pub buyer_commission: Option<i64>,
    pub Option: :is_none")]
    pub seller_commission: Option<i64>,
    pub Option: :is_none")]
    pub commission_rates: Option<Box<models::GetAccountResponseCommissionRates>>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub brokered: Option<bool>,
    pub Option: :is_none"
    )]
    pub require_self_trade_prevention: Option<bool>,
    pub Option: :is_none")]
    pub prevent_sor: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub account_type: Option<String>,
    pub Option: :is_none")]
    pub balances: Option<Vec<models::GetAccountResponseBalancesInner>>,
    pub Option: :is_none")]
    pub permissions: Option<Vec<String>>,
    pub Option: :is_none")]
    pub uid: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponseStandardCommissionForOrder {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub prev_close_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub quote_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponse {
    pub Option: :is_none"
    )]
    pub standard_commission_for_order:
        Option<Box<models::OrderTestResponseStandardCommissionForOrder>>,
    pub Option: :is_none"
    )]
    pub special_commission_for_order:
        Option<Box<models::OrderTestResponseSpecialCommissionForOrder>>,
    pub Option: :is_none"
    )]
    pub tax_commission_for_order: Option<Box<models::OrderTestResponseStandardCommissionForOrder>>,
    pub Option: :is_none")]
    pub discount: Option<Box<models::OrderTestResponseDiscount>>,
}

#[derive(SimpleObject)]
pub struct AllOrderListResponseInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::AllOrderListResponseInnerOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct GetAccountResponseBalancesInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MaxAssetFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub limit: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponseAmendedOrder {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub prevented_qty: Option<String>,
    pub Option: :is_none")]
    pub quote_order_qty: Option<String>,
    pub Option: :is_none")]
    pub cumulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseNewOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none")]
    pub fills: Option<Vec<String>>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct DeleteOrderListResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponseSpecialCommissionForOrder {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseSpecialCommission {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllOrdersResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub is_working: Option<bool>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct MyPreventedMatchesResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub prevented_match_id: Option<i64>,
    pub Option: :is_none")]
    pub taker_order_id: Option<i64>,
    pub Option: :is_none")]
    pub maker_symbol: Option<String>,
    pub Option: :is_none")]
    pub maker_order_id: Option<i64>,
    pub Option: :is_none")]
    pub trade_group_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none"
    )]
    pub maker_prevented_quantity: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TPlusSellFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListOtocoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct MyTradesResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer: Option<bool>,
    pub Option: :is_none")]
    pub is_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_best_match: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseDiscount {
    pub Option: :is_none")]
    pub enabled_for_account: Option<bool>,
    pub Option: :is_none")]
    pub enabled_for_symbol: Option<bool>,
    pub Option: :is_none")]
    pub discount_asset: Option<String>,
    pub Option: :is_none")]
    pub discount: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponseFillsInner {
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MaxNumOrderListsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_num_order_lists: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOrderListResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseDataNewOrderResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeMaxNumIcebergOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub max_num_iceberg_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PriceFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub price_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_price: Option<String>,
    pub Option: :is_none")]
    pub max_price: Option<String>,
    pub Option: :is_none")]
    pub tick_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct SorOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none")]
    pub fills: Option<Vec<models::SorOrderResponseFillsInner>>,
    pub Option: :is_none")]
    pub working_floor: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub used_sor: Option<bool>,
}

#[derive(SimpleObject)]
pub struct SorOrderTestResponse {
    pub Option: :is_none"
    )]
    pub standard_commission_for_order:
        Option<Box<models::OrderTestResponseStandardCommissionForOrder>>,
    pub Option: :is_none"
    )]
    pub tax_commission_for_order: Option<Box<models::OrderTestResponseStandardCommissionForOrder>>,
    pub Option: :is_none")]
    pub discount: Option<Box<models::OrderTestResponseDiscount>>,
}

#[derive(SimpleObject)]
pub struct IcebergPartsFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderListOcoResponseOrderReportsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_time: Option<i64>,
    pub Option: :is_none")]
    pub iceberg_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct DeleteOrderResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct MaxNumIcebergOrdersFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none"
    )]
    pub max_num_iceberg_orders: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityResponseListStatus {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderAmendKeepPriorityResponseListStatusOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct OpenOrderListResponseInner {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OpenOrderListResponseInnerOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct OrderListOtoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListOtoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListOtoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct OpenOrderListResponseInnerOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct PercentPriceBySideFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub multiplier_exponent: Option<i32>,
    pub Option: :is_none")]
    pub bid_multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub bid_multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub ask_multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub ask_multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub avg_price_mins: Option<i32>,
}

#[derive(SimpleObject)]
pub struct OrderListOtoResponseOrdersInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountCommissionResponseTaxCommission {
    pub Option: :is_none")]
    pub maker: Option<String>,
    pub Option: :is_none")]
    pub taker: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<String>,
    pub Option: :is_none")]
    pub seller: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderListOtocoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderListOtocoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderListOtocoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct MyAllocationsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub allocation_id: Option<i64>,
    pub Option: :is_none")]
    pub allocation_type: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub quote_qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer: Option<bool>,
    pub Option: :is_none")]
    pub is_maker: Option<bool>,
    pub Option: :is_none")]
    pub is_allocator: Option<bool>,
}

#[derive(SimpleObject)]
pub struct TickerBookTickerResponse1 {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
}

#[derive(SimpleObject)]
pub struct RateLimitOrderResponseInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderTestResponseDiscount {
    pub Option: :is_none")]
    pub enabled_for_account: Option<bool>,
    pub Option: :is_none")]
    pub enabled_for_symbol: Option<bool>,
    pub Option: :is_none")]
    pub discount_asset: Option<String>,
    pub Option: :is_none")]
    pub discount: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarketLotSizeFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOrderListResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::GetOrderListResponseOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct OrderOcoResponse {
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub contingency_type: Option<String>,
    pub Option: :is_none")]
    pub list_status_type: Option<String>,
    pub Option: :is_none")]
    pub list_order_status: Option<String>,
    pub Option: :is_none")]
    pub list_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transaction_time: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::OrderOcoResponseOrdersInner>>,
    pub Option: :is_none")]
    pub order_reports: Option<Vec<models::OrderOcoResponseOrderReportsInner>>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceResponseDataCancelResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_list_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub transact_time: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_quote_order_qty: Option<String>,
    pub Option: :is_none"
    )]
    pub cummulative_quote_qty: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct MaxPositionFilter {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub qty_exponent: Option<i32>,
    pub Option: :is_none")]
    pub max_position: Option<String>,
}

#[derive(SimpleObject)]
pub struct TickerPriceResponse2Inner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountCommissionParams {
    pub symbol: String,
}

#[derive(SimpleObject)]
pub struct AllOrderListParams {
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct AllOrdersParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct GetAccountParams {
    pub value: `false`
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub omit_zero_balances: Option<bool>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct GetOpenOrdersParams {
    pub symbol: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct GetOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct GetOrderListParams {
    pub order_list_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyAllocationsParams {
    pub symbol: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_allocation_id: Option<i32>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub order_id: Option<i64>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyFiltersParams {
    pub symbol: String,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyPreventedMatchesParams {
    pub symbol: String,
    pub prevented_match_id: Option<i64>,
    pub order_id: Option<i64>,
    pub from_prevented_match_id: Option<i64>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MyTradesParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OpenOrderListParams {
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderAmendmentsParams {
    pub symbol: String,
    pub order_id: i64,
    pub from_execution_id: Option<i64>,
    pub Default: 500; Maximum: 1000
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i64>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct RateLimitOrderParams {
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AggTradesParams {
    pub symbol: String,
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct AvgPriceParams {
    pub symbol: String,
}

#[derive(SimpleObject)]
pub struct DepthParams {
    pub symbol: String,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub symbol_status: Option<DepthSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct GetTradesParams {
    pub symbol: String,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct HistoricalTradesParams {
    pub symbol: String,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
    pub from_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct KlinesParams {
    pub symbol: String,
    pub interval: KlinesIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 0 (UTC)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub time_zone: Option<String>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct TickerParams {
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub window_size: Option<TickerWindowSizeEnum>,
    pub type: Option<TickerTypeEnum>,
    pub symbol_status: Option<TickerSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrParams {
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub type: Option<Ticker24hrTypeEnum>,
    pub symbol_status: Option<Ticker24hrSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct TickerBookTickerParams {
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub symbol_status: Option<TickerBookTickerSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct TickerPriceParams {
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub symbol_status: Option<TickerPriceSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct TickerTradingDayParams {
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub Default: 0 (UTC)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub time_zone: Option<String>,
    pub type: Option<TickerTradingDayTypeEnum>,
    pub symbol_status: Option<TickerTradingDaySymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct UiKlinesParams {
    pub symbol: String,
    pub interval: UiKlinesIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 0 (UTC)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub time_zone: Option<String>,
    pub Default: 500; Maximum: 1000.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub limit: Option<i32>,
}

#[derive(SimpleObject)]
pub struct MockMarketApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct DeleteOpenOrdersParams {
    pub symbol: String,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct DeleteOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub cancel_restrictions: Option<DeleteOrderCancelRestrictionsEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct DeleteOrderListParams {
    pub symbol: String,
    pub order_list_id: Option<i64>,
    pub list_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct NewOrderParams {
    pub symbol: String,
    pub side: NewOrderSideEnum,
    pub type: NewOrderTypeEnum,
    pub time_in_force: Option<NewOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<i64>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub new_order_resp_type: Option<NewOrderNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<NewOrderSelfTradePreventionModeEnum>,
    pub peg_price_type: Option<NewOrderPegPriceTypeEnum>,
    pub max: 100).<br>See[`PeggedOrdersInfo`](#pegged-orders-info)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub peg_offset_value: Option<i32>,
    pub peg_offset_type: Option<NewOrderPegOffsetTypeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderAmendKeepPriorityParams {
    pub symbol: String,
    pub new_qty: rust_decimal::Decimal,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderCancelReplaceParams {
    pub symbol: String,
    pub side: OrderCancelReplaceSideEnum,
    pub type: OrderCancelReplaceTypeEnum,
    pub cancel_replace_mode: OrderCancelReplaceCancelReplaceModeEnum,
    pub time_in_force: Option<OrderCancelReplaceTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub cancel_new_client_order_id: Option<String>,
    pub cancel_orig_client_order_id: Option<String>,
    pub cancel_order_id: Option<i64>,
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<i64>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub new_order_resp_type: Option<OrderCancelReplaceNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderCancelReplaceSelfTradePreventionModeEnum>,
    pub cancel_restrictions: Option<OrderCancelReplaceCancelRestrictionsEnum>,
    pub order_rate_limit_exceeded_mode: Option<OrderCancelReplaceOrderRateLimitExceededModeEnum>,
    pub peg_price_type: Option<OrderCancelReplacePegPriceTypeEnum>,
    pub max: 100).<br>See[`PeggedOrdersInfo`](#pegged-orders-info)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub peg_offset_value: Option<i32>,
    pub peg_offset_type: Option<OrderCancelReplacePegOffsetTypeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListOcoParams {
    pub symbol: String,
    pub side: OrderListOcoSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub above_type: OrderListOcoAboveTypeEnum,
    pub below_type: OrderListOcoBelowTypeEnum,
    pub list_client_order_id: Option<String>,
    pub above_client_order_id: Option<String>,
    pub above_iceberg_qty: Option<i64>,
    pub above_price: Option<rust_decimal::Decimal>,
    pub above_stop_price: Option<rust_decimal::Decimal>,
    pub above_trailing_delta: Option<i64>,
    pub above_time_in_force: Option<OrderListOcoAboveTimeInForceEnum>,
    pub above_strategy_id: Option<i64>,
    pub above_strategy_type: Option<i32>,
    pub above_peg_price_type: Option<OrderListOcoAbovePegPriceTypeEnum>,
    pub above_peg_offset_type: Option<OrderListOcoAbovePegOffsetTypeEnum>,
    pub above_peg_offset_value: Option<i32>,
    pub below_client_order_id: Option<String>,
    pub below_iceberg_qty: Option<i64>,
    pub below_price: Option<rust_decimal::Decimal>,
    pub below_stop_price: Option<rust_decimal::Decimal>,
    pub below_trailing_delta: Option<i64>,
    pub below_time_in_force: Option<OrderListOcoBelowTimeInForceEnum>,
    pub below_strategy_id: Option<i64>,
    pub below_strategy_type: Option<i32>,
    pub below_peg_price_type: Option<OrderListOcoBelowPegPriceTypeEnum>,
    pub below_peg_offset_type: Option<OrderListOcoBelowPegOffsetTypeEnum>,
    pub below_peg_offset_value: Option<i32>,
    pub new_order_resp_type: Option<OrderListOcoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderListOcoSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListOtoParams {
    pub symbol: String,
    pub working_type: OrderListOtoWorkingTypeEnum,
    pub working_side: OrderListOtoWorkingSideEnum,
    pub working_price: rust_decimal::Decimal,
    pub working_quantity: rust_decimal::Decimal,
    pub pending_type: OrderListOtoPendingTypeEnum,
    pub pending_side: OrderListOtoPendingSideEnum,
    pub pending_quantity: rust_decimal::Decimal,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<OrderListOtoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderListOtoSelfTradePreventionModeEnum>,
    pub working_client_order_id: Option<String>,
    pub working_iceberg_qty: Option<rust_decimal::Decimal>,
    pub working_time_in_force: Option<OrderListOtoWorkingTimeInForceEnum>,
    pub working_strategy_id: Option<i64>,
    pub working_strategy_type: Option<i32>,
    pub working_peg_price_type: Option<OrderListOtoWorkingPegPriceTypeEnum>,
    pub working_peg_offset_type: Option<OrderListOtoWorkingPegOffsetTypeEnum>,
    pub working_peg_offset_value: Option<i32>,
    pub pending_client_order_id: Option<String>,
    pub pending_price: Option<rust_decimal::Decimal>,
    pub pending_stop_price: Option<rust_decimal::Decimal>,
    pub pending_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_time_in_force: Option<OrderListOtoPendingTimeInForceEnum>,
    pub pending_strategy_id: Option<i64>,
    pub pending_strategy_type: Option<i32>,
    pub pending_peg_price_type: Option<OrderListOtoPendingPegPriceTypeEnum>,
    pub pending_peg_offset_type: Option<OrderListOtoPendingPegOffsetTypeEnum>,
    pub pending_peg_offset_value: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderListOtocoParams {
    pub symbol: String,
    pub working_type: OrderListOtocoWorkingTypeEnum,
    pub working_side: OrderListOtocoWorkingSideEnum,
    pub working_price: rust_decimal::Decimal,
    pub working_quantity: rust_decimal::Decimal,
    pub pending_side: OrderListOtocoPendingSideEnum,
    pub pending_quantity: rust_decimal::Decimal,
    pub pending_above_type: OrderListOtocoPendingAboveTypeEnum,
    pub list_client_order_id: Option<String>,
    pub new_order_resp_type: Option<OrderListOtocoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderListOtocoSelfTradePreventionModeEnum>,
    pub working_client_order_id: Option<String>,
    pub working_iceberg_qty: Option<rust_decimal::Decimal>,
    pub working_time_in_force: Option<OrderListOtocoWorkingTimeInForceEnum>,
    pub working_strategy_id: Option<i64>,
    pub working_strategy_type: Option<i32>,
    pub working_peg_price_type: Option<OrderListOtocoWorkingPegPriceTypeEnum>,
    pub working_peg_offset_type: Option<OrderListOtocoWorkingPegOffsetTypeEnum>,
    pub working_peg_offset_value: Option<i32>,
    pub pending_above_client_order_id: Option<String>,
    pub pending_above_price: Option<rust_decimal::Decimal>,
    pub pending_above_stop_price: Option<rust_decimal::Decimal>,
    pub pending_above_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_above_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_above_time_in_force: Option<OrderListOtocoPendingAboveTimeInForceEnum>,
    pub pending_above_strategy_id: Option<i64>,
    pub pending_above_strategy_type: Option<i32>,
    pub pending_above_peg_price_type: Option<OrderListOtocoPendingAbovePegPriceTypeEnum>,
    pub pending_above_peg_offset_type: Option<OrderListOtocoPendingAbovePegOffsetTypeEnum>,
    pub pending_above_peg_offset_value: Option<i32>,
    pub pending_below_type: Option<OrderListOtocoPendingBelowTypeEnum>,
    pub pending_below_client_order_id: Option<String>,
    pub pending_below_price: Option<rust_decimal::Decimal>,
    pub pending_below_stop_price: Option<rust_decimal::Decimal>,
    pub pending_below_trailing_delta: Option<rust_decimal::Decimal>,
    pub pending_below_iceberg_qty: Option<rust_decimal::Decimal>,
    pub pending_below_time_in_force: Option<OrderListOtocoPendingBelowTimeInForceEnum>,
    pub pending_below_strategy_id: Option<i64>,
    pub pending_below_strategy_type: Option<i32>,
    pub pending_below_peg_price_type: Option<OrderListOtocoPendingBelowPegPriceTypeEnum>,
    pub pending_below_peg_offset_type: Option<OrderListOtocoPendingBelowPegOffsetTypeEnum>,
    pub pending_below_peg_offset_value: Option<i32>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderOcoParams {
    pub symbol: String,
    pub side: OrderOcoSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub stop_price: rust_decimal::Decimal,
    pub list_client_order_id: Option<String>,
    pub limit_client_order_id: Option<String>,
    pub limit_strategy_id: Option<i64>,
    pub limit_strategy_type: Option<i32>,
    pub limit_iceberg_qty: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<i64>,
    pub stop_client_order_id: Option<String>,
    pub stop_strategy_id: Option<i64>,
    pub stop_strategy_type: Option<i32>,
    pub stop_limit_price: Option<rust_decimal::Decimal>,
    pub stop_iceberg_qty: Option<rust_decimal::Decimal>,
    pub stop_limit_time_in_force: Option<OrderOcoStopLimitTimeInForceEnum>,
    pub new_order_resp_type: Option<OrderOcoNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderOcoSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct OrderTestParams {
    pub symbol: String,
    pub side: OrderTestSideEnum,
    pub type: OrderTestTypeEnum,
    pub Default: `false` <br> See [Commissions FAQ](faqs/commission_faq.md#test-order-diferences) to learn more.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub compute_commission_rates: Option<bool>,
    pub time_in_force: Option<OrderTestTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub quote_order_qty: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub stop_price: Option<rust_decimal::Decimal>,
    pub trailing_delta: Option<i64>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub new_order_resp_type: Option<OrderTestNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<OrderTestSelfTradePreventionModeEnum>,
    pub peg_price_type: Option<OrderTestPegPriceTypeEnum>,
    pub max: 100).<br>See[`PeggedOrdersInfo`](#pegged-orders-info)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub peg_offset_value: Option<i32>,
    pub peg_offset_type: Option<OrderTestPegOffsetTypeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct SorOrderParams {
    pub symbol: String,
    pub side: SorOrderSideEnum,
    pub type: SorOrderTypeEnum,
    pub quantity: rust_decimal::Decimal,
    pub time_in_force: Option<SorOrderTimeInForceEnum>,
    pub price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub new_order_resp_type: Option<SorOrderNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<SorOrderSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct SorOrderTestParams {
    pub symbol: String,
    pub side: SorOrderTestSideEnum,
    pub type: SorOrderTestTypeEnum,
    pub quantity: rust_decimal::Decimal,
    pub Default: `false` <br> See [Commissions FAQ](faqs/commission_faq.md#test-order-diferences) to learn more.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub compute_commission_rates: Option<bool>,
    pub time_in_force: Option<SorOrderTestTimeInForceEnum>,
    pub price: Option<rust_decimal::Decimal>,
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<i64>,
    pub strategy_type: Option<i32>,
    pub iceberg_qty: Option<rust_decimal::Decimal>,
    pub new_order_resp_type: Option<SorOrderTestNewOrderRespTypeEnum>,
    pub self_trade_prevention_mode: Option<SorOrderTestSelfTradePreventionModeEnum>,
    pub recv_window: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct GeneralApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct ExchangeInfoParams {
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
    pub show_permission_sets: Option<bool>,
    pub symbol_status: Option<ExchangeInfoSymbolStatusEnum>,
}

#[derive(SimpleObject)]
pub struct MockGeneralApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub future_algo_api_client: FutureAlgoApiClient,
    pub spot_algo_api_client: SpotAlgoApiClient,
}

#[derive(SimpleObject)]
pub struct QueryHistoricalAlgoOrdersFutureAlgoResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryHistoricalAlgoOrdersFutureAlgoResponseOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentAlgoOpenOrdersFutureAlgoResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryCurrentAlgoOpenOrdersFutureAlgoResponseOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct QueryHistoricalAlgoOrdersSpotAlgoResponseOrdersInner {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub total_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_amt: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub urgency: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryHistoricalAlgoOrdersFutureAlgoResponseOrdersInner {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub total_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_amt: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub urgency: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentAlgoOpenOrdersSpotAlgoResponseOrdersInner {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub total_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_amt: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub urgency: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderSpotAlgoResponse {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct TimeWeightedAveragePriceFutureAlgoResponse {
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct TimeWeightedAveragePriceSpotAlgoResponse {
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QuerySubOrdersFutureAlgoResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_amt: Option<String>,
    pub Option: :is_none")]
    pub sub_orders: Option<Vec<models::QuerySubOrdersFutureAlgoResponseSubOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct VolumeParticipationFutureAlgoResponse {
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderFutureAlgoResponse {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentAlgoOpenOrdersSpotAlgoResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryCurrentAlgoOpenOrdersSpotAlgoResponseOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct QueryHistoricalAlgoOrdersSpotAlgoResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub orders: Option<Vec<models::QueryHistoricalAlgoOrdersSpotAlgoResponseOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentAlgoOpenOrdersFutureAlgoResponseOrdersInner {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub total_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_amt: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_algo_id: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub end_time: Option<i64>,
    pub Option: :is_none")]
    pub algo_status: Option<String>,
    pub Option: :is_none")]
    pub algo_type: Option<String>,
    pub Option: :is_none")]
    pub urgency: Option<String>,
}

#[derive(SimpleObject)]
pub struct QuerySubOrdersSpotAlgoResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_amt: Option<String>,
    pub Option: :is_none")]
    pub sub_orders: Option<Vec<models::QuerySubOrdersFutureAlgoResponseSubOrdersInner>>,
}

#[derive(SimpleObject)]
pub struct QuerySubOrdersFutureAlgoResponseSubOrdersInner {
    pub Option: :is_none")]
    pub algo_id: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub order_status: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_amt: Option<String>,
    pub Option: :is_none")]
    pub fee_amt: Option<String>,
    pub Option: :is_none")]
    pub fee_asset: Option<String>,
    pub Option: :is_none")]
    pub book_time: Option<i64>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub sub_id: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
}

#[derive(SimpleObject)]
pub struct SpotAlgoApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderSpotAlgoParams {
    pub algo_id: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentAlgoOpenOrdersSpotAlgoParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryHistoricalAlgoOrdersSpotAlgoParams {
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubOrdersSpotAlgoParams {
    pub algo_id: i64,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TimeWeightedAveragePriceSpotAlgoParams {
    pub symbol: String,
    pub side: String,
    pub quantity: rust_decimal::Decimal,
    pub duration: i64,
    pub client_algo_id: Option<String>,
    pub limit_price: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct MockSpotAlgoApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct FutureAlgoApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CancelAlgoOrderFutureAlgoParams {
    pub algo_id: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentAlgoOpenOrdersFutureAlgoParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryHistoricalAlgoOrdersFutureAlgoParams {
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QuerySubOrdersFutureAlgoParams {
    pub algo_id: i64,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TimeWeightedAveragePriceFutureAlgoParams {
    pub symbol: String,
    pub side: String,
    pub quantity: rust_decimal::Decimal,
    pub duration: i64,
    pub position_side: Option<String>,
    pub client_algo_id: Option<String>,
    pub reduce_only: Option<bool>,
    pub limit_price: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct VolumeParticipationFutureAlgoParams {
    pub symbol: String,
    pub side: String,
    pub quantity: rust_decimal::Decimal,
    pub ENUM: LOW,
    pub urgency: String,
    pub position_side: Option<String>,
    pub client_algo_id: Option<String>,
    pub reduce_only: Option<bool>,
    pub limit_price: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockFutureAlgoApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub asset_api_client: AssetApiClient,
    pub capital_api_client: CapitalApiClient,
    pub others_api_client: OthersApiClient,
    pub travel_rule_api_client: TravelRuleApiClient,
}

#[derive(SimpleObject)]
pub struct AssetDetailResponse {
    pub Option: :is_none")]
    pub ctr: Option<Box<models::AssetDetailResponseCtr>>,
    pub Option: :is_none")]
    pub sky: Option<Box<models::AssetDetailResponseSky>>,
}

#[derive(SimpleObject)]
pub struct FundingWalletResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub freeze: Option<String>,
    pub Option: :is_none")]
    pub withdrawing: Option<String>,
    pub Option: :is_none")]
    pub btc_valuation: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUserUniversalTransferHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryUserUniversalTransferHistoryResponseRowsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountStatusResponse {
    pub Option: :is_none")]
    pub data: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSymbolsDelistScheduleForSpotResponseInner {
    pub Option: :is_none")]
    pub delist_time: Option<i64>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct VaspListResponseInner {
    pub Option: :is_none")]
    pub vasp_name: Option<String>,
    pub Option: :is_none")]
    pub vasp_code: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUserUniversalTransferHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DepositHistoryV2ResponseInnerQuestionnaire {
    pub Option: :is_none")]
    pub vasp_name: Option<String>,
    pub Option: :is_none")]
    pub deposit_originator: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiTradingStatusResponseData {
    pub Option: :is_none")]
    pub is_locked: Option<bool>,
    pub Option: :is_none")]
    pub planned_recover_time: Option<i64>,
    pub Option: :is_none")]
    pub trigger_condition: Option<Box<models::AccountApiTradingStatusResponseDataTriggerCondition>>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AssetDividendRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::AssetDividendRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserDelegationHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::QueryUserDelegationHistoryResponseRowsInner>>,
}

#[derive(SimpleObject)]
pub struct GetAssetsThatCanBeConvertedIntoBnbResponse {
    pub Option: :is_none")]
    pub details: Option<Vec<models::GetAssetsThatCanBeConvertedIntoBnbResponseDetailsInner>>,
    pub Option: :is_none")]
    pub total_transfer_btc: Option<String>,
    pub Option: :is_none")]
    pub total_transfer_bnb: Option<String>,
    pub Option: :is_none")]
    pub dribblet_percentage: Option<String>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub snapshot_vos: Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInner>>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotResponseSnapshotVosInnerDataAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetCloudMiningPaymentAndRefundHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllCoinsInformationResponseInner {
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub deposit_all_enable: Option<bool>,
    pub Option: :is_none")]
    pub withdraw_all_enable: Option<bool>,
    pub Option: :is_none")]
    pub name: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub freeze: Option<String>,
    pub Option: :is_none")]
    pub withdrawing: Option<String>,
    pub Option: :is_none")]
    pub ipoing: Option<String>,
    pub Option: :is_none")]
    pub ipoable: Option<String>,
    pub Option: :is_none")]
    pub storage: Option<String>,
    pub Option: :is_none")]
    pub is_legal_money: Option<bool>,
    pub Option: :is_none")]
    pub trading: Option<bool>,
    pub Option: :is_none")]
    pub network_list: Option<Vec<models::AllCoinsInformationResponseInnerNetworkListInner>>,
}

#[derive(SimpleObject)]
pub struct QueryUserWalletBalanceResponseInner {
    pub Option: :is_none")]
    pub activate: Option<bool>,
    pub Option: :is_none")]
    pub balance: Option<String>,
    pub Option: :is_none")]
    pub wallet_name: Option<String>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotResponseSnapshotVosInnerData {
    pub Option: :is_none")]
    pub balances:
        Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataBalancesInner>>,
    pub Option: :is_none")]
    pub total_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub margin_level: Option<String>,
    pub Option: :is_none"
    )]
    pub total_liability_of_btc: Option<String>,
    pub Option: :is_none")]
    pub total_net_asset_of_btc: Option<String>,
    pub Option: :is_none")]
    pub user_assets:
        Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataUserAssetsInner>>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataAssetsInner>>,
    pub Option: :is_none")]
    pub position:
        Option<Vec<models::DailyAccountSnapshotResponseSnapshotVosInnerDataPositionInner>>,
}

#[derive(SimpleObject)]
pub struct DepositHistoryResponseInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub address_tag: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub insert_time: Option<i64>,
    pub Option: :is_none")]
    pub complete_time: Option<i64>,
    pub Option: :is_none")]
    pub transfer_type: Option<i64>,
    pub Option: :is_none")]
    pub confirm_times: Option<String>,
    pub Option: :is_none")]
    pub unlock_confirm: Option<i64>,
    pub Option: :is_none")]
    pub wallet_type: Option<i64>,
    pub Option: :is_none")]
    pub travel_rule_status: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterestResponse {
    pub Option: :is_none")]
    pub spot_bnb_burn: Option<bool>,
    pub Option: :is_none")]
    pub interest_bnb_burn: Option<bool>,
}

#[derive(SimpleObject)]
pub struct TradeFeeResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub maker_commission: Option<String>,
    pub Option: :is_none")]
    pub taker_commission: Option<String>,
}

#[derive(SimpleObject)]
pub struct BrokerWithdrawResponse {
    pub Option: :is_none")]
    pub tr_id: Option<i64>,
    pub Option: :is_none")]
    pub accpted: Option<bool>,
    pub Option: :is_none")]
    pub info: Option<String>,
}

#[derive(SimpleObject)]
pub struct DustlogResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub user_asset_dribblets: Option<Vec<models::DustlogResponseUserAssetDribbletsInner>>,
}

#[derive(SimpleObject)]
pub struct WithdrawResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotResponseSnapshotVosInnerDataBalancesInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetApiKeyPermissionResponse {
    pub Option: :is_none")]
    pub ip_restrict: Option<bool>,
    pub Option: :is_none")]
    pub create_time: Option<i64>,
    pub Option: :is_none")]
    pub enable_reading: Option<bool>,
    pub Option: :is_none")]
    pub enable_withdrawals: Option<bool>,
    pub Option: :is_none"
    )]
    pub enable_internal_transfer: Option<bool>,
    pub Option: :is_none")]
    pub enable_margin: Option<bool>,
    pub Option: :is_none")]
    pub enable_futures: Option<bool>,
    pub Option: :is_none"
    )]
    pub permits_universal_transfer: Option<bool>,
    pub Option: :is_none"
    )]
    pub enable_vanilla_options: Option<bool>,
    pub Option: :is_none")]
    pub enable_fix_api_trade: Option<bool>,
    pub Option: :is_none")]
    pub enable_fix_read_only: Option<bool>,
    pub Option: :is_none"
    )]
    pub enable_spot_and_margin_trading: Option<bool>,
    pub Option: :is_none"
    )]
    pub enable_portfolio_margin_trading: Option<bool>,
}

#[derive(SimpleObject)]
pub struct FetchWithdrawAddressListResponseInner {
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub address_tag: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub name: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub origin: Option<String>,
    pub Option: :is_none")]
    pub origin_type: Option<String>,
    pub Option: :is_none")]
    pub white_status: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetSpotDelistScheduleResponseInner {
    pub Option: :is_none")]
    pub delist_time: Option<i64>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct DepositHistoryTravelRuleResponseInner {
    pub Option: :is_none")]
    pub tr_id: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub deposit_status: Option<i64>,
    pub Option: :is_none")]
    pub travel_rule_status: Option<i64>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub address_tag: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub insert_time: Option<i64>,
    pub Option: :is_none")]
    pub transfer_type: Option<i64>,
    pub Option: :is_none")]
    pub confirm_times: Option<String>,
    pub Option: :is_none")]
    pub unlock_confirm: Option<i64>,
    pub Option: :is_none")]
    pub wallet_type: Option<i64>,
    pub Option: :is_none"
    )]
    pub require_questionnaire: Option<bool>,
    pub Option: :is_none")]
    pub questionnaire: Option<String>,
}

#[derive(SimpleObject)]
pub struct FetchAddressVerificationListResponseInner {
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub token: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub wallet_address: Option<String>,
    pub Option: :is_none"
    )]
    pub address_questionnaire:
        Option<Box<models::FetchAddressVerificationListResponseInnerAddressQuestionnaire>>,
}

#[derive(SimpleObject)]
pub struct WithdrawHistoryResponseInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub transaction_fee: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub apply_time: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub transfer_type: Option<i64>,
    pub Option: :is_none")]
    pub withdraw_order_id: Option<String>,
    pub Option: :is_none")]
    pub info: Option<String>,
    pub Option: :is_none")]
    pub confirm_no: Option<i64>,
    pub Option: :is_none")]
    pub wallet_type: Option<i64>,
    pub Option: :is_none")]
    pub tx_key: Option<String>,
    pub Option: :is_none")]
    pub complete_time: Option<String>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotResponseSnapshotVosInnerDataUserAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub borrowed: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub interest: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub net_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct OneClickArrivalDepositApplyResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<bool>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct UserUniversalTransferResponse {
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SystemStatusResponse {
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct FetchWithdrawQuotaResponse {
    pub Option: :is_none")]
    pub wd_quota: Option<String>,
    pub Option: :is_none")]
    pub used_wd_quota: Option<String>,
}

#[derive(SimpleObject)]
pub struct FetchDepositAddressListWithNetworkResponseInner {
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub tag: Option<String>,
    pub Option: :is_none")]
    pub is_default: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubmitDepositQuestionnaireTravelRuleResponse {
    pub Option: :is_none")]
    pub tr_id: Option<i64>,
    pub Option: :is_none")]
    pub accepted: Option<bool>,
    pub Option: :is_none")]
    pub info: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetAssetsThatCanBeConvertedIntoBnbResponseDetailsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub asset_full_name: Option<String>,
    pub Option: :is_none")]
    pub amount_free: Option<String>,
    pub Option: :is_none")]
    pub to_btc: Option<String>,
    pub Option: :is_none")]
    pub to_bnb: Option<String>,
    pub Option: :is_none")]
    pub to_bnb_off_exchange: Option<String>,
    pub Option: :is_none")]
    pub exchange: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubmitDepositQuestionnaireResponse {
    pub Option: :is_none")]
    pub tr_id: Option<i64>,
    pub Option: :is_none")]
    pub accepted: Option<bool>,
    pub Option: :is_none")]
    pub info: Option<String>,
}

#[derive(SimpleObject)]
pub struct WithdrawHistoryV2ResponseInner {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub tr_id: Option<i64>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub transaction_fee: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub withdrawal_status: Option<i64>,
    pub Option: :is_none")]
    pub travel_rule_status: Option<i64>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub address_tag: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub apply_time: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub transfer_type: Option<i64>,
    pub Option: :is_none")]
    pub withdraw_order_id: Option<String>,
    pub Option: :is_none")]
    pub info: Option<String>,
    pub Option: :is_none")]
    pub confirm_no: Option<i64>,
    pub Option: :is_none")]
    pub wallet_type: Option<i64>,
    pub Option: :is_none")]
    pub tx_key: Option<String>,
    pub Option: :is_none")]
    pub questionnaire: Option<String>,
    pub Option: :is_none")]
    pub complete_time: Option<String>,
}

#[derive(SimpleObject)]
pub struct WithdrawTravelRuleResponse {
    pub Option: :is_none")]
    pub tr_id: Option<i64>,
    pub Option: :is_none")]
    pub accpted: Option<bool>,
    pub Option: :is_none")]
    pub info: Option<String>,
}

#[derive(SimpleObject)]
pub struct DepositAddressResponse {
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub tag: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
}

#[derive(SimpleObject)]
pub struct AssetDetailResponseSky {
    pub Option: :is_none")]
    pub min_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub deposit_status: Option<bool>,
    pub Option: :is_none")]
    pub withdraw_fee: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub withdraw_status: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AssetDetailResponseCtr {
    pub Option: :is_none")]
    pub min_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub deposit_status: Option<bool>,
    pub Option: :is_none")]
    pub withdraw_fee: Option<i64>,
    pub Option: :is_none")]
    pub withdraw_status: Option<bool>,
    pub Option: :is_none")]
    pub deposit_tip: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOpenSymbolListResponseInner {
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotResponseSnapshotVosInner {
    pub Option: :is_none")]
    pub data: Option<Box<models::DailyAccountSnapshotResponseSnapshotVosInnerData>>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCloudMiningPaymentAndRefundHistoryResponse {
    pub Option: :is_none")]
    pub total: Option<i64>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetCloudMiningPaymentAndRefundHistoryResponseRowsInner>>,
}

#[derive(SimpleObject)]
pub struct DustlogResponseUserAssetDribbletsInner {
    pub Option: :is_none")]
    pub operate_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub total_transfered_amount: Option<String>,
    pub Option: :is_none"
    )]
    pub total_service_charge_amount: Option<String>,
    pub Option: :is_none")]
    pub trans_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub user_asset_dribblet_details:
        Option<Vec<models::DustlogResponseUserAssetDribbletsInnerUserAssetDribbletDetailsInner>>,
}

#[derive(SimpleObject)]
pub struct FetchAddressVerificationListResponseInnerAddressQuestionnaire {
    pub Option: :is_none")]
    pub send_to: Option<i64>,
    pub Option: :is_none")]
    pub satoshi_token: Option<String>,
    pub Option: :is_none")]
    pub is_address_owner: Option<i64>,
    pub Option: :is_none")]
    pub verify_method: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiTradingStatusResponseDataTriggerCondition {
    pub Option: :is_none")]
    pub gcr: Option<i64>,
    pub Option: :is_none")]
    pub ifer: Option<i64>,
    pub Option: :is_none")]
    pub ufr: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DustlogResponseUserAssetDribbletsInnerUserAssetDribbletDetailsInner {
    pub Option: :is_none")]
    pub trans_id: Option<i64>,
    pub Option: :is_none"
    )]
    pub service_charge_amount: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub operate_time: Option<i64>,
    pub Option: :is_none")]
    pub transfered_amount: Option<String>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct DepositHistoryV2ResponseInner {
    pub Option: :is_none")]
    pub deposit_id: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub deposit_status: Option<i64>,
    pub Option: :is_none"
    )]
    pub travel_rule_req_status: Option<i64>,
    pub Option: :is_none")]
    pub address: Option<String>,
    pub Option: :is_none")]
    pub address_tag: Option<String>,
    pub Option: :is_none")]
    pub tx_id: Option<String>,
    pub Option: :is_none")]
    pub transfer_type: Option<i64>,
    pub Option: :is_none")]
    pub confirm_times: Option<String>,
    pub Option: :is_none"
    )]
    pub require_questionnaire: Option<bool>,
    pub Option: :is_none")]
    pub questionnaire: Option<Box<models::DepositHistoryV2ResponseInnerQuestionnaire>>,
    pub Option: :is_none")]
    pub insert_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiTradingStatusResponse {
    pub Option: :is_none")]
    pub data: Option<Box<models::AccountApiTradingStatusResponseData>>,
}

#[derive(SimpleObject)]
pub struct AllCoinsInformationResponseInnerNetworkListInner {
    pub Option: :is_none")]
    pub network: Option<String>,
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none"
    )]
    pub withdraw_integer_multiple: Option<String>,
    pub Option: :is_none")]
    pub is_default: Option<bool>,
    pub Option: :is_none")]
    pub deposit_enable: Option<bool>,
    pub Option: :is_none")]
    pub withdraw_enable: Option<bool>,
    pub Option: :is_none")]
    pub deposit_desc: Option<String>,
    pub Option: :is_none")]
    pub withdraw_desc: Option<String>,
    pub Option: :is_none")]
    pub special_tips: Option<String>,
    pub Option: :is_none"
    )]
    pub special_withdraw_tips: Option<String>,
    pub Option: :is_none")]
    pub name: Option<String>,
    pub Option: :is_none")]
    pub reset_address_status: Option<bool>,
    pub Option: :is_none")]
    pub address_regex: Option<String>,
    pub Option: :is_none")]
    pub memo_regex: Option<String>,
    pub Option: :is_none")]
    pub withdraw_fee: Option<String>,
    pub Option: :is_none")]
    pub withdraw_min: Option<String>,
    pub Option: :is_none")]
    pub withdraw_max: Option<String>,
    pub Option: :is_none"
    )]
    pub withdraw_internal_min: Option<String>,
    pub Option: :is_none")]
    pub deposit_dust: Option<String>,
    pub Option: :is_none")]
    pub min_confirm: Option<i64>,
    pub Option: :is_none")]
    pub un_lock_confirm: Option<i64>,
    pub Option: :is_none")]
    pub same_address: Option<bool>,
    pub Option: :is_none")]
    pub withdraw_tag: Option<bool>,
    pub Option: :is_none"
    )]
    pub estimated_arrival_time: Option<i64>,
    pub Option: :is_none")]
    pub busy: Option<bool>,
    pub Option: :is_none")]
    pub contract_address_url: Option<String>,
    pub Option: :is_none")]
    pub contract_address: Option<String>,
    pub Option: :is_none")]
    pub denomination: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DustTransferResponse {
    pub Option: :is_none")]
    pub total_service_charge: Option<String>,
    pub Option: :is_none")]
    pub total_transfered: Option<String>,
    pub Option: :is_none")]
    pub transfer_result: Option<Vec<models::DustTransferResponseTransferResultInner>>,
}

#[derive(SimpleObject)]
pub struct UserAssetResponseInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub free: Option<String>,
    pub Option: :is_none")]
    pub locked: Option<String>,
    pub Option: :is_none")]
    pub freeze: Option<String>,
    pub Option: :is_none")]
    pub withdrawing: Option<String>,
    pub Option: :is_none")]
    pub ipoable: Option<String>,
    pub Option: :is_none")]
    pub btc_valuation: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryUserDelegationHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub client_tran_id: Option<String>,
    pub Option: :is_none")]
    pub transfer_type: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DustTransferResponseTransferResultInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub operate_time: Option<i64>,
    pub Option: :is_none"
    )]
    pub service_charge_amount: Option<String>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
    pub Option: :is_none")]
    pub transfered_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct CheckQuestionnaireRequirementsResponse {
    pub Option: :is_none"
    )]
    pub questionnaire_country_code: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountInfoResponse {
    pub Option: :is_none")]
    pub vip_level: Option<i64>,
    pub Option: :is_none")]
    pub is_margin_enabled: Option<bool>,
    pub Option: :is_none")]
    pub is_future_enabled: Option<bool>,
    pub Option: :is_none")]
    pub is_options_enabled: Option<bool>,
    pub Option: :is_none"
    )]
    pub is_portfolio_margin_retail_enabled: Option<bool>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotResponseSnapshotVosInnerDataPositionInner {
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
}

#[derive(SimpleObject)]
pub struct AssetDividendRecordResponseRowsInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub div_time: Option<i64>,
    pub Option: :is_none")]
    pub en_info: Option<String>,
    pub Option: :is_none")]
    pub tran_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountApiTradingStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInfoParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountStatusParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DailyAccountSnapshotParams {
    pub type: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DisableFastWithdrawSwitchParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EnableFastWithdrawSwitchParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetApiKeyPermissionParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TravelRuleApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct BrokerWithdrawParams {
    pub address: String,
    pub coin: String,
    pub amount: rust_decimal::Decimal,
    pub withdraw_order_id: String,
    pub questionnaire: String,
    pub originator_pii: String,
    pub signature: String,
    pub address_tag: Option<String>,
    pub network: Option<String>,
    pub address_name: Option<String>,
    pub transaction_fee_flag: Option<bool>,
    pub wallet_type: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CheckQuestionnaireRequirementsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DepositHistoryTravelRuleParams {
    pub tr_id: Option<String>,
    pub tx_id: Option<String>,
    pub tran_id: Option<String>,
    pub network: Option<String>,
    pub coin: Option<String>,
    pub 0: Completed,
    pub 1: Pending,
    pub 2: Failed
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub travel_rule_status: Option<i64>,
    pub true: Only return records that pending deposit questionnaire. false/not provided: return all records.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub pending_questionnaire: Option<bool>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 0
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DepositHistoryV2Params {
    pub deposit_id: Option<String>,
    pub tx_id: Option<String>,
    pub network: Option<String>,
    pub coin: Option<String>,
    pub true: return `questionnaire` within response.
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub retrieve_questionnaire: Option<bool>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 0
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FetchAddressVerificationListParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubmitDepositQuestionnaireParams {
    pub sub_account_id: String,
    pub deposit_id: String,
    pub questionnaire: String,
    pub beneficiary_pii: String,
    pub signature: String,
    pub network: Option<String>,
    pub coin: Option<String>,
    pub amount: Option<rust_decimal::Decimal>,
    pub address: Option<String>,
    pub address_tag: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubmitDepositQuestionnaireTravelRuleParams {
    pub tran_id: i64,
    pub questionnaire: String,
}

#[derive(SimpleObject)]
pub struct VaspListParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WithdrawHistoryV1Params {
    pub tr_id: Option<String>,
    pub tx_id: Option<String>,
    pub withdraw_order_id: Option<String>,
    pub network: Option<String>,
    pub coin: Option<String>,
    pub 0: Completed,
    pub 1: Pending,
    pub 2: Failed
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub travel_rule_status: Option<i64>,
    pub Default: 0
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WithdrawHistoryV2Params {
    pub tr_id: Option<String>,
    pub tx_id: Option<String>,
    pub withdraw_order_id: Option<String>,
    pub network: Option<String>,
    pub coin: Option<String>,
    pub 0: Completed,
    pub 1: Pending,
    pub 2: Failed
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub travel_rule_status: Option<i64>,
    pub Default: 0
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WithdrawTravelRuleParams {
    pub coin: String,
    pub address: String,
    pub amount: rust_decimal::Decimal,
    pub questionnaire: String,
    pub withdraw_order_id: Option<String>,
    pub network: Option<String>,
    pub address_tag: Option<String>,
    pub transaction_fee_flag: Option<bool>,
    pub name: Option<String>,
    pub wallet_type: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTravelRuleApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct OthersApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetSymbolsDelistScheduleForSpotParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockOthersApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct CapitalApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AllCoinsInformationParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DepositAddressParams {
    pub coin: String,
    pub network: Option<String>,
    pub amount: Option<rust_decimal::Decimal>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DepositHistoryParams {
    pub Default: `false`,
    pub include_source: Option<bool>,
    pub coin: Option<String>,
    pub 0: Email Sent,
    pub 2: Awaiting Approval 3:Rejected 4:Processing 6:Completed)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub status: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 0
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
    pub tx_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct FetchDepositAddressListWithNetworkParams {
    pub coin: String,
    pub network: Option<String>,
}

#[derive(SimpleObject)]
pub struct OneClickArrivalDepositApplyParams {
    pub deposit_id: Option<i64>,
    pub tx_id: Option<String>,
    pub sub_account_id: Option<i64>,
    pub sub_user_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WithdrawParams {
    pub coin: String,
    pub address: String,
    pub amount: rust_decimal::Decimal,
    pub withdraw_order_id: Option<String>,
    pub network: Option<String>,
    pub address_tag: Option<String>,
    pub transaction_fee_flag: Option<bool>,
    pub name: Option<String>,
    pub wallet_type: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WithdrawHistoryParams {
    pub coin: Option<String>,
    pub withdraw_order_id: Option<String>,
    pub 0: Email Sent,
    pub 2: Awaiting Approval 3:Rejected 4:Processing 6:Completed)
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub status: Option<i64>,
    pub Default: 0
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub id_list: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockCapitalApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct AssetApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AssetDetailParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AssetDividendRecordParams {
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DustTransferParams {
    pub asset: String,
    pub account_type: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DustlogParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FundingWalletParams {
    pub asset: Option<String>,
    pub need_btc_valuation: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetAssetsThatCanBeConvertedIntoBnbParams {
    pub account_type: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCloudMiningPaymentAndRefundHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub tran_id: Option<i64>,
    pub client_tran_id: Option<String>,
    pub asset: Option<String>,
    pub current: Option<i64>,
    pub size: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserDelegationHistoryParams {
    pub email: String,
    pub start_time: i64,
    pub end_time: i64,
    pub type: Option<String>,
    pub asset: Option<String>,
    pub current: Option<i64>,
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserUniversalTransferHistoryParams {
    pub type: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub current: Option<i64>,
    pub size: Option<i64>,
    pub from_symbol: Option<String>,
    pub to_symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryUserWalletBalanceParams {
    pub quote_asset: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ToggleBnbBurnOnSpotTradeAndMarginInterestParams {
    pub spot_bnb_burn: Option<String>,
    pub interest_bnb_burn: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TradeFeeParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UserAssetParams {
    pub asset: Option<String>,
    pub need_btc_valuation: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UserUniversalTransferParams {
    pub type: String,
    pub asset: String,
    pub amount: rust_decimal::Decimal,
    pub from_symbol: Option<String>,
    pub to_symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAssetApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub market_data_api_client: MarketDataApiClient,
}

#[derive(SimpleObject)]
pub struct FetchTokenLimitResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Vec<models::FetchTokenLimitResponseDataInner>>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct CreateADualTokenGiftCardResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::CreateADualTokenGiftCardResponseData>>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct FetchRsaPublicKeyResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<String>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct VerifyBinanceGiftCardByGiftCardNumberResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::VerifyBinanceGiftCardByGiftCardNumberResponseData>>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct FetchTokenLimitResponseDataInner {
    pub Option: :is_none")]
    pub coin: Option<String>,
    pub Option: :is_none")]
    pub from_min: Option<String>,
    pub Option: :is_none")]
    pub from_max: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemABinanceGiftCardResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::RedeemABinanceGiftCardResponseData>>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct CreateASingleTokenGiftCardResponse {
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub message: Option<String>,
    pub Option: :is_none")]
    pub data: Option<Box<models::CreateADualTokenGiftCardResponseData>>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct VerifyBinanceGiftCardByGiftCardNumberResponseData {
    pub Option: :is_none")]
    pub valid: Option<bool>,
    pub Option: :is_none")]
    pub token: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemABinanceGiftCardResponseData {
    pub Option: :is_none")]
    pub reference_no: Option<String>,
    pub Option: :is_none")]
    pub identity_no: Option<String>,
    pub Option: :is_none")]
    pub token: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct CreateADualTokenGiftCardResponseData {
    pub Option: :is_none")]
    pub reference_no: Option<String>,
    pub Option: :is_none")]
    pub code: Option<String>,
    pub Option: :is_none")]
    pub expired_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct CreateADualTokenGiftCardParams {
    pub example: BUSD
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub base_token: String,
    pub example: BNB. If faceToken = baseToken,
    pub face_token: String,
    pub example: 1.002
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub base_token_amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CreateASingleTokenGiftCardParams {
    pub token: String,
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FetchRsaPublicKeyParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FetchTokenLimitParams {
    pub example: BUSD
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub base_token: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemABinanceGiftCardParams {
    pub code: String,
    pub external_uid: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct VerifyBinanceGiftCardByGiftCardNumberParams {
    pub reference_no: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub account_api_client: AccountApiClient,
    pub market_data_api_client: MarketDataApiClient,
    pub portfolio_margin_endpoints_api_client: PortfolioMarginEndpointsApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_data_streams_api_client: UserDataStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketApi {
    pub websocket_api_base: Arc<WebsocketApiBase>,
    pub account_api_client: AccountApiClient,
    pub trade_api_client: TradeApiClient,
    pub user_data_streams_api_client: UserDataStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketApiHandle {
    pub configuration: ConfigurationWebsocketApi,
}

#[derive(SimpleObject)]
pub struct WebsocketStreams {
    pub websocket_streams_base: Arc<WebsocketStreamsBase>,
    pub websocket_market_streams_api_client: WebsocketMarketStreamsApiClient,
}

#[derive(SimpleObject)]
pub struct WebsocketStreamsHandle {
    pub configuration: ConfigurationWebsocketStreams,
}

#[derive(SimpleObject)]
pub struct AccountUpdateA {
    pub Option: :is_none")]
    pub m: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<Vec<models::AccountUpdateABInner>>,
    pub Option: :is_none")]
    pub p_uppercase: Option<Vec<models::AccountUpdateAPInner>>,
}

#[derive(SimpleObject)]
pub struct AllMarketMiniTickersStreamResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceKlineCandlestickStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::MarkPriceKlineCandlestickStreamsResponseK>>,
}

#[derive(SimpleObject)]
pub struct AccountConfigUpdateAc {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub l: Option<i64>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::KlineCandlestickStreamsResponseK>>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolBookTickerStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllMarketLiquidationOrderStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<Box<models::AllMarketLiquidationOrderStreamsResponseO>>,
}

#[derive(SimpleObject)]
pub struct MarkPriceOfAllSymbolsOfAPairResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct DiffBookDepthStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub pu: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub a: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub ct: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::ContinuousContractKlineCandlestickStreamsResponseK>>,
}

#[derive(SimpleObject)]
pub struct GridUpdate {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub gu: Option<Box<models::GridUpdateGu>>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateABInner {
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub wb: Option<String>,
    pub Option: :is_none")]
    pub cw: Option<String>,
    pub Option: :is_none")]
    pub bc: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickStreamsResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndexKlineCandlestickStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub k: Option<Box<models::IndexKlineCandlestickStreamsResponseK>>,
}

#[derive(SimpleObject)]
pub struct ContractInfoStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub ct: Option<String>,
    pub Option: :is_none")]
    pub dt: Option<i64>,
    pub Option: :is_none")]
    pub ot: Option<i64>,
    pub Option: :is_none")]
    pub cs: Option<String>,
    pub Option: :is_none")]
    pub bks: Option<Vec<models::ContractInfoStreamResponseBksInner>>,
}

#[derive(SimpleObject)]
pub struct AccountConfigUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub ac: Option<Box<models::AccountConfigUpdateAc>>,
}

#[derive(SimpleObject)]
pub struct Listenkeyexpired {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllBookTickersStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub a_uppercase: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct IndexKlineCandlestickStreamsResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceKlineCandlestickStreamsResponseK {
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub n: Option<i64>,
    pub Option: :is_none")]
    pub x: Option<bool>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub b_uppercase: Option<String>,
}

#[derive(SimpleObject)]
pub struct LiquidationOrderStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub o: Option<Box<models::AllMarketLiquidationOrderStreamsResponseO>>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolTickerStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarginCallPInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub pa: Option<String>,
    pub Option: :is_none")]
    pub mt: Option<String>,
    pub Option: :is_none")]
    pub iw: Option<String>,
    pub Option: :is_none")]
    pub mp: Option<String>,
    pub Option: :is_none")]
    pub up: Option<String>,
    pub Option: :is_none")]
    pub mm: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub a: Option<Box<models::AccountUpdateA>>,
}

#[derive(SimpleObject)]
pub struct MarginCall {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub cw: Option<String>,
    pub Option: :is_none")]
    pub p: Option<Vec<models::MarginCallPInner>>,
}

#[derive(SimpleObject)]
pub struct AccountUpdateAPInner {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub pa: Option<String>,
    pub Option: :is_none")]
    pub ep: Option<String>,
    pub Option: :is_none")]
    pub bep: Option<String>,
    pub Option: :is_none")]
    pub cr: Option<String>,
    pub Option: :is_none")]
    pub up: Option<String>,
    pub Option: :is_none")]
    pub mt: Option<String>,
    pub Option: :is_none")]
    pub iw: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketLiquidationOrderStreamsResponseO {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub ap: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct IndexPriceStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
}

#[derive(SimpleObject)]
pub struct StrategyUpdateSu {
    pub Option: :is_none")]
    pub si: Option<i64>,
    pub Option: :is_none")]
    pub st: Option<String>,
    pub Option: :is_none")]
    pub ss: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ut: Option<i64>,
    pub Option: :is_none")]
    pub c: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllMarketTickersStreamsResponseInner {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub w: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub q_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub o_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub c_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub f_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub l_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub n: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AggregateTradeStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
}

#[derive(SimpleObject)]
pub struct ContractInfoStreamResponseBksInner {
    pub Option: :is_none")]
    pub bs: Option<i64>,
    pub Option: :is_none")]
    pub bnf: Option<i64>,
    pub Option: :is_none")]
    pub bnc: Option<i64>,
    pub Option: :is_none")]
    pub mmr: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub cf: Option<i64>,
    pub Option: :is_none")]
    pub mi: Option<i64>,
    pub Option: :is_none")]
    pub ma: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthStreamsResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub u_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub u: Option<i64>,
    pub Option: :is_none")]
    pub pu: Option<i64>,
    pub Option: :is_none")]
    pub b: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub a: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdate {
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub o: Option<Box<models::OrderTradeUpdateO>>,
}

#[derive(SimpleObject)]
pub struct GridUpdateGu {
    pub Option: :is_none")]
    pub si: Option<i64>,
    pub Option: :is_none")]
    pub st: Option<String>,
    pub Option: :is_none")]
    pub ss: Option<String>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub up: Option<String>,
    pub Option: :is_none")]
    pub uq: Option<String>,
    pub Option: :is_none")]
    pub uf: Option<String>,
    pub Option: :is_none")]
    pub mp: Option<String>,
    pub Option: :is_none")]
    pub ut: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderTradeUpdateO {
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub s_uppercase: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub f: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub ap: Option<String>,
    pub Option: :is_none")]
    pub sp: Option<String>,
    pub Option: :is_none")]
    pub x: Option<String>,
    pub Option: :is_none")]
    pub x_uppercase: Option<String>,
    pub Option: :is_none")]
    pub i: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub z: Option<String>,
    pub Option: :is_none")]
    pub l_uppercase: Option<String>,
    pub Option: :is_none")]
    pub ma: Option<String>,
    pub Option: :is_none")]
    pub n_uppercase: Option<String>,
    pub Option: :is_none")]
    pub n: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t: Option<i64>,
    pub Option: :is_none")]
    pub rp: Option<String>,
    pub Option: :is_none")]
    pub b: Option<String>,
    pub Option: :is_none")]
    pub a: Option<String>,
    pub Option: :is_none")]
    pub m: Option<bool>,
    pub Option: :is_none")]
    pub r_uppercase: Option<bool>,
    pub Option: :is_none")]
    pub wt: Option<String>,
    pub Option: :is_none")]
    pub ot: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub cp: Option<bool>,
    pub Option: :is_none")]
    pub ap_uppercase: Option<String>,
    pub Option: :is_none")]
    pub cr: Option<String>,
    pub Option: :is_none")]
    pub p_p: Option<bool>,
    pub Option: :is_none")]
    pub v_uppercase: Option<String>,
    pub Option: :is_none")]
    pub pm: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolMiniTickerStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub c: Option<String>,
    pub Option: :is_none")]
    pub o: Option<String>,
    pub Option: :is_none")]
    pub h: Option<String>,
    pub Option: :is_none")]
    pub l: Option<String>,
    pub Option: :is_none")]
    pub v: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
}

#[derive(SimpleObject)]
pub struct StrategyUpdate {
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub su: Option<Box<models::StrategyUpdateSu>>,
}

#[derive(SimpleObject)]
pub struct MarkPriceStreamResponse {
    pub Option: :is_none")]
    pub e: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub s: Option<String>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub p_uppercase: Option<String>,
    pub Option: :is_none")]
    pub i: Option<String>,
    pub Option: :is_none")]
    pub r: Option<String>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WebsocketMarketStreamsApiClient {
    pub websocket_streams_base: Arc<WebsocketStreams>,
}

#[derive(SimpleObject)]
pub struct AggregateTradeStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllBookTickersStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketLiquidationOrderStreamsParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketMiniTickersStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllMarketTickersStreamsParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickStreamsParams {
    pub pair: String,
    pub contract_type: String,
    pub interval: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ContractInfoStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct DiffBookDepthStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndexKlineCandlestickStreamsParams {
    pub pair: String,
    pub interval: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndexPriceStreamParams {
    pub pair: String,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolBookTickerStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolMiniTickerStreamParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndividualSymbolTickerStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickStreamsParams {
    pub symbol: String,
    pub interval: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct LiquidationOrderStreamsParams {
    pub symbol: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceKlineCandlestickStreamsParams {
    pub symbol: String,
    pub interval: String,
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceOfAllSymbolsOfAPairParams {
    pub pair: String,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct MarkPriceStreamParams {
    pub symbol: String,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct PartialBookDepthStreamsParams {
    pub symbol: String,
    pub levels: i64,
    pub id: Option<String>,
    pub update_speed: Option<String>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::FuturesAccountBalanceResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct CloseUserDataStreamResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<serde_json::Value>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CloseUserDataStreamResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamResponseResult {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::NewOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CancelOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::KeepaliveUserDataStreamResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CloseUserDataStreamResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct CancelOrderResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponseResult {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct PositionInformationResponseResultInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub margin_type: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub is_auto_add_margin: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub notional_value: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponseResult {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceResponseResultInner {
    pub Option: :is_none")]
    pub account_alias: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub balance: Option<String>,
    pub Option: :is_none")]
    pub withdraw_available: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderResponseResult {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::StartUserDataStreamResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::StartUserDataStreamResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct CloseUserDataStreamResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::QueryOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CancelOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseResult {
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub assets: Option<Vec<models::AccountInformationResponseResultAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::AccountInformationResponseResultPositionsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseResultPositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub isolated: Option<bool>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub notional_value: Option<String>,
    pub Option: :is_none")]
    pub isolated_wallet: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
}

#[derive(SimpleObject)]
pub struct CancelOrderResponseResult {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponseRateLimitsInner {
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::AccountInformationResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct PositionInformationResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Vec<models::PositionInformationResponseResultInner>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::AccountInformationResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseResultAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderResponseResult {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::CancelOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CancelOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderResponse {
    pub Option: :is_none")]
    pub id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<i64>,
    pub Option: :is_none")]
    pub result: Option<Box<models::ModifyOrderResponseResult>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::CancelOrderResponseRateLimitsInner>>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct AccountInformationParams {
    pub id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceParams {
    pub id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct CancelOrderParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderParams {
    pub symbol: String,
    pub side: ModifyOrderSideEnum,
    pub quantity: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub price_match: Option<ModifyOrderPriceMatchEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewOrderParams {
    pub symbol: String,
    pub side: NewOrderSideEnum,
    pub type: NewOrderTypeEnum,
    pub id: Option<String>,
    pub position_side: Option<NewOrderPositionSideEnum>,
    pub time_in_force: Option<NewOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct PositionInformationParams {
    pub id: Option<String>,
    pub margin_asset: Option<String>,
    pub pair: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderParams {
    pub symbol: String,
    pub id: Option<String>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UserDataStreamsApiClient {
    pub websocket_api_base: Arc<WebsocketApi>,
}

#[derive(SimpleObject)]
pub struct CloseUserDataStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct KeepaliveUserDataStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamParams {
    pub id: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetIncomeHistoryResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub income_type: Option<String>,
    pub Option: :is_none")]
    pub income: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub info: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub tran_id: Option<String>,
    pub Option: :is_none")]
    pub trade_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct UsersForceOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInnerAmendment {
    pub Option: :is_none")]
    pub price: Option<Box<models::GetOrderModifyHistoryResponseInnerAmendmentPrice>>,
    pub Option: :is_none")]
    pub orig_qty: Option<Box<models::GetOrderModifyHistoryResponseInnerAmendmentOrigQty>>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTransactionHistoryDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct CompressedAggregateTradesListResponseInner {
    pub Option: :is_none")]
    pub a: Option<i64>,
    pub Option: :is_none")]
    pub p: Option<String>,
    pub Option: :is_none")]
    pub q: Option<String>,
    pub Option: :is_none")]
    pub f: Option<i64>,
    pub Option: :is_none")]
    pub l: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub m: Option<bool>,
}

#[derive(SimpleObject)]
pub struct CancelAllOpenOrdersResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInner {
    pub Option: :is_none")]
    pub amendment_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub amendment: Option<Box<models::GetOrderModifyHistoryResponseInnerAmendment>>,
}

#[derive(SimpleObject)]
pub struct QueryIndexPriceConstituentsResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub constituents: Option<Vec<models::QueryIndexPriceConstituentsResponseConstituentsInner>>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub ps: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub un_realized_profit: Option<String>,
    pub Option: :is_none")]
    pub liquidation_price: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub margin_type: Option<String>,
    pub Option: :is_none")]
    pub isolated_margin: Option<String>,
    pub Option: :is_none")]
    pub is_auto_add_margin: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetPositionMarginChangeHistoryResponseInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
}

#[derive(SimpleObject)]
pub struct PositionAdlQuantileEstimationResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub adl_quantile: Option<Box<models::PositionAdlQuantileEstimationResponseInnerAdlQuantile>>,
}

#[derive(SimpleObject)]
pub struct NewOrderResponse {
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetCurrentPositionModeResponse {
    pub Option: :is_none")]
    pub dual_side_position: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTradeHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseSymbolsInnerFiltersInner {
    pub Option: :is_none")]
    pub filter_type: Option<String>,
    pub Option: :is_none")]
    pub max_price: Option<String>,
    pub Option: :is_none")]
    pub min_price: Option<String>,
    pub Option: :is_none")]
    pub tick_size: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub min_qty: Option<String>,
    pub Option: :is_none")]
    pub step_size: Option<String>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub multiplier_up: Option<String>,
    pub Option: :is_none")]
    pub multiplier_down: Option<String>,
    pub Option: :is_none")]
    pub multiplier_decimal: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFuturesOrderHistoryDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct NotionalBracketForPairResponseInnerBracketsInner {
    pub Option: :is_none")]
    pub bracket: Option<i64>,
    pub Option: :is_none")]
    pub initial_leverage: Option<i64>,
    pub Option: :is_none")]
    pub qty_cap: Option<i64>,
    pub Option: :is_none")]
    pub qtyl_floor: Option<i64>,
    pub Option: :is_none")]
    pub maint_margin_ratio: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub cum: Option<rust_decimal::Decimal>,
}

#[derive(SimpleObject)]
pub struct NotionalBracketForPairResponseInner {
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub brackets: Option<Vec<models::NotionalBracketForPairResponseInnerBracketsInner>>,
}

#[derive(SimpleObject)]
pub struct OldTradesLookupResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub base_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer_maker: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AccountTradeListResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub realized_pnl: Option<String>,
    pub Option: :is_none")]
    pub margin_asset: Option<String>,
    pub Option: :is_none")]
    pub base_qty: Option<String>,
    pub Option: :is_none")]
    pub commission: Option<String>,
    pub Option: :is_none")]
    pub commission_asset: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub buyer: Option<bool>,
    pub Option: :is_none")]
    pub maker: Option<bool>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponsePositionsInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub position_amt: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub leverage: Option<String>,
    pub Option: :is_none")]
    pub isolated: Option<bool>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub entry_price: Option<String>,
    pub Option: :is_none")]
    pub break_even_price: Option<String>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub notional_value: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentOpenOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseSymbolsInner {
    pub Option: :is_none")]
    pub filters: Option<Vec<models::ExchangeInformationResponseSymbolsInnerFiltersInner>>,
    pub Option: :is_none")]
    pub order_type: Option<Vec<String>>,
    pub Option: :is_none")]
    pub time_in_force: Option<Vec<String>>,
    pub Option: :is_none")]
    pub liquidation_fee: Option<String>,
    pub Option: :is_none")]
    pub market_take_bound: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub contract_type: Option<String>,
    pub Option: :is_none")]
    pub delivery_date: Option<i64>,
    pub Option: :is_none")]
    pub onboard_date: Option<i64>,
    pub Option: :is_none")]
    pub contract_status: Option<String>,
    pub Option: :is_none")]
    pub contract_size: Option<i64>,
    pub Option: :is_none")]
    pub quote_asset: Option<String>,
    pub Option: :is_none")]
    pub base_asset: Option<String>,
    pub Option: :is_none")]
    pub margin_asset: Option<String>,
    pub Option: :is_none")]
    pub price_precision: Option<i64>,
    pub Option: :is_none")]
    pub quantity_precision: Option<i64>,
    pub Option: :is_none")]
    pub base_asset_precision: Option<i64>,
    pub Option: :is_none")]
    pub quote_precision: Option<i64>,
    pub Option: :is_none")]
    pub equal_qty_precision: Option<i64>,
    pub Option: :is_none")]
    pub trigger_protect: Option<String>,
    pub Option: :is_none")]
    pub maint_margin_percent: Option<String>,
    pub Option: :is_none"
    )]
    pub required_margin_percent: Option<String>,
    pub Option: :is_none")]
    pub underlying_type: Option<String>,
    pub Option: :is_none")]
    pub underlying_sub_type: Option<Vec<String>>,
}

#[derive(SimpleObject)]
pub struct IndexPriceAndMarkPriceResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub mark_price: Option<String>,
    pub Option: :is_none")]
    pub index_price: Option<String>,
    pub Option: :is_none"
    )]
    pub estimated_settle_price: Option<String>,
    pub Option: :is_none")]
    pub last_funding_rate: Option<String>,
    pub Option: :is_none")]
    pub interest_rate: Option<String>,
    pub Option: :is_none")]
    pub next_funding_time: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OpenInterestStatisticsResponseInner {
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub contract_type: Option<String>,
    pub Option: :is_none")]
    pub sum_open_interest: Option<String>,
    pub Option: :is_none"
    )]
    pub sum_open_interest_value: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct LongShortRatioResponseInner {
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub long_short_ratio: Option<String>,
    pub Option: :is_none")]
    pub long_account: Option<String>,
    pub Option: :is_none")]
    pub short_account: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIndexPriceConstituentsResponseConstituentsInner {
    pub Option: :is_none")]
    pub exchange: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyMultipleOrdersResponseInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct CurrentAllOpenOrdersResponseInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFundingRateInfoResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub adjusted_funding_rate_cap: Option<String>,
    pub Option: :is_none"
    )]
    pub adjusted_funding_rate_floor: Option<String>,
    pub Option: :is_none"
    )]
    pub funding_interval_hours: Option<i64>,
    pub Option: :is_none")]
    pub disclaimer: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInnerAmendmentPrice {
    pub Option: :is_none")]
    pub before: Option<String>,
    pub Option: :is_none")]
    pub after: Option<String>,
}

#[derive(SimpleObject)]
pub struct OrderBookResponse {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub e_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub t_uppercase: Option<i64>,
    pub Option: :is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    pub Option: :is_none")]
    pub asks: Option<Vec<Vec<String>>>,
}

#[derive(SimpleObject)]
pub struct RecentTradesListResponseInner {
    pub Option: :is_none")]
    pub id: Option<i64>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub qty: Option<String>,
    pub Option: :is_none")]
    pub base_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub is_buyer_maker: Option<bool>,
}

#[derive(SimpleObject)]
pub struct PositionAdlQuantileEstimationResponseInnerAdlQuantile {
    pub Option: :is_none")]
    pub long: Option<i64>,
    pub Option: :is_none")]
    pub short: Option<i64>,
    pub Option: :is_none")]
    pub hedge: Option<i64>,
    pub Option: :is_none")]
    pub both: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTradeDownloadLinkByIdResponse {
    pub Option: :is_none")]
    pub download_id: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub url: Option<String>,
    pub Option: :is_none")]
    pub notified: Option<bool>,
    pub Option: :is_none"
    )]
    pub expiration_timestamp: Option<i64>,
    pub Option: :is_none")]
    pub is_expired: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerResponseInner {
    pub Option: :is_none")]
    pub last_update_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub bid_price: Option<String>,
    pub Option: :is_none")]
    pub bid_qty: Option<String>,
    pub Option: :is_none")]
    pub ask_price: Option<String>,
    pub Option: :is_none")]
    pub ask_qty: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTransactionHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioPositionsResponseInner {
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub long_short_ratio: Option<String>,
    pub Option: :is_none")]
    pub long_position: Option<String>,
    pub Option: :is_none")]
    pub short_position: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct StartUserDataStreamResponse {
    pub Option: :is_none")]
    pub listen_key: Option<String>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrPriceChangeStatisticsResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub price_change: Option<String>,
    pub Option: :is_none")]
    pub price_change_percent: Option<String>,
    pub Option: :is_none")]
    pub weighted_avg_price: Option<String>,
    pub Option: :is_none")]
    pub last_price: Option<String>,
    pub Option: :is_none")]
    pub last_qty: Option<String>,
    pub Option: :is_none")]
    pub open_price: Option<String>,
    pub Option: :is_none")]
    pub high_price: Option<String>,
    pub Option: :is_none")]
    pub low_price: Option<String>,
    pub Option: :is_none")]
    pub volume: Option<String>,
    pub Option: :is_none")]
    pub base_volume: Option<String>,
    pub Option: :is_none")]
    pub open_time: Option<i64>,
    pub Option: :is_none")]
    pub close_time: Option<i64>,
    pub Option: :is_none")]
    pub first_id: Option<i64>,
    pub Option: :is_none")]
    pub last_id: Option<i64>,
    pub Option: :is_none")]
    pub count: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OpenInterestResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub open_interest: Option<String>,
    pub Option: :is_none")]
    pub contract_type: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesOrderHistoryResponse {
    pub Option: :is_none"
    )]
    pub avg_cost_timestamp_of_last30d: Option<i64>,
    pub Option: :is_none")]
    pub download_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangeInitialLeverageResponse {
    pub Option: :is_none")]
    pub leverage: Option<i64>,
    pub Option: :is_none")]
    pub max_qty: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
}

#[derive(SimpleObject)]
pub struct QueryOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponse {
    pub Option: :is_none")]
    pub exchange_filters: Option<Vec<String>>,
    pub Option: :is_none")]
    pub rate_limits: Option<Vec<models::ExchangeInformationResponseRateLimitsInner>>,
    pub Option: :is_none")]
    pub server_time: Option<i64>,
    pub Option: :is_none")]
    pub symbols: Option<Vec<models::ExchangeInformationResponseSymbolsInner>>,
    pub Option: :is_none")]
    pub timezone: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryResponseInnerAmendmentOrigQty {
    pub Option: :is_none")]
    pub before: Option<String>,
    pub Option: :is_none")]
    pub after: Option<String>,
}

#[derive(SimpleObject)]
pub struct AllOrdersResponseInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetFundingRateHistoryOfPerpetualFuturesResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub funding_time: Option<i64>,
    pub Option: :is_none")]
    pub funding_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponse {
    pub Option: :is_none")]
    pub assets: Option<Vec<models::AccountInformationResponseAssetsInner>>,
    pub Option: :is_none")]
    pub positions: Option<Vec<models::AccountInformationResponsePositionsInner>>,
    pub Option: :is_none")]
    pub can_deposit: Option<bool>,
    pub Option: :is_none")]
    pub can_trade: Option<bool>,
    pub Option: :is_none")]
    pub can_withdraw: Option<bool>,
    pub Option: :is_none")]
    pub fee_tier: Option<i64>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioAccountsResponseInner {
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub long_short_ratio: Option<String>,
    pub Option: :is_none")]
    pub long_account: Option<String>,
    pub Option: :is_none")]
    pub short_account: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountInformationResponseAssetsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub unrealized_profit: Option<String>,
    pub Option: :is_none")]
    pub margin_balance: Option<String>,
    pub Option: :is_none")]
    pub maint_margin: Option<String>,
    pub Option: :is_none")]
    pub initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub position_initial_margin: Option<String>,
    pub Option: :is_none"
    )]
    pub open_order_initial_margin: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CheckServerTimeResponse {
    pub Option: :is_none")]
    pub server_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMultipleOrdersResponseInner {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct TakerBuySellVolumeResponseInner {
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub contract_type: Option<String>,
    pub Option: :is_none")]
    pub taker_buy_vol: Option<String>,
    pub Option: :is_none")]
    pub taker_sell_vol: Option<String>,
    pub Option: :is_none")]
    pub taker_buy_vol_value: Option<String>,
    pub Option: :is_none")]
    pub taker_sell_vol_value: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeMarginTypeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct ChangePositionModeResponse {
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
}

#[derive(SimpleObject)]
pub struct UserCommissionRateResponse {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none"
    )]
    pub maker_commission_rate: Option<String>,
    pub Option: :is_none"
    )]
    pub taker_commission_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyIsolatedPositionMarginResponse {
    pub Option: :is_none")]
    pub amount: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub code: Option<i64>,
    pub Option: :is_none")]
    pub msg: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ClassicPortfolioMarginAccountInformationResponse {
    pub Option: :is_none"
    )]
    pub max_withdraw_amount_usd: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub max_withdraw_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyMultipleOrdersBatchOrdersParameterInner {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_client_order_id: Option<String>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub side: Option<SideEnum>,
    pub Option: :is_none")]
    pub quantity: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub price: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ExchangeInformationResponseRateLimitsInner {
    pub Option: :is_none")]
    pub interval: Option<String>,
    pub Option: :is_none")]
    pub interval_num: Option<i64>,
    pub Option: :is_none")]
    pub limit: Option<i64>,
    pub Option: :is_none")]
    pub rate_limit_type: Option<String>,
}

#[derive(SimpleObject)]
pub struct BasisResponseInner {
    pub Option: :is_none")]
    pub index_price: Option<String>,
    pub Option: :is_none")]
    pub contract_type: Option<String>,
    pub Option: :is_none")]
    pub basis_rate: Option<String>,
    pub Option: :is_none")]
    pub futures_price: Option<String>,
    pub Option: :is_none"
    )]
    pub annualized_basis_rate: Option<String>,
    pub Option: :is_none")]
    pub basis: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub timestamp: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NotionalBracketForSymbolResponseInner {
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub notional_coef: Option<rust_decimal::Decimal>,
    pub Option: :is_none")]
    pub brackets: Option<Vec<models::NotionalBracketForPairResponseInnerBracketsInner>>,
}

#[derive(SimpleObject)]
pub struct CancelOrderResponse {
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub activate_price: Option<String>,
    pub Option: :is_none")]
    pub price_rate: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderResponse {
    pub Option: :is_none")]
    pub order_id: Option<i64>,
    pub Option: :is_none")]
    pub symbol: Option<String>,
    pub Option: :is_none")]
    pub pair: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none")]
    pub client_order_id: Option<String>,
    pub Option: :is_none")]
    pub price: Option<String>,
    pub Option: :is_none")]
    pub avg_price: Option<String>,
    pub Option: :is_none")]
    pub orig_qty: Option<String>,
    pub Option: :is_none")]
    pub executed_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_qty: Option<String>,
    pub Option: :is_none")]
    pub cum_base: Option<String>,
    pub Option: :is_none")]
    pub time_in_force: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub reduce_only: Option<bool>,
    pub Option: :is_none")]
    pub close_position: Option<bool>,
    pub Option: :is_none")]
    pub side: Option<String>,
    pub Option: :is_none")]
    pub position_side: Option<String>,
    pub Option: :is_none")]
    pub stop_price: Option<String>,
    pub Option: :is_none")]
    pub working_type: Option<String>,
    pub Option: :is_none")]
    pub price_protect: Option<bool>,
    pub Option: :is_none")]
    pub orig_type: Option<String>,
    pub Option: :is_none")]
    pub price_match: Option<String>,
    pub Option: :is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceResponseInner {
    pub Option: :is_none")]
    pub account_alias: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub balance: Option<String>,
    pub Option: :is_none")]
    pub withdraw_available: Option<String>,
    pub Option: :is_none")]
    pub cross_wallet_balance: Option<String>,
    pub Option: :is_none")]
    pub cross_un_pnl: Option<String>,
    pub Option: :is_none")]
    pub available_balance: Option<String>,
    pub Option: :is_none")]
    pub update_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AccountApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountInformationParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct FuturesAccountBalanceParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCurrentPositionModeParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesOrderHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTradeHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetDownloadIdForFuturesTransactionHistoryParams {
    pub start_time: i64,
    pub end_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesOrderHistoryDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTradeDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFuturesTransactionHistoryDownloadLinkByIdParams {
    pub download_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetIncomeHistoryParams {
    pub symbol: Option<String>,
    pub income_type: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NotionalBracketForPairParams {
    pub pair: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NotionalBracketForSymbolParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UserCommissionRateParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockAccountApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct TradeApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct AccountTradeListParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
    pub order_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub from_id: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AllOrdersParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
    pub order_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct AutoCancelAllOpenOrdersParams {
    pub symbol: String,
    pub countdown_time: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelAllOpenOrdersParams {
    pub symbol: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelMultipleOrdersParams {
    pub symbol: String,
    pub order_id_list: Option<Vec<i64>>,
    pub orig_client_order_id_list: Option<Vec<String>>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CancelOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeInitialLeverageParams {
    pub symbol: String,
    pub leverage: int from 1 to 125
    ///
    /// This field is **required.
    #[builder(setter(into))]
    pub leverage: i64,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangeMarginTypeParams {
    pub symbol: String,
    pub margin_type: ChangeMarginTypeMarginTypeEnum,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ChangePositionModeParams {
    pub dual_side_position: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CurrentAllOpenOrdersParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOrderModifyHistoryParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetPositionMarginChangeHistoryParams {
    pub symbol: String,
    pub 1: Add position margin,
    pub 2: Reduce position margin
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub type: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyIsolatedPositionMarginParams {
    pub symbol: String,
    pub amount: rust_decimal::Decimal,
    pub type: ModifyIsolatedPositionMarginTypeEnum,
    pub position_side: Option<ModifyIsolatedPositionMarginPositionSideEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyMultipleOrdersParams {
    pub batch_orders: Vec<models::ModifyMultipleOrdersBatchOrdersParameterInner>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ModifyOrderParams {
    pub symbol: String,
    pub side: ModifyOrderSideEnum,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub price: Option<rust_decimal::Decimal>,
    pub price_match: Option<ModifyOrderPriceMatchEnum>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct NewOrderParams {
    pub symbol: String,
    pub side: NewOrderSideEnum,
    pub type: NewOrderTypeEnum,
    pub position_side: Option<NewOrderPositionSideEnum>,
    pub time_in_force: Option<NewOrderTimeInForceEnum>,
    pub quantity: Option<rust_decimal::Decimal>,
    pub reduce_only: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub rule: `^[\.A-Z\:/a-z0-9_-]{1,
}

#[derive(SimpleObject)]
pub struct PositionAdlQuantileEstimationParams {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PositionInformationParams {
    pub margin_asset: Option<String>,
    pub pair: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryCurrentOpenOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryOrderParams {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct UsersForceOrdersParams {
    pub symbol: Option<String>,
    pub auto_close_type: Option<UsersForceOrdersAutoCloseTypeEnum>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockTradeApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct MarketDataApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct BasisParams {
    pub pair: String,
    pub contract_type: BasisContractTypeEnum,
    pub period: BasisPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct CompressedAggregateTradesListParams {
    pub symbol: String,
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ContinuousContractKlineCandlestickDataParams {
    pub pair: String,
    pub contract_type: ContinuousContractKlineCandlestickDataContractTypeEnum,
    pub interval: ContinuousContractKlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetFundingRateHistoryOfPerpetualFuturesParams {
    pub symbol: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct IndexPriceAndMarkPriceParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
}

#[derive(SimpleObject)]
pub struct IndexPriceKlineCandlestickDataParams {
    pub pair: String,
    pub interval: IndexPriceKlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct KlineCandlestickDataParams {
    pub symbol: String,
    pub interval: KlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct LongShortRatioParams {
    pub pair: String,
    pub period: LongShortRatioPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MarkPriceKlineCandlestickDataParams {
    pub symbol: String,
    pub interval: MarkPriceKlineCandlestickDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OldTradesLookupParams {
    pub symbol: String,
    pub limit: Option<i64>,
    pub from_id: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OpenInterestParams {
    pub symbol: String,
}

#[derive(SimpleObject)]
pub struct OpenInterestStatisticsParams {
    pub pair: String,
    pub contract_type: OpenInterestStatisticsContractTypeEnum,
    pub period: OpenInterestStatisticsPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OrderBookParams {
    pub symbol: String,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct PremiumIndexKlineDataParams {
    pub symbol: String,
    pub interval: PremiumIndexKlineDataIntervalEnum,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct QueryIndexPriceConstituentsParams {
    pub symbol: String,
}

#[derive(SimpleObject)]
pub struct RecentTradesListParams {
    pub symbol: String,
    pub limit: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SymbolOrderBookTickerParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
}

#[derive(SimpleObject)]
pub struct SymbolPriceTickerParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
}

#[derive(SimpleObject)]
pub struct TakerBuySellVolumeParams {
    pub pair: String,
    pub contract_type: TakerBuySellVolumeContractTypeEnum,
    pub period: TakerBuySellVolumePeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct Ticker24hrPriceChangeStatisticsParams {
    pub symbol: Option<String>,
    pub pair: Option<String>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioAccountsParams {
    pub symbol: String,
    pub period: TopTraderLongShortRatioAccountsPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct TopTraderLongShortRatioPositionsParams {
    pub pair: String,
    pub period: TopTraderLongShortRatioPositionsPeriodEnum,
    pub limit: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockMarketDataApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct PortfolioMarginEndpointsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct ClassicPortfolioMarginAccountInformationParams {
    pub asset: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockPortfolioMarginEndpointsApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct UserDataStreamsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct MockUserDataStreamsApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct RestApi {
    pub configuration: ConfigurationRestApi,
    pub eth_staking_api_client: EthStakingApiClient,
    pub on_chain_yields_api_client: OnChainYieldsApiClient,
    pub soft_staking_api_client: SoftStakingApiClient,
    pub sol_staking_api_client: SolStakingApiClient,
}

#[derive(SimpleObject)]
pub struct GetWbethWrapHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedSubscriptionRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetOnChainYieldsLockedSubscriptionRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeSolStakingResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub bnsol_amount: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct WrapBethResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub wbeth_amount: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetWbethRateHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetWbethRateHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetEthRedemptionHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetEthRedemptionHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemOnChainYieldsLockedProductResponse {
    pub Option: :is_none")]
    pub redeem_id: Option<i64>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct SetSoftStakingResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductListResponseRowsInner {
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub detail: Option<Box<models::GetOnChainYieldsLockedProductListResponseRowsInnerDetail>>,
    pub Option: :is_none")]
    pub quota: Option<Box<models::GetOnChainYieldsLockedProductListResponseRowsInnerQuota>>,
}

#[derive(SimpleObject)]
pub struct GetSolRedemptionHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub distribute_asset: Option<String>,
    pub Option: :is_none")]
    pub distribute_amount: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBoostRewardsHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBoostRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethRewardsHistoryResponse {
    pub Option: :is_none")]
    pub est_rewards_in_eth: Option<String>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetWbethRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBnsolRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub amount_in_sol: Option<String>,
    pub Option: :is_none")]
    pub holding: Option<String>,
    pub Option: :is_none")]
    pub holding_in_sol: Option<String>,
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub lock_period: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedRedemptionRecordResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<String>,
    pub Option: :is_none")]
    pub redeem_id: Option<i64>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub lock_period: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub original_amount: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub deliver_date: Option<String>,
    pub Option: :is_none")]
    pub loss_amount: Option<String>,
    pub Option: :is_none")]
    pub is_complete: Option<bool>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub reward_amt: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductListResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetOnChainYieldsLockedProductListResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBoostRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub token: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub bnsol_holding: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemEthResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub eth_amount: Option<String>,
    pub Option: :is_none")]
    pub conversion_ratio: Option<String>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductPositionResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetOnChainYieldsLockedProductPositionResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetEthStakingHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetEthStakingHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub amount_in_eth: Option<String>,
    pub Option: :is_none")]
    pub holding: Option<String>,
    pub Option: :is_none")]
    pub holding_in_eth: Option<String>,
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubscribeEthStakingResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub wbeth_amount: Option<String>,
    pub Option: :is_none")]
    pub conversion_ratio: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetUnclaimedRewardsResponseInner {
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub rewards_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedRedemptionRecordResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetOnChainYieldsLockedRedemptionRecordResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductPositionResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<String>,
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub purchase_time: Option<String>,
    pub Option: :is_none")]
    pub duration: Option<String>,
    pub Option: :is_none")]
    pub accrual_days: Option<String>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub apy: Option<String>,
    pub Option: :is_none")]
    pub reward_amt: Option<String>,
    pub Option: :is_none")]
    pub next_pay: Option<String>,
    pub Option: :is_none")]
    pub next_pay_date: Option<String>,
    pub Option: :is_none")]
    pub pay_period: Option<String>,
    pub Option: :is_none")]
    pub rewards_pay_date: Option<String>,
    pub Option: :is_none")]
    pub rewards_end_date: Option<String>,
    pub Option: :is_none")]
    pub deliver_date: Option<String>,
    pub Option: :is_none"
    )]
    pub next_subscription_date: Option<String>,
    pub Option: :is_none")]
    pub redeeming_amt: Option<String>,
    pub Option: :is_none")]
    pub redeem_to: Option<String>,
    pub Option: :is_none")]
    pub can_redeem_early: Option<bool>,
    pub Option: :is_none")]
    pub auto_subscribe: Option<bool>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBnsolRateHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBnsolRateHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductListResponseRowsInnerDetail {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub duration: Option<i64>,
    pub Option: :is_none")]
    pub renewable: Option<bool>,
    pub Option: :is_none")]
    pub is_sold_out: Option<bool>,
    pub Option: :is_none")]
    pub apr: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
    pub Option: :is_none"
    )]
    pub subscription_start_time: Option<i64>,
    pub Option: :is_none")]
    pub can_redeem_to_flex: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductListResponseRowsInnerQuota {
    pub Option: :is_none")]
    pub total_personal_quota: Option<String>,
    pub Option: :is_none")]
    pub minimum: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSoftStakingRewardsHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub rewards: Option<String>,
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub avg_amount: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedPersonalLeftQuotaResponse {
    pub Option: :is_none")]
    pub left_personal_quota: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSolStakingHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetSolStakingHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSolRedemptionHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetSolRedemptionHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetOnChainYieldsLockedAutoSubscribeResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetSolStakingQuotaDetailsResponse {
    pub Option: :is_none"
    )]
    pub left_staking_personal_quota: Option<String>,
    pub Option: :is_none"
    )]
    pub left_redemption_personal_quota: Option<String>,
    pub Option: :is_none")]
    pub min_stake_amount: Option<String>,
    pub Option: :is_none")]
    pub min_redeem_amount: Option<String>,
    pub Option: :is_none")]
    pub redeem_period: Option<i64>,
    pub Option: :is_none")]
    pub stakeable: Option<bool>,
    pub Option: :is_none")]
    pub redeemable: Option<bool>,
    pub Option: :is_none")]
    pub sold_out: Option<bool>,
    pub Option: :is_none")]
    pub commission_fee: Option<String>,
    pub Option: :is_none")]
    pub next_epoch_time: Option<i64>,
    pub Option: :is_none")]
    pub calculating: Option<bool>,
}

#[derive(SimpleObject)]
pub struct SolStakingAccountResponse {
    pub Option: :is_none")]
    pub bnsol_amount: Option<String>,
    pub Option: :is_none")]
    pub holding_in_sol: Option<String>,
    pub Option: :is_none"
    )]
    pub thirty_days_profit_in_sol: Option<String>,
}

#[derive(SimpleObject)]
pub struct EthStakingAccountResponse {
    pub Option: :is_none")]
    pub holding_in_eth: Option<String>,
    pub Option: :is_none")]
    pub holdings: Option<Box<models::EthStakingAccountResponseHoldings>>,
    pub Option: :is_none"
    )]
    pub thirty_days_profit_in_eth: Option<String>,
    pub Option: :is_none")]
    pub profit: Option<Box<models::EthStakingAccountResponseProfit>>,
}

#[derive(SimpleObject)]
pub struct SetOnChainYieldsLockedProductRedeemOptionResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetWbethWrapHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetWbethWrapHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSoftStakingProductListResponse {
    pub Option: :is_none")]
    pub status: Option<bool>,
    pub Option: :is_none")]
    pub total_rewards_usdt: Option<String>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetSoftStakingProductListResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct ClaimBoostRewardsResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetSoftStakingRewardsHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetSoftStakingRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethUnwrapHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetWbethUnwrapHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethUnwrapHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub from_asset: Option<String>,
    pub Option: :is_none")]
    pub from_amount: Option<String>,
    pub Option: :is_none")]
    pub to_asset: Option<String>,
    pub Option: :is_none")]
    pub to_amount: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct RedeemSolResponse {
    pub Option: :is_none")]
    pub success: Option<bool>,
    pub Option: :is_none")]
    pub sol_amount: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethRateHistoryResponseRowsInner {
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetEthRedemptionHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub arrival_time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub distribute_asset: Option<String>,
    pub Option: :is_none")]
    pub distribute_amount: Option<String>,
    pub Option: :is_none")]
    pub conversion_ratio: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBnsolRateHistoryResponseRowsInnerBoostRewardsInner {
    pub Option: :is_none")]
    pub boost_apr: Option<String>,
    pub Option: :is_none")]
    pub rewards_asset: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetSolStakingHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub distribute_asset: Option<String>,
    pub Option: :is_none")]
    pub distribute_amount: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedRewardsHistoryResponse {
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetOnChainYieldsLockedRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSoftStakingProductListResponseRowsInner {
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub min_amount: Option<String>,
    pub Option: :is_none")]
    pub max_cap: Option<String>,
    pub Option: :is_none")]
    pub apr: Option<String>,
    pub Option: :is_none")]
    pub staked_amount: Option<String>,
    pub Option: :is_none")]
    pub total_profit: Option<String>,
}

#[derive(SimpleObject)]
pub struct OnChainYieldsAccountResponse {
    pub Option: :is_none")]
    pub total_amount_in_btc: Option<String>,
    pub Option: :is_none")]
    pub total_amount_in_usdt: Option<String>,
    pub Option: :is_none"
    )]
    pub total_flexible_amount_in_btc: Option<String>,
    pub Option: :is_none"
    )]
    pub total_flexible_amount_in_usdt: Option<String>,
    pub Option: :is_none")]
    pub total_locked_in_btc: Option<String>,
    pub Option: :is_none")]
    pub total_locked_in_usdt: Option<String>,
}

#[derive(SimpleObject)]
pub struct EthStakingAccountResponseProfit {
    pub Option: :is_none")]
    pub amount_from_wbeth: Option<String>,
    pub Option: :is_none")]
    pub amount_from_beth: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedSubscriptionPreviewResponse {
    pub Option: :is_none")]
    pub reward_asset: Option<String>,
    pub Option: :is_none")]
    pub total_reward_amt: Option<String>,
    pub Option: :is_none")]
    pub next_pay: Option<String>,
    pub Option: :is_none")]
    pub next_pay_date: Option<String>,
    pub Option: :is_none")]
    pub rewards_pay_date: Option<String>,
    pub Option: :is_none")]
    pub value_date: Option<String>,
    pub Option: :is_none")]
    pub rewards_end_date: Option<String>,
    pub Option: :is_none")]
    pub deliver_date: Option<String>,
    pub Option: :is_none"
    )]
    pub next_subscription_date: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedSubscriptionRecordResponseRowsInner {
    pub Option: :is_none")]
    pub position_id: Option<String>,
    pub Option: :is_none")]
    pub purchase_id: Option<String>,
    pub Option: :is_none")]
    pub project_id: Option<String>,
    pub Option: :is_none")]
    pub client_id: Option<String>,
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub lock_period: Option<String>,
    pub Option: :is_none")]
    pub r#type: Option<String>,
    pub Option: :is_none")]
    pub source_account: Option<String>,
    pub Option: :is_none")]
    pub amt_from_spot: Option<String>,
    pub Option: :is_none")]
    pub amt_from_funding: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetEthStakingHistoryResponseRowsInner {
    pub Option: :is_none")]
    pub time: Option<i64>,
    pub Option: :is_none")]
    pub asset: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub distribute_asset: Option<String>,
    pub Option: :is_none")]
    pub distribute_amount: Option<String>,
    pub Option: :is_none")]
    pub conversion_ratio: Option<String>,
    pub Option: :is_none")]
    pub status: Option<String>,
}

#[derive(SimpleObject)]
pub struct SubscribeOnChainYieldsLockedProductResponse {
    pub Option: :is_none")]
    pub purchase_id: Option<i64>,
    pub Option: :is_none")]
    pub position_id: Option<String>,
    pub Option: :is_none")]
    pub amount: Option<String>,
    pub Option: :is_none")]
    pub success: Option<bool>,
}

#[derive(SimpleObject)]
pub struct GetBnsolRateHistoryResponseRowsInner {
    pub Option: :is_none"
    )]
    pub annual_percentage_rate: Option<String>,
    pub Option: :is_none")]
    pub exchange_rate: Option<String>,
    pub Option: :is_none")]
    pub boost_rewards: Option<Vec<models::GetBnsolRateHistoryResponseRowsInnerBoostRewardsInner>>,
    pub Option: :is_none")]
    pub time: Option<i64>,
}

#[derive(SimpleObject)]
pub struct EthStakingAccountResponseHoldings {
    pub Option: :is_none")]
    pub wbeth_amount: Option<String>,
    pub Option: :is_none")]
    pub beth_amount: Option<String>,
}

#[derive(SimpleObject)]
pub struct GetBnsolRewardsHistoryResponse {
    pub Option: :is_none")]
    pub est_rewards_in_sol: Option<String>,
    pub Option: :is_none")]
    pub rows: Option<Vec<models::GetBnsolRewardsHistoryResponseRowsInner>>,
    pub Option: :is_none")]
    pub total: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCurrentEthStakingQuotaResponse {
    pub Option: :is_none"
    )]
    pub left_staking_personal_quota: Option<String>,
    pub Option: :is_none"
    )]
    pub left_redemption_personal_quota: Option<String>,
    pub Option: :is_none")]
    pub min_stake_amount: Option<String>,
    pub Option: :is_none")]
    pub min_redeem_amount: Option<String>,
    pub Option: :is_none")]
    pub redeem_period: Option<i64>,
    pub Option: :is_none")]
    pub stakeable: Option<bool>,
    pub Option: :is_none")]
    pub redeemable: Option<bool>,
    pub Option: :is_none")]
    pub commission_fee: Option<String>,
    pub Option: :is_none")]
    pub calculating: Option<bool>,
}

#[derive(SimpleObject)]
pub struct EthStakingApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct EthStakingAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetCurrentEthStakingQuotaParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetEthRedemptionHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetEthStakingHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethRateHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethRewardsHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethUnwrapHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetWbethWrapHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemEthParams {
    pub amount: rust_decimal::Decimal,
    pub asset: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeEthStakingParams {
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct WrapBethParams {
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockEthStakingApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct SolStakingApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct ClaimBoostRewardsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBnsolRateHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBnsolRewardsHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetBoostRewardsHistoryParams {
    pub type: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSolRedemptionHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSolStakingHistoryParams {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSolStakingQuotaDetailsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetUnclaimedRewardsParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemSolParams {
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SolStakingAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeSolStakingParams {
    pub amount: rust_decimal::Decimal,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockSolStakingApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct SoftStakingApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetSoftStakingProductListParams {
    pub asset: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetSoftStakingRewardsHistoryParams {
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetSoftStakingParams {
    pub soft_staking: bool,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockSoftStakingApiClient {
    pub force_error: bool,
}

#[derive(SimpleObject)]
pub struct OnChainYieldsApiClient {
    pub configuration: ConfigurationRestApi,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedPersonalLeftQuotaParams {
    pub project_id: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductListParams {
    pub asset: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedProductPositionParams {
    pub asset: Option<String>,
    pub position_id: Option<i64>,
    pub project_id: Option<String>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedRedemptionRecordParams {
    pub position_id: Option<i64>,
    pub redeem_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedRewardsHistoryParams {
    pub position_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedSubscriptionPreviewParams {
    pub project_id: String,
    pub amount: rust_decimal::Decimal,
    pub auto_subscribe: Option<bool>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct GetOnChainYieldsLockedSubscriptionRecordParams {
    pub purchase_id: Option<String>,
    pub client_id: Option<String>,
    pub asset: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub Default: 1
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub current: Option<i64>,
    pub Default: 10,
    pub Max: 100
    ///
    /// This field is **optional.
    #[builder(setter(into),
    pub size: Option<i64>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct OnChainYieldsAccountParams {
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct RedeemOnChainYieldsLockedProductParams {
    pub position_id: String,
    pub channel_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetOnChainYieldsLockedAutoSubscribeParams {
    pub position_id: String,
    pub auto_subscribe: bool,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SetOnChainYieldsLockedProductRedeemOptionParams {
    pub position_id: String,
    pub redeem_to: String,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct SubscribeOnChainYieldsLockedProductParams {
    pub project_id: String,
    pub amount: rust_decimal::Decimal,
    pub auto_subscribe: Option<bool>,
    pub source_account: Option<String>,
    pub redeem_to: Option<String>,
    pub channel_id: Option<String>,
    pub client_id: Option<String>,
    pub recv_window: Option<i64>,
}

#[derive(SimpleObject)]
pub struct MockOnChainYieldsApiClient {
    pub force_error: bool,
}