use async_graphql::{Context, FieldResult, Object};

pub struct Mutation;

#[Object]
impl Mutation {
    #[graphql(name = "sapiV1LendingAutoInvestOneOff")]
    async fn sapiV1LendingAutoInvestOneOff(
        &self,
        ctx: &Context<'_>, source_type: str, subscription_amount: f32, source_asset: str, timestamp: i64, signature: str, request_id: Option<str>, flexible_allowed_to_use: Option<bool>, plan_id: Option<i64>, index_id: Option<i64>, details: Option<Vec<models::SapiV1LendingAutoInvestOneOffPostDetailsParameterInner>>, recv_window: Option<i64>) -> Result<models::SapiV1LendingAutoInvestOneOffPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p.to_string())).collect: :<Vec<(std::string::String, std: :string::String)>>()), param_value.into_iter().map(|p| p.to_string()).collect: :<Vec<String>>().join(", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LendingAutoInvestOneOffResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_one_off_post(
            &client.config(),
            source_type,
            subscription_amount,
            source_asset,
            timestamp,
            signature,
            request_id.unwrap_or_default(),
            flexible_allowed_to_use.unwrap_or_default(),
            plan_id.unwrap_or_default(),
            index_id.unwrap_or_default(),
            details.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p.to_string())).collect,
            std,
            param_value.into_iter().map(|p| p.to_string()).collect,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestPlanAdd")]
    async fn sapiV1LendingAutoInvestPlanAdd(
        &self,
        ctx: &Context<'_>, source_type: str, plan_type: str, subscription_amount: f32, subscription_cycle: str, subscription_start_time: i32, source_asset: str, details: Vec<models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner>, timestamp: i64, signature: str, request_id: Option<str>, index_id: Option<i64>, subscription_start_day: Option<i32>, subscription_start_weekday: Option<str>, flexible_allowed_to_use: Option<bool>, recv_window: Option<i64>) -> Result<models::SapiV1LendingAutoInvestPlanAddPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("subscriptionStartTime", p.to_string())).collect: :<Vec<(std::string::String, std: :string::String)>>()), p_query_details.into_iter().map(|p| p.to_string()).collect: :<Vec<String>>().join(", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LendingAutoInvestPlanAddResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_plan_add_post(
            &client.config(),
            source_type,
            plan_type,
            subscription_amount,
            subscription_cycle,
            subscription_start_time,
            source_asset,
            details,
            timestamp,
            signature,
            request_id.unwrap_or_default(),
            index_id.unwrap_or_default(),
            subscription_start_day.unwrap_or_default(),
            subscription_start_weekday.unwrap_or_default(),
            flexible_allowed_to_use.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p.to_string())).collect,
            std,
            p_query_details.into_iter().map(|p| p.to_string()).collect,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestPlanEdit")]
    async fn sapiV1LendingAutoInvestPlanEdit(
        &self,
        ctx: &Context<'_>, plan_id: i32, subscription_amount: f32, subscription_cycle: str, subscription_start_time: i32, source_asset: str, timestamp: i64, signature: str, subscription_start_day: Option<i32>, subscription_start_weekday: Option<str>, flexible_allowed_to_use: Option<bool>, details: Option<Vec<models::SapiV1LendingAutoInvestPlanAddPostDetailsParameterInner>>, recv_window: Option<i64>) -> Result<models::SapiV1LendingAutoInvestPlanAddPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("subscriptionStartTime", p.to_string())).collect: :<Vec<(std::string::String, std: :string::String)>>()), param_value.into_iter().map(|p| p.to_string()).collect: :<Vec<String>>().join(", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LendingAutoInvestPlanEditResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_plan_edit_post(
            &client.config(),
            plan_id,
            subscription_amount,
            subscription_cycle,
            subscription_start_time,
            source_asset,
            timestamp,
            signature,
            subscription_start_day.unwrap_or_default(),
            subscription_start_weekday.unwrap_or_default(),
            flexible_allowed_to_use.unwrap_or_default(),
            details.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p.to_string())).collect,
            std,
            param_value.into_iter().map(|p| p.to_string()).collect,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestPlanEditStatus")]
    async fn sapiV1LendingAutoInvestPlanEditStatus(
        &self,
        ctx: &Context<'_>, plan_id: i32, status: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1LendingAutoInvestPlanEditStatusPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LendingAutoInvestPlanEditStatusResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_plan_edit_status_post(
            &client.config(),
            plan_id,
            status,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingAutoInvestRedeem")]
    async fn sapiV1LendingAutoInvestRedeem(
        &self,
        ctx: &Context<'_>, index_id: i64, redemption_percentage: i32, timestamp: i64, signature: str, request_id: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1LendingAutoInvestRedeemPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LendingAutoInvestRedeemResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::auto_invest_api::sapi_v1_lending_auto_invest_redeem_post(
            &client.config(),
            index_id,
            redemption_percentage,
            timestamp,
            signature,
            request_id.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1BlvtRedeem")]
    async fn sapiV1BlvtRedeem(
        &self,
        ctx: &Context<'_>, token_name: str, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1BlvtRedeemPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<BlvtRedeemResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::blvt_api::sapi_v1_blvt_redeem_post(
            &client.config(),
            token_name,
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1BlvtSubscribe")]
    async fn sapiV1BlvtSubscribe(
        &self,
        ctx: &Context<'_>, token_name: str, cost: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1BlvtSubscribePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<BlvtSubscribeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::blvt_api::sapi_v1_blvt_subscribe_post(
            &client.config(),
            token_name,
            cost,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1ConvertAcceptQuote")]
    async fn sapiV1ConvertAcceptQuote(
        &self,
        ctx: &Context<'_>, quote_id: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1ConvertAcceptQuotePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ConvertAcceptQuoteResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::convert_api::sapi_v1_convert_accept_quote_post(
            &client.config(),
            quote_id,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1ConvertGetQuote")]
    async fn sapiV1ConvertGetQuote(
        &self,
        ctx: &Context<'_>, from_asset: str, to_asset: str, timestamp: i64, signature: str, from_amount: Option<f32>, to_amount: Option<f32>, valid_time: Option<str>, wallet_type: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1ConvertGetQuotePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ConvertQuoteResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::convert_api::sapi_v1_convert_get_quote_post(
            &client.config(),
            from_asset,
            to_asset,
            timestamp,
            signature,
            from_amount.unwrap_or_default(),
            to_amount.unwrap_or_default(),
            valid_time.unwrap_or_default(),
            wallet_type.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1ConvertLimitCancelOrder")]
    async fn sapiV1ConvertLimitCancelOrder(
        &self,
        ctx: &Context<'_>, order_id: i64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1ConvertLimitCancelOrderPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ConvertLimitCancelOrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::convert_api::sapi_v1_convert_limit_cancel_order_post(
            &client.config(),
            order_id,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1ConvertLimitPlaceOrder")]
    async fn sapiV1ConvertLimitPlaceOrder(
        &self,
        ctx: &Context<'_>, base_asset: str, quote_asset: str, limit_price: f64, side: str, timestamp: i64, signature: str, base_amount: Option<f64>, quote_amount: Option<f64>, wallet_type: Option<str>, expired_type: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1ConvertLimitPlaceOrderPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_expired_type { req_builder = req_builder.query(&[("expiredType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ConvertLimitPlaceOrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::convert_api::sapi_v1_convert_limit_place_order_post(
            &client.config(),
            base_asset,
            quote_asset,
            limit_price,
            side,
            timestamp,
            signature,
            base_amount.unwrap_or_default(),
            quote_amount.unwrap_or_default(),
            wallet_type.unwrap_or_default(),
            expired_type.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LoanAdjustLtv")]
    async fn sapiV1LoanAdjustLtv(
        &self,
        ctx: &Context<'_>, order_id: i64, amount: f64, direction: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1LoanAdjustLtvPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanAdjustLtvResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::crypto_loans_api::sapi_v1_loan_adjust_ltv_post(
            &client.config(),
            order_id,
            amount,
            direction,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LoanBorrow")]
    async fn sapiV1LoanBorrow(
        &self,
        ctx: &Context<'_>, loan_coin: str, collateral_coin: str, loan_term: i32, timestamp: i64, signature: str, loan_amount: Option<f32>, collateral_amount: Option<f32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanBorrowPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanBorrowResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::crypto_loans_api::sapi_v1_loan_borrow_post(
            &client.config(),
            loan_coin,
            collateral_coin,
            loan_term,
            timestamp,
            signature,
            loan_amount.unwrap_or_default(),
            collateral_amount.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LoanCustomizeMarginCall")]
    async fn sapiV1LoanCustomizeMarginCall(
        &self,
        ctx: &Context<'_>, margin_call: f32, timestamp: i64, signature: str, order_id: Option<i64>, collateral_coin: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1LoanCustomizeMarginCallPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanCustomizeMarginCallResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::crypto_loans_api::sapi_v1_loan_customize_margin_call_post(
            &client.config(),
            margin_call,
            timestamp,
            signature,
            order_id.unwrap_or_default(),
            collateral_coin.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LoanRepay")]
    async fn sapiV1LoanRepay(
        &self,
        ctx: &Context<'_>, order_id: i64, amount: f64, timestamp: i64, signature: str, r#type: Option<i32>, collateral_return: Option<bool>, recv_window: Option<i64>) -> Result<models::SapiV1LoanRepayPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanRepayResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::crypto_loans_api::sapi_v1_loan_repay_post(
            &client.config(),
            order_id,
            amount,
            timestamp,
            signature,
            r#type.unwrap_or_default(),
            collateral_return.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV2LoanFlexibleAdjustLtv")]
    async fn sapiV2LoanFlexibleAdjustLtv(
        &self,
        ctx: &Context<'_>, adjustment_amount: f32, direction: str, timestamp: i64, signature: str, loan_coin: Option<str>, collateral_coin: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV2LoanFlexibleAdjustLtvPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanFlexibleAdjustLtvResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::crypto_loans_api::sapi_v2_loan_flexible_adjust_ltv_post(
            &client.config(),
            adjustment_amount,
            direction,
            timestamp,
            signature,
            loan_coin.unwrap_or_default(),
            collateral_coin.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV2LoanFlexibleBorrow")]
    async fn sapiV2LoanFlexibleBorrow(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, loan_coin: Option<str>, loan_amount: Option<f32>, collateral_coin: Option<str>, collateral_amount: Option<f32>, recv_window: Option<i64>) -> Result<models::SapiV2LoanFlexibleBorrowPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanFlexibleBorrowResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::crypto_loans_api::sapi_v2_loan_flexible_borrow_post(
            &client.config(),
            timestamp,
            signature,
            loan_coin.unwrap_or_default(),
            loan_amount.unwrap_or_default(),
            collateral_coin.unwrap_or_default(),
            collateral_amount.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV2LoanFlexibleRepay")]
    async fn sapiV2LoanFlexibleRepay(
        &self,
        ctx: &Context<'_>, repay_amount: f32, timestamp: i64, signature: str, loan_coin: Option<str>, collateral_coin: Option<str>, collateral_return: Option<bool>, full_repayment: Option<bool>, recv_window: Option<i64>) -> Result<models::SapiV2LoanFlexibleRepayPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanFlexibleRepayResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::crypto_loans_api::sapi_v2_loan_flexible_repay_post(
            &client.config(),
            repay_amount,
            timestamp,
            signature,
            loan_coin.unwrap_or_default(),
            collateral_coin.unwrap_or_default(),
            collateral_return.unwrap_or_default(),
            full_repayment.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1DciProductAutoCompoundEditStatus")]
    async fn sapiV1DciProductAutoCompoundEditStatus(
        &self,
        ctx: &Context<'_>, position_id: i64, auto_compound_plan: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1DciProductAutoCompoundEditStatusPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<DciProductAutoCompoundEditStatusResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::dual_investment_api::sapi_v1_dci_product_auto_compound_edit_status_post(
            &client.config(),
            position_id,
            auto_compound_plan,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1DciProductSubscribe")]
    async fn sapiV1DciProductSubscribe(
        &self,
        ctx: &Context<'_>, id: str, order_id: str, deposit_amount: f64, auto_compound_plan: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1DciProductSubscribePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<DciProductSubscribeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::dual_investment_api::sapi_v1_dci_product_subscribe_post(
            &client.config(),
            id,
            order_id,
            deposit_amount,
            auto_compound_plan,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AlgoFuturesNewOrderTwap")]
    async fn sapiV1AlgoFuturesNewOrderTwap(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, quantity: f64, duration: i64, timestamp: i64, signature: str, position_side: Option<str>, client_algo_id: Option<str>, reduce_only: Option<bool>, limit_price: Option<f64>, recv_window: Option<i64>) -> Result<models::SapiV1AlgoFuturesNewOrderVpPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("quantity", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AlgoFuturesNewOrderTwapResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::futures_algo_api::sapi_v1_algo_futures_new_order_twap_post(
            &client.config(),
            symbol,
            side,
            quantity,
            duration,
            timestamp,
            signature,
            position_side.unwrap_or_default(),
            client_algo_id.unwrap_or_default(),
            reduce_only.unwrap_or_default(),
            limit_price.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AlgoFuturesNewOrderVp")]
    async fn sapiV1AlgoFuturesNewOrderVp(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, quantity: f64, urgency: str, timestamp: i64, signature: str, position_side: Option<str>, client_algo_id: Option<str>, reduce_only: Option<bool>, limit_price: Option<f64>, recv_window: Option<i64>) -> Result<models::SapiV1AlgoFuturesNewOrderVpPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("quantity", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AlgoFuturesNewOrderVpResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::futures_algo_api::sapi_v1_algo_futures_new_order_vp_post(
            &client.config(),
            symbol,
            side,
            quantity,
            urgency,
            timestamp,
            signature,
            position_side.unwrap_or_default(),
            client_algo_id.unwrap_or_default(),
            reduce_only.unwrap_or_default(),
            limit_price.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AlgoFuturesOrder")]
    async fn sapiV1AlgoFuturesOrder(
        &self,
        ctx: &Context<'_>, algo_id: i64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1AlgoFuturesOrderDelete200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AlgoFuturesOrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::futures_algo_api::sapi_v1_algo_futures_order_delete(
            &client.config(),
            algo_id,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1FuturesTransfer")]
    async fn sapiV1FuturesTransfer(
        &self,
        ctx: &Context<'_>, asset: str, amount: f64, r#type: i64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginBorrowRepayPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<FuturesTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::futures_api::sapi_v1_futures_transfer_post(
            &client.config(),
            asset,
            amount,
            r#type,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1GiftcardBuyCode")]
    async fn sapiV1GiftcardBuyCode(
        &self,
        ctx: &Context<'_>, base_token: str, face_token: str, base_token_amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1GiftcardCreateCodePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<GiftcardBuyCodeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::gift_card_api::sapi_v1_giftcard_buy_code_post(
            &client.config(),
            base_token,
            face_token,
            base_token_amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1GiftcardCreateCode")]
    async fn sapiV1GiftcardCreateCode(
        &self,
        ctx: &Context<'_>, token: str, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1GiftcardCreateCodePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<GiftcardCreateCodeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::gift_card_api::sapi_v1_giftcard_create_code_post(
            &client.config(),
            token,
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1GiftcardRedeemCode")]
    async fn sapiV1GiftcardRedeemCode(
        &self,
        ctx: &Context<'_>, code: str, timestamp: i64, signature: str, external_uid: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1GiftcardRedeemCodePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<GiftcardRedeemCodeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::gift_card_api::sapi_v1_giftcard_redeem_code_post(
            &client.config(),
            code,
            timestamp,
            signature,
            external_uid.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1UserDataStreamIsolated")]
    async fn sapiV1UserDataStreamIsolated(
        &self,
        ctx: &Context<'_>, listen_key: Option<str>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<UserDataStreamIsolatedResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::isolated_margin_stream_api::sapi_v1_user_data_stream_isolated_delete(
            &client.config(),
            listen_key.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1UserDataStreamIsolated")]
    async fn sapiV1UserDataStreamIsolated(
        &self,
        ctx: &Context<'_>, ) -> Result<models: :SapiV1UserDataStreamIsolatedPost200Response, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, uri_str);  if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<UserDataStreamIsolatedResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::isolated_margin_stream_api::sapi_v1_user_data_stream_isolated_post(
            &client.config(),
            ) -> Result<models,
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            uri_str);  if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1UserDataStreamIsolated")]
    async fn sapiV1UserDataStreamIsolated(
        &self,
        ctx: &Context<'_>, listen_key: Option<str>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::PUT, param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<UserDataStreamIsolatedPutResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::isolated_margin_stream_api::sapi_v1_user_data_stream_isolated_put(
            &client.config(),
            listen_key.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1BnbBurn")]
    async fn sapiV1BnbBurn(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, spot_bnb_burn: Option<str>, interest_bnb_burn: Option<str>, recv_window: Option<i64>) -> Result<models::BnbBurnStatu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_interest_bnb_burn { req_builder = req_builder.query(&[("interestBNBBurn", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<BnbBurnResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_bnb_burn_post(
            &client.config(),
            timestamp,
            signature,
            spot_bnb_burn.unwrap_or_default(),
            interest_bnb_burn.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginBorrowRepay")]
    async fn sapiV1MarginBorrowRepay(
        &self,
        ctx: &Context<'_>, asset: str, is_isolated: str, symbol: str, amount: f64, r#type: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginBorrowRepayPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginBorrowRepayResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_borrow_repay_post(
            &client.config(),
            asset,
            is_isolated,
            symbol,
            amount,
            r#type,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginIsolatedAccount")]
    async fn sapiV1MarginIsolatedAccount(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginIsolatedAccountPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginIsolatedAccountResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_isolated_account_delete(
            &client.config(),
            symbol,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginIsolatedAccount")]
    async fn sapiV1MarginIsolatedAccount(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginIsolatedAccountPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginIsolatedAccountResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_isolated_account_post(
            &client.config(),
            symbol,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginManualLiquidation")]
    async fn sapiV1MarginManualLiquidation(
        &self,
        ctx: &Context<'_>, r#type: str, timestamp: i64, signature: str, symbol: Option<str>) -> Result<Vec<models::SapiV1MarginExchangeSmallLiabilityGet200ResponseInner>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginManualLiquidationResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_manual_liquidation_post(
            &client.config(),
            r#type,
            timestamp,
            signature,
            symbol.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginMaxLeverage")]
    async fn sapiV1MarginMaxLeverage(
        &self,
        ctx: &Context<'_>, max_leverage: i32, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginMaxLeveragePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginMaxLeverageResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_max_leverage_post(
            &client.config(),
            max_leverage,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginOpenOrders")]
    async fn sapiV1MarginOpenOrders(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, is_isolated: Option<str>, recv_window: Option<i64>) -> Result<Vec<models::SapiV1MarginOpenOrdersDelete200ResponseInner>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginOpenOrdersResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_open_orders_delete(
            &client.config(),
            symbol,
            timestamp,
            signature,
            is_isolated.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginOrder")]
    async fn sapiV1MarginOrder(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, is_isolated: Option<str>, order_id: Option<i64>, orig_client_order_id: Option<str>, new_client_order_id: Option<str>, recv_window: Option<i64>) -> Result<models::MarginOrde>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_order_id { req_builder = req_builder.query(&[("orderId", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginOrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_order_delete(
            &client.config(),
            symbol,
            timestamp,
            signature,
            is_isolated.unwrap_or_default(),
            order_id.unwrap_or_default(),
            orig_client_order_id.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginOrderList")]
    async fn sapiV1MarginOrderList(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, is_isolated: Option<str>, order_list_id: Option<i64>, list_client_order_id: Option<str>, new_client_order_id: Option<str>, recv_window: Option<i64>) -> Result<models::MarginOcoOrde>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_order_list_id { req_builder = req_builder.query(&[("orderListId", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginOrderListResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_order_list_delete(
            &client.config(),
            symbol,
            timestamp,
            signature,
            is_isolated.unwrap_or_default(),
            order_list_id.unwrap_or_default(),
            list_client_order_id.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginOrderOco")]
    async fn sapiV1MarginOrderOco(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, quantity: f64, price: f64, stop_price: f64, timestamp: i64, signature: str, is_isolated: Option<str>, list_client_order_id: Option<str>, limit_client_order_id: Option<str>, limit_iceberg_qty: Option<f64>, stop_client_order_id: Option<str>, stop_limit_price: Option<f64>, stop_iceberg_qty: Option<f64>, stop_limit_time_in_force: Option<str>, new_order_resp_type: Option<str>, side_effect_type: Option<str>, self_trade_prevention_mode: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1MarginOrderOcoPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_list_client_order_id { req_builder = req_builder.query(&[("listClientOrderId", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_new_order_resp_type { req_builder = req_builder.query(&[("newOrderRespType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_side_effect_type { req_builder = req_builder.query(&[("sideEffectType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginOrderOcoResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_order_oco_post(
            &client.config(),
            symbol,
            side,
            quantity,
            price,
            stop_price,
            timestamp,
            signature,
            is_isolated.unwrap_or_default(),
            list_client_order_id.unwrap_or_default(),
            limit_client_order_id.unwrap_or_default(),
            limit_iceberg_qty.unwrap_or_default(),
            stop_client_order_id.unwrap_or_default(),
            stop_limit_price.unwrap_or_default(),
            stop_iceberg_qty.unwrap_or_default(),
            stop_limit_time_in_force.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            side_effect_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginOrderOto")]
    async fn sapiV1MarginOrderOto(
        &self,
        ctx: &Context<'_>, symbol: str, working_type: str, working_side: str, working_price: f64, working_quantity: f64, working_iceberg_qty: f64, pending_type: str, pending_side: str, pending_quantity: f64, timestamp: i64, signature: str, is_isolated: Option<str>, list_client_order_id: Option<str>, new_order_resp_type: Option<str>, side_effect_type: Option<str>, self_trade_prevention_mode: Option<str>, auto_repay_at_cancel: Option<bool>, working_client_order_id: Option<str>, working_time_in_force: Option<str>, pending_client_order_id: Option<str>, pending_price: Option<f64>, pending_stop_price: Option<f64>, pending_trailing_delta: Option<f64>, pending_iceberg_qty: Option<f64>, pending_time_in_force: Option<str>) -> Result<models::SapiV1MarginOrderOtoPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_list_client_order_id { req_builder = req_builder.query(&[("listClientOrderId", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_side_effect_type { req_builder = req_builder.query(&[("sideEffectType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_auto_repay_at_cancel { req_builder = req_builder.query(&[("autoRepayAtCancel", serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("pendingType", serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("timestamp", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginOrderOtoResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_order_oto_post(
            &client.config(),
            symbol,
            working_type,
            working_side,
            working_price,
            working_quantity,
            working_iceberg_qty,
            pending_type,
            pending_side,
            pending_quantity,
            timestamp,
            signature,
            is_isolated.unwrap_or_default(),
            list_client_order_id.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            side_effect_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            auto_repay_at_cancel.unwrap_or_default(),
            working_client_order_id.unwrap_or_default(),
            working_time_in_force.unwrap_or_default(),
            pending_client_order_id.unwrap_or_default(),
            pending_price.unwrap_or_default(),
            pending_stop_price.unwrap_or_default(),
            pending_trailing_delta.unwrap_or_default(),
            pending_iceberg_qty.unwrap_or_default(),
            pending_time_in_force.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginOrderOtoco")]
    async fn sapiV1MarginOrderOtoco(
        &self,
        ctx: &Context<'_>, symbol: str, working_type: str, working_side: str, working_price: f64, working_quantity: f64, working_iceberg_qty: f64, pending_side: str, pending_quantity: f64, pending_above_type: str, timestamp: i64, signature: str, is_isolated: Option<str>, side_effect_type: Option<str>, auto_repay_at_cancel: Option<bool>, list_client_order_id: Option<str>, new_order_resp_type: Option<str>, self_trade_prevention_mode: Option<str>, working_client_order_id: Option<str>, working_time_in_force: Option<str>, pending_above_client_order_id: Option<str>, pending_above_price: Option<f64>, pending_above_stop_price: Option<f64>, pending_above_trailing_delta: Option<f64>, pending_above_iceberg_qty: Option<f64>, pending_above_time_in_force: Option<str>, pending_below_type: Option<str>, pending_below_client_order_id: Option<str>, pending_below_price: Option<f64>, pending_below_stop_price: Option<f64>, pending_below_trailing_delta: Option<f64>, pending_below_iceberg_qty: Option<f64>, pending_below_time_in_force: Option<str>) -> Result<models::SapiV1MarginOrderOtocoPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_side_effect_type { req_builder = req_builder.query(&[("sideEffectType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_auto_repay_at_cancel { req_builder = req_builder.query(&[("autoRepayAtCancel", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("workingType", serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("pendingSide", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_pending_below_type { req_builder = req_builder.query(&[("pendingBelowType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_pending_below_client_order_id { req_builder = req_builder.query(&[("pendingBelowClientOrderId", serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("timestamp", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginOrderOtocoResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_order_otoco_post(
            &client.config(),
            symbol,
            working_type,
            working_side,
            working_price,
            working_quantity,
            working_iceberg_qty,
            pending_side,
            pending_quantity,
            pending_above_type,
            timestamp,
            signature,
            is_isolated.unwrap_or_default(),
            side_effect_type.unwrap_or_default(),
            auto_repay_at_cancel.unwrap_or_default(),
            list_client_order_id.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            working_client_order_id.unwrap_or_default(),
            working_time_in_force.unwrap_or_default(),
            pending_above_client_order_id.unwrap_or_default(),
            pending_above_price.unwrap_or_default(),
            pending_above_stop_price.unwrap_or_default(),
            pending_above_trailing_delta.unwrap_or_default(),
            pending_above_iceberg_qty.unwrap_or_default(),
            pending_above_time_in_force.unwrap_or_default(),
            pending_below_type.unwrap_or_default(),
            pending_below_client_order_id.unwrap_or_default(),
            pending_below_price.unwrap_or_default(),
            pending_below_stop_price.unwrap_or_default(),
            pending_below_trailing_delta.unwrap_or_default(),
            pending_below_iceberg_qty.unwrap_or_default(),
            pending_below_time_in_force.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MarginOrder")]
    async fn sapiV1MarginOrder(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, r#type: str, quantity: f64, auto_repay_at_cancel: bool, timestamp: i64, signature: str, is_isolated: Option<str>, quote_order_qty: Option<f64>, price: Option<f64>, stop_price: Option<f64>, new_client_order_id: Option<str>, iceberg_qty: Option<f64>, new_order_resp_type: Option<str>, side_effect_type: Option<str>, time_in_force: Option<str>, self_trade_prevention_mode: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1MarginOrderPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("side", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_side_effect_type { req_builder = req_builder.query(&[("sideEffectType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_time_in_force { req_builder = req_builder.query(&[("timeInForce", serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("autoRepayAtCancel", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MarginOrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_api::sapi_v1_margin_order_post(
            &client.config(),
            symbol,
            side,
            r#type,
            quantity,
            auto_repay_at_cancel,
            timestamp,
            signature,
            is_isolated.unwrap_or_default(),
            quote_order_qty.unwrap_or_default(),
            price.unwrap_or_default(),
            stop_price.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            iceberg_qty.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            side_effect_type.unwrap_or_default(),
            time_in_force.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1UserDataStream")]
    async fn sapiV1UserDataStream(
        &self,
        ctx: &Context<'_>, listen_key: Option<str>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<UserDataStreamResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_stream_api::sapi_v1_user_data_stream_delete(
            &client.config(),
            listen_key.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1UserDataStream")]
    async fn sapiV1UserDataStream(
        &self,
        ctx: &Context<'_>, ) -> Result<models: :ApiV3UserDataStreamPost200Response, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, uri_str);  if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<UserDataStreamResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_stream_api::sapi_v1_user_data_stream_post(
            &client.config(),
            ) -> Result<models,
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            uri_str);  if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1UserDataStream")]
    async fn sapiV1UserDataStream(
        &self,
        ctx: &Context<'_>, listen_key: Option<str>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::PUT, param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<UserDataStreamPutResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::margin_stream_api::sapi_v1_user_data_stream_put(
            &client.config(),
            listen_key.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MiningHashTransferConfigCancel")]
    async fn sapiV1MiningHashTransferConfigCancel(
        &self,
        ctx: &Context<'_>, config_id: str, user_name: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MiningHashTransferConfigCancelPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MiningHashTransferConfigCancelResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::mining_api::sapi_v1_mining_hash_transfer_config_cancel_post(
            &client.config(),
            config_id,
            user_name,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1MiningHashTransferConfig")]
    async fn sapiV1MiningHashTransferConfig(
        &self,
        ctx: &Context<'_>, user_name: str, algo: str, to_pool_user: str, hash_rate: str, timestamp: i64, signature: str, start_date: Option<str>, end_date: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1MiningHashTransferConfigPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<MiningHashTransferConfigResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::mining_api::sapi_v1_mining_hash_transfer_config_post(
            &client.config(),
            user_name,
            algo,
            to_pool_user,
            hash_rate,
            timestamp,
            signature,
            start_date.unwrap_or_default(),
            end_date.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1PortfolioAssetCollection")]
    async fn sapiV1PortfolioAssetCollection(
        &self,
        ctx: &Context<'_>, asset: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<PortfolioAssetCollectionResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::portfolio_margin_api::sapi_v1_portfolio_asset_collection_post(
            &client.config(),
            asset,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1PortfolioAutoCollection")]
    async fn sapiV1PortfolioAutoCollection(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<PortfolioAutoCollectionResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::portfolio_margin_api::sapi_v1_portfolio_auto_collection_post(
            &client.config(),
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1PortfolioBnbTransfer")]
    async fn sapiV1PortfolioBnbTransfer(
        &self,
        ctx: &Context<'_>, transfer_side: str, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginBorrowRepayPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<PortfolioBnbTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::portfolio_margin_api::sapi_v1_portfolio_bnb_transfer_post(
            &client.config(),
            transfer_side,
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1PortfolioRepayFuturesNegativeBalance")]
    async fn sapiV1PortfolioRepayFuturesNegativeBalance(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<PortfolioRepayFuturesNegativeBalanceResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::portfolio_margin_api::sapi_v1_portfolio_repay_futures_negative_balance_post(
            &client.config(),
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1PortfolioRepayFuturesSwitch")]
    async fn sapiV1PortfolioRepayFuturesSwitch(
        &self,
        ctx: &Context<'_>, auto_repay: bool, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<PortfolioRepayFuturesSwitchResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::portfolio_margin_api::sapi_v1_portfolio_repay_futures_switch_post(
            &client.config(),
            auto_repay,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1PortfolioRepay")]
    async fn sapiV1PortfolioRepay(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, from: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioRepayPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<PortfolioRepayResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::portfolio_margin_api::sapi_v1_portfolio_repay_post(
            &client.config(),
            timestamp,
            signature,
            from.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingCustomizedFixedPurchase")]
    async fn sapiV1LendingCustomizedFixedPurchase(
        &self,
        ctx: &Context<'_>, project_id: str, lot: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1LendingCustomizedFixedPurchasePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LendingCustomizedFixedPurchaseResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::savings_api::sapi_v1_lending_customized_fixed_purchase_post(
            &client.config(),
            project_id,
            lot,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LendingPositionChanged")]
    async fn sapiV1LendingPositionChanged(
        &self,
        ctx: &Context<'_>, project_id: str, lot: str, timestamp: i64, signature: str, position_id: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1LendingPositionChangedPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LendingPositionChangedResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::savings_api::sapi_v1_lending_position_changed_post(
            &client.config(),
            project_id,
            lot,
            timestamp,
            signature,
            position_id.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SimpleEarnFlexibleRedeem")]
    async fn sapiV1SimpleEarnFlexibleRedeem(
        &self,
        ctx: &Context<'_>, product_id: str, timestamp: i64, signature: str, redeem_all: Option<bool>, amount: Option<f64>, dest_account: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1SimpleEarnFlexibleRedeemPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SimpleEarnFlexibleRedeemResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::simple_earn_api::sapi_v1_simple_earn_flexible_redeem_post(
            &client.config(),
            product_id,
            timestamp,
            signature,
            redeem_all.unwrap_or_default(),
            amount.unwrap_or_default(),
            dest_account.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SimpleEarnFlexibleSetAutoSubscribe")]
    async fn sapiV1SimpleEarnFlexibleSetAutoSubscribe(
        &self,
        ctx: &Context<'_>, product_id: str, auto_subscribe: bool, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginMaxLeveragePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SimpleEarnFlexibleSetAutoSubscribeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::simple_earn_api::sapi_v1_simple_earn_flexible_set_auto_subscribe_post(
            &client.config(),
            product_id,
            auto_subscribe,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SimpleEarnFlexibleSubscribe")]
    async fn sapiV1SimpleEarnFlexibleSubscribe(
        &self,
        ctx: &Context<'_>, product_id: str, amount: f64, timestamp: i64, signature: str, auto_subscribe: Option<bool>, source_account: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1SimpleEarnFlexibleSubscribePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SimpleEarnFlexibleSubscribeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::simple_earn_api::sapi_v1_simple_earn_flexible_subscribe_post(
            &client.config(),
            product_id,
            amount,
            timestamp,
            signature,
            auto_subscribe.unwrap_or_default(),
            source_account.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SimpleEarnLockedRedeem")]
    async fn sapiV1SimpleEarnLockedRedeem(
        &self,
        ctx: &Context<'_>, position_id: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SimpleEarnFlexibleRedeemPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SimpleEarnLockedRedeemResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::simple_earn_api::sapi_v1_simple_earn_locked_redeem_post(
            &client.config(),
            position_id,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SimpleEarnLockedSetAutoSubscribe")]
    async fn sapiV1SimpleEarnLockedSetAutoSubscribe(
        &self,
        ctx: &Context<'_>, position_id: str, auto_subscribe: bool, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1MarginMaxLeveragePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SimpleEarnLockedSetAutoSubscribeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::simple_earn_api::sapi_v1_simple_earn_locked_set_auto_subscribe_post(
            &client.config(),
            position_id,
            auto_subscribe,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SimpleEarnLockedSubscribe")]
    async fn sapiV1SimpleEarnLockedSubscribe(
        &self,
        ctx: &Context<'_>, project_id: str, amount: f64, timestamp: i64, signature: str, auto_subscribe: Option<bool>, source_account: Option<str>, redeem_to: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1SimpleEarnLockedSubscribePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SimpleEarnLockedSubscribeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::simple_earn_api::sapi_v1_simple_earn_locked_subscribe_post(
            &client.config(),
            project_id,
            amount,
            timestamp,
            signature,
            auto_subscribe.unwrap_or_default(),
            source_account.unwrap_or_default(),
            redeem_to.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AlgoSpotNewOrderTwap")]
    async fn sapiV1AlgoSpotNewOrderTwap(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, quantity: f64, duration: i32, timestamp: i64, signature: str, client_algo_id: Option<str>, limit_price: Option<f32>, recv_window: Option<i64>) -> Result<models::SapiV1AlgoSpotNewOrderTwapPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AlgoSpotNewOrderTwapResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::spot_algo_api::sapi_v1_algo_spot_new_order_twap_post(
            &client.config(),
            symbol,
            side,
            quantity,
            duration,
            timestamp,
            signature,
            client_algo_id.unwrap_or_default(),
            limit_price.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AlgoSpotOrder")]
    async fn sapiV1AlgoSpotOrder(
        &self,
        ctx: &Context<'_>, algo_id: i64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1AlgoSpotOrderDelete200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AlgoSpotOrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::spot_algo_api::sapi_v1_algo_spot_order_delete(
            &client.config(),
            algo_id,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1EthStakingEthRedeem")]
    async fn sapiV1EthStakingEthRedeem(
        &self,
        ctx: &Context<'_>, amount: f64, timestamp: i64, signature: str, asset: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1EthStakingEthRedeemPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<EthStakingEthRedeemResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::staking_api::sapi_v1_eth_staking_eth_redeem_post(
            &client.config(),
            amount,
            timestamp,
            signature,
            asset.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1EthStakingWbethWrap")]
    async fn sapiV1EthStakingWbethWrap(
        &self,
        ctx: &Context<'_>, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1EthStakingWbethWrapPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<EthStakingWbethWrapResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::staking_api::sapi_v1_eth_staking_wbeth_wrap_post(
            &client.config(),
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV2EthStakingEthStake")]
    async fn sapiV2EthStakingEthStake(
        &self,
        ctx: &Context<'_>, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV2EthStakingEthStakePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<EthStakingEthStakeResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::staking_api::sapi_v2_eth_staking_eth_stake_post(
            &client.config(),
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3UserDataStream")]
    async fn apiV3UserDataStream(
        &self,
        ctx: &Context<'_>, listen_key: Option<str>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3UserDataStreamResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::stream_api::api_v3_user_data_stream_delete(
            &client.config(),
            listen_key.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3UserDataStream")]
    async fn apiV3UserDataStream(
        &self,
        ctx: &Context<'_>, ) -> Result<models: :ApiV3UserDataStreamPost200Response, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, uri_str);  if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3UserDataStreamResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::stream_api::api_v3_user_data_stream_post(
            &client.config(),
            ) -> Result<models,
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            uri_str);  if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3UserDataStream")]
    async fn apiV3UserDataStream(
        &self,
        ctx: &Context<'_>, listen_key: Option<str>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::PUT, param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3UserDataStreamPutResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::stream_api::api_v3_user_data_stream_put(
            &client.config(),
            listen_key.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            param_value.to_string())]); } if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1ManagedSubaccountDeposit")]
    async fn sapiV1ManagedSubaccountDeposit(
        &self,
        ctx: &Context<'_>, to_email: str, asset: str, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1ManagedSubaccountDepositPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ManagedSubaccountDepositResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_managed_subaccount_deposit_post(
            &client.config(),
            to_email,
            asset,
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1ManagedSubaccountWithdraw")]
    async fn sapiV1ManagedSubaccountWithdraw(
        &self,
        ctx: &Context<'_>, from_email: str, asset: str, amount: f64, timestamp: i64, signature: str, transfer_date: Option<i64>, recv_window: Option<i64>) -> Result<models::SapiV1ManagedSubaccountDepositPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ManagedSubaccountWithdrawResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_managed_subaccount_withdraw_post(
            &client.config(),
            from_email,
            asset,
            amount,
            timestamp,
            signature,
            transfer_date.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountBlvtEnable")]
    async fn sapiV1SubAccountBlvtEnable(
        &self,
        ctx: &Context<'_>, email: str, enable_blvt: bool, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountBlvtEnablePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountBlvtEnableResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_blvt_enable_post(
            &client.config(),
            email,
            enable_blvt,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountEoptionsEnable")]
    async fn sapiV1SubAccountEoptionsEnable(
        &self,
        ctx: &Context<'_>, email: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountEoptionsEnablePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountEoptionsEnableResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_eoptions_enable_post(
            &client.config(),
            email,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountFuturesEnable")]
    async fn sapiV1SubAccountFuturesEnable(
        &self,
        ctx: &Context<'_>, email: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountFuturesEnablePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountFuturesEnableResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_futures_enable_post(
            &client.config(),
            email,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountFuturesInternalTransfer")]
    async fn sapiV1SubAccountFuturesInternalTransfer(
        &self,
        ctx: &Context<'_>, from_email: str, to_email: str, futures_type: i32, asset: str, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountFuturesInternalTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountFuturesInternalTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_futures_internal_transfer_post(
            &client.config(),
            from_email,
            to_email,
            futures_type,
            asset,
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountFuturesTransfer")]
    async fn sapiV1SubAccountFuturesTransfer(
        &self,
        ctx: &Context<'_>, email: str, asset: str, amount: f64, r#type: i32, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountFuturesTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountFuturesTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_futures_transfer_post(
            &client.config(),
            email,
            asset,
            amount,
            r#type,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountMarginEnable")]
    async fn sapiV1SubAccountMarginEnable(
        &self,
        ctx: &Context<'_>, email: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountMarginEnablePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountMarginEnableResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_margin_enable_post(
            &client.config(),
            email,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountMarginTransfer")]
    async fn sapiV1SubAccountMarginTransfer(
        &self,
        ctx: &Context<'_>, email: str, asset: str, amount: f64, r#type: i32, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountFuturesTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountMarginTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_margin_transfer_post(
            &client.config(),
            email,
            asset,
            amount,
            r#type,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountSubAccountApiIpRestrictionIpList")]
    async fn sapiV1SubAccountSubAccountApiIpRestrictionIpList(
        &self,
        ctx: &Context<'_>, email: str, sub_account_api_key: str, timestamp: i64, signature: str, ip_address: Option<str>, third_party_name: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountSubAccountApiIpRestrictionIpListDelete200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountSubAccountApiIpRestrictionIpListResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_sub_account_api_ip_restriction_ip_list_delete(
            &client.config(),
            email,
            sub_account_api_key,
            timestamp,
            signature,
            ip_address.unwrap_or_default(),
            third_party_name.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountTransferSubToMaster")]
    async fn sapiV1SubAccountTransferSubToMaster(
        &self,
        ctx: &Context<'_>, asset: str, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountFuturesTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountTransferSubToMasterResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_transfer_sub_to_master_post(
            &client.config(),
            asset,
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountTransferSubToSub")]
    async fn sapiV1SubAccountTransferSubToSub(
        &self,
        ctx: &Context<'_>, to_email: str, asset: str, amount: f64, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountFuturesTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountTransferSubToSubResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_transfer_sub_to_sub_post(
            &client.config(),
            to_email,
            asset,
            amount,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountUniversalTransfer")]
    async fn sapiV1SubAccountUniversalTransfer(
        &self,
        ctx: &Context<'_>, from_account_type: str, to_account_type: str, asset: str, amount: f64, timestamp: i64, signature: str, from_email: Option<str>, to_email: Option<str>, client_tran_id: Option<str>, symbol: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountUniversalTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountUniversalTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_universal_transfer_post(
            &client.config(),
            from_account_type,
            to_account_type,
            asset,
            amount,
            timestamp,
            signature,
            from_email.unwrap_or_default(),
            to_email.unwrap_or_default(),
            client_tran_id.unwrap_or_default(),
            symbol.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1SubAccountVirtualSubAccount")]
    async fn sapiV1SubAccountVirtualSubAccount(
        &self,
        ctx: &Context<'_>, sub_account_string: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1SubAccountVirtualSubAccountPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountVirtualSubAccountResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v1_sub_account_virtual_sub_account_post(
            &client.config(),
            sub_account_string,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV2SubAccountSubAccountApiIpRestriction")]
    async fn sapiV2SubAccountSubAccountApiIpRestriction(
        &self,
        ctx: &Context<'_>, email: str, sub_account_api_key: str, status: str, timestamp: i64, signature: str, third_party_name: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV2SubAccountSubAccountApiIpRestrictionPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<SubAccountSubAccountApiIpRestrictionResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::sub_account_api::sapi_v2_sub_account_sub_account_api_ip_restriction_post(
            &client.config(),
            email,
            sub_account_api_key,
            status,
            timestamp,
            signature,
            third_party_name.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3OpenOrders")]
    async fn apiV3OpenOrders(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<Vec<models::ApiV3OpenOrdersDelete200ResponseInner>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OpenOrdersResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_open_orders_delete(
            &client.config(),
            symbol,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3OrderCancelReplace")]
    async fn apiV3OrderCancelReplace(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, r#type: str, cancel_replace_mode: str, timestamp: i64, signature: str, cancel_restrictions: Option<str>, time_in_force: Option<str>, quantity: Option<f64>, quote_order_qty: Option<f64>, price: Option<f64>, cancel_new_client_order_id: Option<str>, cancel_orig_client_order_id: Option<str>, cancel_order_id: Option<i64>, new_client_order_id: Option<str>, strategy_id: Option<i64>, strategy_type: Option<i64>, stop_price: Option<f64>, trailing_delta: Option<f64>, iceberg_qty: Option<f64>, new_order_resp_type: Option<str>, self_trade_prevention_mode: Option<str>, recv_window: Option<i64>) -> Result<models::ApiV3OrderCancelReplacePost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_time_in_force { req_builder = req_builder.query(&[("timeInForce", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_quantity { req_builder = req_builder.query(&[("quantity", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OrderCancelReplaceResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_order_cancel_replace_post(
            &client.config(),
            symbol,
            side,
            r#type,
            cancel_replace_mode,
            timestamp,
            signature,
            cancel_restrictions.unwrap_or_default(),
            time_in_force.unwrap_or_default(),
            quantity.unwrap_or_default(),
            quote_order_qty.unwrap_or_default(),
            price.unwrap_or_default(),
            cancel_new_client_order_id.unwrap_or_default(),
            cancel_orig_client_order_id.unwrap_or_default(),
            cancel_order_id.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            strategy_id.unwrap_or_default(),
            strategy_type.unwrap_or_default(),
            stop_price.unwrap_or_default(),
            trailing_delta.unwrap_or_default(),
            iceberg_qty.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3Order")]
    async fn apiV3Order(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, order_id: Option<i64>, orig_client_order_id: Option<str>, new_client_order_id: Option<str>, cancel_restrictions: Option<str>, recv_window: Option<i64>) -> Result<models::Orde>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_order_delete(
            &client.config(),
            symbol,
            timestamp,
            signature,
            order_id.unwrap_or_default(),
            orig_client_order_id.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            cancel_restrictions.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3OrderList")]
    async fn apiV3OrderList(
        &self,
        ctx: &Context<'_>, symbol: str, timestamp: i64, signature: str, order_list_id: Option<i64>, list_client_order_id: Option<str>, new_client_order_id: Option<str>, recv_window: Option<i64>) -> Result<models::OcoOrde>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::DELETE, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OrderListResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_order_list_delete(
            &client.config(),
            symbol,
            timestamp,
            signature,
            order_list_id.unwrap_or_default(),
            list_client_order_id.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3OrderListOco")]
    async fn apiV3OrderListOco(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, quantity: f64, above_type: str, below_type: str, timestamp: i64, signature: str, list_client_order_id: Option<str>, above_client_order_id: Option<str>, above_iceberg_qty: Option<f64>, above_price: Option<f64>, above_stop_price: Option<f64>, above_trailing_delta: Option<f64>, above_time_in_force: Option<str>, above_strategy_id: Option<f64>, above_strategy_type: Option<i64>, below_client_order_id: Option<str>, below_iceberg_qty: Option<f64>, below_price: Option<f64>, below_stop_price: Option<f64>, below_trailing_delta: Option<f64>, below_time_in_force: Option<str>, below_strategy_id: Option<f64>, below_strategy_type: Option<i64>, new_order_resp_type: Option<str>, self_trade_prevention_mode: Option<str>, recv_window: Option<i64>) -> Result<models::ApiV3OrderListOcoPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_above_strategy_id { req_builder = req_builder.query(&[("aboveStrategyId", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_below_strategy_id { req_builder = req_builder.query(&[("belowStrategyId", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OrderListOcoResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_order_list_oco_post(
            &client.config(),
            symbol,
            side,
            quantity,
            above_type,
            below_type,
            timestamp,
            signature,
            list_client_order_id.unwrap_or_default(),
            above_client_order_id.unwrap_or_default(),
            above_iceberg_qty.unwrap_or_default(),
            above_price.unwrap_or_default(),
            above_stop_price.unwrap_or_default(),
            above_trailing_delta.unwrap_or_default(),
            above_time_in_force.unwrap_or_default(),
            above_strategy_id.unwrap_or_default(),
            above_strategy_type.unwrap_or_default(),
            below_client_order_id.unwrap_or_default(),
            below_iceberg_qty.unwrap_or_default(),
            below_price.unwrap_or_default(),
            below_stop_price.unwrap_or_default(),
            below_trailing_delta.unwrap_or_default(),
            below_time_in_force.unwrap_or_default(),
            below_strategy_id.unwrap_or_default(),
            below_strategy_type.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3OrderListOto")]
    async fn apiV3OrderListOto(
        &self,
        ctx: &Context<'_>, symbol: str, working_type: str, working_side: str, working_price: f64, working_quantity: f64, working_iceberg_qty: f64, pending_type: str, pending_side: str, pending_quantity: f64, timestamp: i64, signature: str, list_client_order_id: Option<str>, new_order_resp_type: Option<str>, self_trade_prevention_mode: Option<str>, working_client_order_id: Option<str>, working_time_in_force: Option<str>, working_strategy_id: Option<f64>, working_strategy_type: Option<i64>, pending_client_order_id: Option<str>, pending_price: Option<f64>, pending_stop_price: Option<f64>, pending_trailing_delta: Option<f64>, pending_iceberg_qty: Option<f64>, pending_time_in_force: Option<str>, pending_strategy_id: Option<f64>, pending_strategy_type: Option<i64>) -> Result<models::ApiV3OrderListOtoPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("workingType", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_working_strategy_id { req_builder = req_builder.query(&[("workingStrategyId", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_pending_strategy_id { req_builder = req_builder.query(&[("pendingStrategyId", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OrderListOtoResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_order_list_oto_post(
            &client.config(),
            symbol,
            working_type,
            working_side,
            working_price,
            working_quantity,
            working_iceberg_qty,
            pending_type,
            pending_side,
            pending_quantity,
            timestamp,
            signature,
            list_client_order_id.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            working_client_order_id.unwrap_or_default(),
            working_time_in_force.unwrap_or_default(),
            working_strategy_id.unwrap_or_default(),
            working_strategy_type.unwrap_or_default(),
            pending_client_order_id.unwrap_or_default(),
            pending_price.unwrap_or_default(),
            pending_stop_price.unwrap_or_default(),
            pending_trailing_delta.unwrap_or_default(),
            pending_iceberg_qty.unwrap_or_default(),
            pending_time_in_force.unwrap_or_default(),
            pending_strategy_id.unwrap_or_default(),
            pending_strategy_type.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3Order")]
    async fn apiV3Order(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, r#type: str, timestamp: i64, signature: str, time_in_force: Option<str>, quantity: Option<f64>, quote_order_qty: Option<f64>, price: Option<f64>, new_client_order_id: Option<str>, strategy_id: Option<i64>, strategy_type: Option<i64>, stop_price: Option<f64>, trailing_delta: Option<f64>, iceberg_qty: Option<f64>, new_order_resp_type: Option<str>, self_trade_prevention_mode: Option<str>, recv_window: Option<i64>) -> Result<models::ApiV3OrderPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_quantity { req_builder = req_builder.query(&[("quantity", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_order_post(
            &client.config(),
            symbol,
            side,
            r#type,
            timestamp,
            signature,
            time_in_force.unwrap_or_default(),
            quantity.unwrap_or_default(),
            quote_order_qty.unwrap_or_default(),
            price.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            strategy_id.unwrap_or_default(),
            strategy_type.unwrap_or_default(),
            stop_price.unwrap_or_default(),
            trailing_delta.unwrap_or_default(),
            iceberg_qty.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3OrderTest")]
    async fn apiV3OrderTest(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, r#type: str, timestamp: i64, signature: str, time_in_force: Option<str>, quantity: Option<f64>, quote_order_qty: Option<f64>, price: Option<f64>, new_client_order_id: Option<str>, strategy_id: Option<i64>, strategy_type: Option<i64>, stop_price: Option<f64>, trailing_delta: Option<f64>, iceberg_qty: Option<f64>, new_order_resp_type: Option<str>, recv_window: Option<i64>, compute_commission_rates: Option<bool>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_quantity { req_builder = req_builder.query(&[("quantity", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3OrderTestResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_order_test_post(
            &client.config(),
            symbol,
            side,
            r#type,
            timestamp,
            signature,
            time_in_force.unwrap_or_default(),
            quantity.unwrap_or_default(),
            quote_order_qty.unwrap_or_default(),
            price.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            strategy_id.unwrap_or_default(),
            strategy_type.unwrap_or_default(),
            stop_price.unwrap_or_default(),
            trailing_delta.unwrap_or_default(),
            iceberg_qty.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            compute_commission_rates.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3SorOrder")]
    async fn apiV3SorOrder(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, r#type: str, quantity: f64, timestamp: i64, signature: str, time_in_force: Option<str>, price: Option<f64>, new_client_order_id: Option<str>, strategy_id: Option<i64>, strategy_type: Option<i64>, iceberg_qty: Option<f64>, new_order_resp_type: Option<str>, self_trade_prevention_mode: Option<str>, recv_window: Option<i64>) -> Result<models::ApiV3SorOrderPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("quantity", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3SorOrderResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_sor_order_post(
            &client.config(),
            symbol,
            side,
            r#type,
            quantity,
            timestamp,
            signature,
            time_in_force.unwrap_or_default(),
            price.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            strategy_id.unwrap_or_default(),
            strategy_type.unwrap_or_default(),
            iceberg_qty.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "apiV3SorOrderTest")]
    async fn apiV3SorOrderTest(
        &self,
        ctx: &Context<'_>, symbol: str, side: str, r#type: str, quantity: f64, timestamp: i64, signature: str, time_in_force: Option<str>, price: Option<f64>, new_client_order_id: Option<str>, strategy_id: Option<i64>, strategy_type: Option<i64>, iceberg_qty: Option<f64>, new_order_resp_type: Option<str>, self_trade_prevention_mode: Option<str>, compute_commission_rates: Option<bool>, recv_window: Option<i64>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } req_builder = req_builder.query(&[("quantity", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_self_trade_prevention_mode { req_builder = req_builder.query(&[("selfTradePreventionMode", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_compute_commission_rates { req_builder = req_builder.query(&[("computeCommissionRates", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<ApiV3SorOrderTestResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::trade_api::api_v3_sor_order_test_post(
            &client.config(),
            symbol,
            side,
            r#type,
            quantity,
            timestamp,
            signature,
            time_in_force.unwrap_or_default(),
            price.unwrap_or_default(),
            new_client_order_id.unwrap_or_default(),
            strategy_id.unwrap_or_default(),
            strategy_type.unwrap_or_default(),
            iceberg_qty.unwrap_or_default(),
            new_order_resp_type.unwrap_or_default(),
            self_trade_prevention_mode.unwrap_or_default(),
            compute_commission_rates.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            serde_json,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LoanVipBorrow")]
    async fn sapiV1LoanVipBorrow(
        &self,
        ctx: &Context<'_>, loan_account_id: i64, loan_amount: f32, collateral_account_id: str, collateral_coin: str, is_flexible_rate: str, timestamp: i64, signature: str, loan_coin: Option<str>, loan_term: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipBorrowPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanVipBorrowResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::vip_loans_api::sapi_v1_loan_vip_borrow_post(
            &client.config(),
            loan_account_id,
            loan_amount,
            collateral_account_id,
            collateral_coin,
            is_flexible_rate,
            timestamp,
            signature,
            loan_coin.unwrap_or_default(),
            loan_term.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LoanVipRenew")]
    async fn sapiV1LoanVipRenew(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, order_id: Option<i64>, loan_term: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipRenewPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanVipRenewResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::vip_loans_api::sapi_v1_loan_vip_renew_post(
            &client.config(),
            timestamp,
            signature,
            order_id.unwrap_or_default(),
            loan_term.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1LoanVipRepay")]
    async fn sapiV1LoanVipRepay(
        &self,
        ctx: &Context<'_>, amount: f64, timestamp: i64, signature: str, order_id: Option<i64>, recv_window: Option<i64>) -> Result<models::SapiV1LoanVipRepayPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<LoanVipRepayResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::vip_loans_api::sapi_v1_loan_vip_repay_post(
            &client.config(),
            amount,
            timestamp,
            signature,
            order_id.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AccountDisableFastWithdrawSwitch")]
    async fn sapiV1AccountDisableFastWithdrawSwitch(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AccountDisableFastWithdrawSwitchResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_account_disable_fast_withdraw_switch_post(
            &client.config(),
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AccountEnableFastWithdrawSwitch")]
    async fn sapiV1AccountEnableFastWithdrawSwitch(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<serde_json::Valu>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AccountEnableFastWithdrawSwitchResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_account_enable_fast_withdraw_switch_post(
            &client.config(),
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AssetConvertTransfer")]
    async fn sapiV1AssetConvertTransfer(
        &self,
        ctx: &Context<'_>, client_tran_id: str, asset: str, amount: f64, target_asset: str, timestamp: i64, signature: str, recv_window: Option<i64>) -> Result<models::SapiV1AssetConvertTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AssetConvertTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_asset_convert_transfer_post(
            &client.config(),
            client_tran_id,
            asset,
            amount,
            target_asset,
            timestamp,
            signature,
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AssetDustBtc")]
    async fn sapiV1AssetDustBtc(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, account_type: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1AssetDustBtcPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AssetDustBtcResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_asset_dust_btc_post(
            &client.config(),
            timestamp,
            signature,
            account_type.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AssetDust")]
    async fn sapiV1AssetDust(
        &self,
        ctx: &Context<'_>, asset: Vec<String>, timestamp: i64, signature: str, account_type: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1AssetDustPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p.to_string())).collect: :<Vec<(std::string::String, std: :string::String)>>()), p_query_asset.into_iter().map(|p| p.to_string()).collect: :<Vec<String>>().join(", serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AssetDustResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_asset_dust_post(
            &client.config(),
            asset,
            timestamp,
            signature,
            account_type.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p.to_string())).collect,
            std,
            p_query_asset.into_iter().map(|p| p.to_string()).collect,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AssetGetFundingAsset")]
    async fn sapiV1AssetGetFundingAsset(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, asset: Option<str>, need_btc_valuation: Option<str>, recv_window: Option<i64>) -> Result<Vec<models::SapiV1AssetGetFundingAssetPost200ResponseInner>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AssetFundingAssetResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_asset_get_funding_asset_post(
            &client.config(),
            timestamp,
            signature,
            asset.unwrap_or_default(),
            need_btc_valuation.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1AssetTransfer")]
    async fn sapiV1AssetTransfer(
        &self,
        ctx: &Context<'_>, r#type: str, asset: str, amount: f64, timestamp: i64, signature: str, from_symbol: Option<str>, to_symbol: Option<str>, recv_window: Option<i64>) -> Result<models::SapiV1AssetTransferPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<AssetTransferResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_asset_transfer_post(
            &client.config(),
            r#type,
            asset,
            amount,
            timestamp,
            signature,
            from_symbol.unwrap_or_default(),
            to_symbol.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1CapitalContractConvertibleCoins")]
    async fn sapiV1CapitalContractConvertibleCoins(
        &self,
        ctx: &Context<'_>, coin: str, enable: bool) -> Result<serde_json::Value, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_enable.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<CapitalContractConvertibleCoinsResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_capital_contract_convertible_coins_post(
            &client.config(),
            coin,
            enable,
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_enable.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1CapitalDepositCreditApply")]
    async fn sapiV1CapitalDepositCreditApply(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, deposit_id: Option<i64>, tx_id: Option<str>, sub_account_id: Option<i64>, sub_user_id: Option<i64>, recv_window: Option<i64>) -> Result<models::SapiV1CapitalDepositCreditApplyPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<CapitalDepositCreditApplyResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_capital_deposit_credit_apply_post(
            &client.config(),
            timestamp,
            signature,
            deposit_id.unwrap_or_default(),
            tx_id.unwrap_or_default(),
            sub_account_id.unwrap_or_default(),
            sub_user_id.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV1CapitalWithdrawApply")]
    async fn sapiV1CapitalWithdrawApply(
        &self,
        ctx: &Context<'_>, coin: str, address: str, amount: f64, timestamp: i64, signature: str, withdraw_order_id: Option<str>, network: Option<str>, address_tag: Option<str>, transaction_fee_flag: Option<bool>, name: Option<str>, wallet_type: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1CapitalWithdrawApplyPost200Respons>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<CapitalWithdrawApplyResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v1_capital_withdraw_apply_post(
            &client.config(),
            coin,
            address,
            amount,
            timestamp,
            signature,
            withdraw_order_id.unwrap_or_default(),
            network.unwrap_or_default(),
            address_tag.unwrap_or_default(),
            transaction_fee_flag.unwrap_or_default(),
            name.unwrap_or_default(),
            wallet_type.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }

    #[graphql(name = "sapiV3AssetGetUserAsset")]
    async fn sapiV3AssetGetUserAsset(
        &self,
        ctx: &Context<'_>, timestamp: i64, signature: str, asset: Option<str>, need_btc_valuation: Option<str>, recv_window: Option<i64>) -> Result<Vec<models::SapiV3AssetGetUserAssetPost200ResponseInner>, configuration.base_path); let mut req_builder = configuration.client.request(reqwest: :Method::POST, serde_json: :to_string(param_value)?)]); } if let Some(ref param_value) = p_query_recv_window { req_builder = req_builder.query(&[("recvWindow", p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest: :header::USER_AGENT
    ) -> FieldResult<V3AssetUserAssetResponse> {
        let client = ctx.data_unchecked::<std::sync::Arc<BinanceClient>>();
        binance::wallet_api::sapi_v3_asset_get_user_asset_post(
            &client.config(),
            timestamp,
            signature,
            asset.unwrap_or_default(),
            need_btc_valuation.unwrap_or_default(),
            recv_window.unwrap_or_default(),
            configuration.base_path); let mut req_builder = configuration.client.request(reqwest,
            serde_json,
            p_query_signature.to_string())]); if let Some(ref user_agent) = configuration.user_agent { req_builder = req_builder.header(reqwest
        ).await
        .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}
