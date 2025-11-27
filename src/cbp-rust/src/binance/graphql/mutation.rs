// src/graphql/mutation.rs
use async_graphql::{Context, FieldResult, Object};
use std::sync::Arc;

use crate::binance_client::BinanceClient;
pub mod models {
    pub use binance::models::*;
}

pub struct Mutation;

#[Object]
impl Mutation {
    #[graphql(name = "sapiV1LendingAutoInvestOneOff")]
    async fn sapi_v1_lending_auto_invest_one_off(
        &self,
        ctx: &Context<'_>,
        source_type: String,
        subscription_amount: f64,
        source_asset: String,
        timestamp: i64,
        signature: String,
        request_id: Option<String>,
        flexible_allowed_to_use: Option<bool>,
        plan_id: Option<i64>,
        index_id: Option<i64>,
        details: Option<Vec<models::SapiV1LendingAutoInvestOneOffPostDetailsParameterInner>>,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1LendingAutoInvestOneOffPost200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_one_off_post(
            &client.config(),
            &source_type,
            subscription_amount,
            &source_asset,
            timestamp,
            &signature,
            request_id.as_deref(),
            flexible_allowed_to_use,
            plan_id,
            index_id,
            details.as_ref(),
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestPlanAdd")]
    async fn sapi_v1_lending_auto_invest_plan_add(
        &self,
        ctx: &Context<'_>,
        source_type: String,
        plan_type: String,
        subscription_amount: f64,
        subscription_cycle: String,
        subscription_start_time: i64,
        source_asset: String,
        details: Vec<models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner>,
        timestamp: i64,
        signature: String,
        request_id: Option<String>,
        index_id: Option<i64>,
        subscription_start_day: Option<i32>,
        subscription_start_weekday: Option<String>,
        flexible_allowed_to_use: Option<bool>,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1LendingAutoInvestPlanAddPost200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_plan_add_post(
            &client.config(),
            &source_type,
            &plan_type,
            subscription_amount,
            &subscription_cycle,
            subscription_start_time,
            &source_asset,
            &details,
            timestamp,
            &signature,
            request_id.as_deref(),
            index_id,
            subscription_start_day,
            subscription_start_weekday.as_deref(),
            flexible_allowed_to_use,
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestPlanEdit")]
    async fn sapi_v1_lending_auto_invest_plan_edit(
        &self,
        ctx: &Context<'_>,
        plan_id: i64,
        subscription_amount: f64,
        subscription_cycle: String,
        subscription_start_time: i64,
        source_asset: String,
        timestamp: i64,
        signature: String,
        subscription_start_day: Option<i32>,
        subscription_start_weekday: Option<String>,
        flexible_allowed_to_use: Option<bool>,
        details: Option<Vec<models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner>>,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1LendingAutoInvestPlanEditPost200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_plan_edit_post(
            &client.config(),
            plan_id,
            subscription_amount,
            &subscription_cycle,
            subscription_start_time,
            &source_asset,
            timestamp,
            &signature,
            subscription_start_day,
            subscription_start_weekday.as_deref(),
            flexible_allowed_to_use,
            details.as_ref(),
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AssetTransfer")]
    async fn sapi_v1_asset_transfer(
        &self,
        ctx: &Context<'_>,
        r#type: String,
        asset: String,
        amount: f64,
        timestamp: i64,
        signature: String,
        from_symbol: Option<String>,
        to_symbol: Option<String>,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1AssetTransferPost200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_asset_transfer_post(
            &client.config(),
            &r#type,
            &asset,
            amount,
            timestamp,
            &signature,
            from_symbol.as_deref(),
            to_symbol.as_deref(),
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1CapitalWithdrawApply")]
    async fn sapi_v1_capital_withdraw_apply(
        &self,
        ctx: &Context<'_>,
        coin: String,
        address: String,
        amount: f64,
        timestamp: i64,
        signature: String,
        withdraw_order_id: Option<String>,
        network: Option<String>,
        address_tag: Option<String>,
        transaction_fee_flag: Option<bool>,
        name: Option<String>,
        wallet_type: Option<i32>,
        recv_window: Option<i64>,
    ) -> FieldResult<models::SapiV1CapitalWithdrawApplyPost200Response> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_capital_withdraw_apply_post(
            &client.config(),
            &coin,
            &address,
            amount,
            timestamp,
            &signature,
            withdraw_order_id.as_deref(),
            network.as_deref(),
            address_tag.as_deref(),
            transaction_fee_flag,
            name.as_deref(),
            wallet_type,
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV3AssetGetUserAsset")]
    async fn sapi_v3_asset_get_user_asset(
        &self,
        ctx: &Context<'_>,
        timestamp: i64,
        signature: String,
        asset: Option<String>,
        need_btc_valuation: Option<String>,
        recv_window: Option<i64>,
    ) -> FieldResult<Vec<models::SapiV3AssetGetUserAssetPost200ResponseInner>> {
        let client = ctx.data_unchecked::<Arc<BinanceClient>>();
        binance::wallet_api::sapi_v3_asset_get_user_asset_post(
            &client.config(),
            timestamp,
            &signature,
            asset.as_deref(),
            need_btc_valuation.as_deref(),
            recv_window,
        )
        .await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}