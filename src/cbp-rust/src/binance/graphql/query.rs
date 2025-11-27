// src/graphql/query.rs
use async_graphql::{Context, FieldResult, Object};
use std::sync::Arc;

use crate::binance_client::BinanceClient;

// Import all generated models (adjust path if needed)
pub mod models {
    pub use binance::models::*;
}

pub struct Query;

#[Object]
impl Query {
    #[graphql(name = "sapiV1LendingAutoInvestAllAsset")]
    async fn sapi_v1_lending_auto_invest_all_asset(
        &self,
        ctx: &Context<'_>,
        timestamp: i64,
        signature: String,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1LendingAutoInvestAllAssetGet200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_all_asset_get(
            &client.config(),
            timestamp,
            &signature,
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestHistoryList")]
    async fn sapi_v1_lending_auto_invest_history_list(
        &self,
        ctx: &Context<'_>,
        timestamp: i64,
        signature: String,
        plan_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        target_asset: Option<String>,
        plan_type: Option<String>,
        size: Option<i32>,
        current: Option<i32>,
        recv_window: Option<i64>,
    ) -> FieldResult<Vec<models::SapiV1LendingAutoInvestHistoryListGet200ResponseInner>> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_history_list_get(
            &client.config(),
            timestamp,
            &signature,
            plan_id,
            start_time,
            end_time,
            target_asset.as_deref(),
            plan_type.as_deref(),
            size,
            current,
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestIndexInfo")]
    async fn sapi_v1_lending_auto_invest_index_info(
        &self,
        ctx: &Context<'_>,
        index_id: i64,
        timestamp: i64,
        signature: String,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1LendingAutoInvestIndexInfoGet200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_index_info_get(
            &client.config(),
            index_id,
            timestamp,
            &signature,
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestIndexUserSummary")]
    async fn sapi_v1_lending_auto_invest_index_user_summary(
        &self,
        ctx: &Context<'_>,
        index_id: i64,
        timestamp: i64,
        signature: String,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1LendingAutoInvestIndexUserSummaryGet200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_index_user_summary_get(
            &client.config(),
            index_id,
            timestamp,
            &signature,
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestOneOffStatus")]
    async fn sapi_v1_lending_auto_invest_one_off_status(
        &self,
        ctx: &Context<'_>,
        transaction_id: i64,
        timestamp: i64,
        signature: String,
        request_id: Option<String>,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1LendingAutoInvestOneOffStatusGet200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_one_off_status_get(
            &client.config(),
            transaction_id,
            timestamp,
            &signature,
            request_id.as_deref(),
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1CapitalDepositAddressList")]
    async fn sapi_v1_capital_deposit_address_list(
        &self,
        ctx: &Context<'_>,
        coin: String,
        timestamp: i64,
        signature: String,
        network: Option<String>,
        recv_window: Option<i64>,
    ) -> FieldResult<Vec<models::SapiV1CapitalDepositAddressListGet200ResponseInner>> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_capital_deposit_address_list_get(
            &client.config(),
            &coin,
            timestamp,
            &signature,
            network.as_deref(),
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1CapitalDepositHisrec")]
    async fn sapi_v1_capital_deposit_hisrec(
        &self,
        ctx: &Context<'_>,
        timestamp: i64,
        signature: String,
        coin: Option<String>,
        status: Option<i32>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        offset: Option<i32>,
        limit: Option<i32>,
        recv_window: Option<i64>,
    ) -> FieldResult<Vec<models::SapiV1CapitalDepositHisrecGet200ResponseInner>> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_capital_deposit_hisrec_get(
            &client.config(),
            timestamp,
            &signature,
            coin.as_deref(),
            status,
            start_time,
            end_time,
            offset,
            limit,
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SystemStatus")]
    async fn sapi_v1_system_status(&self, ctx: &Context<'_>) -> FieldResult<models::SapiV1SystemStatusGet200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_system_status_get(&client.config())
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}