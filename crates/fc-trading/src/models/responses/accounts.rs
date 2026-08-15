use serde::Deserialize;

use crate::DecimalNumber;

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct CashAccountBalance {
    pub account: String,
    #[serde(rename = "cashBal")]
    pub cash_balance: DecimalNumber,
    #[serde(rename = "cashOnHold")]
    pub cash_on_hold: DecimalNumber,
    #[serde(rename = "secureAmount")]
    pub secure_amount: DecimalNumber,
    pub withdrawable: DecimalNumber,
    #[serde(rename = "receivingCashT1")]
    pub receiving_cash_t1: DecimalNumber,
    #[serde(rename = "receivingCashT2")]
    pub receiving_cash_t2: DecimalNumber,
    #[serde(rename = "matchedBuyVolume")]
    pub matched_buy_volume: DecimalNumber,
    #[serde(rename = "matchedSellVolume")]
    pub matched_sell_volume: DecimalNumber,
    #[serde(rename = "unMatchedBuyVolume", alias = "unmatchedBuyVolume")]
    pub unmatched_buy_volume: DecimalNumber,
    #[serde(rename = "unMatchedSellVolume", alias = "unmatchedSellVolume")]
    pub unmatched_sell_volume: DecimalNumber,
    #[serde(rename = "paidCashT1")]
    pub paid_cash_t1: DecimalNumber,
    #[serde(rename = "paidCashT2")]
    pub paid_cash_t2: DecimalNumber,
    pub cia: DecimalNumber,
    pub debt: DecimalNumber,
    #[serde(rename = "purchasingPower")]
    pub purchasing_power: DecimalNumber,
    #[serde(rename = "totalAssets")]
    pub total_assets: DecimalNumber,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct DerivativeAccountBalance {
    pub account: String,
    #[serde(rename = "accountBalance")]
    pub account_balance: DecimalNumber,
    pub fee: DecimalNumber,
    pub commission: DecimalNumber,
    pub interest: DecimalNumber,
    pub loan: DecimalNumber,
    #[serde(rename = "deliveryAmount")]
    pub delivery_amount: DecimalNumber,
    #[serde(rename = "floatingPL")]
    pub floating_pl: DecimalNumber,
    #[serde(rename = "totalPL")]
    pub total_pl: DecimalNumber,
    pub marginable: DecimalNumber,
    pub depositable: DecimalNumber,
    #[serde(rename = "rcCall")]
    pub rc_call: DecimalNumber,
    pub withdrawable: DecimalNumber,
    #[serde(rename = "nonCashDrawableRCCall")]
    pub non_cash_drawable_rc_call: DecimalNumber,
    #[serde(rename = "internalAssets")]
    pub internal_assets: AssetSummary,
    #[serde(rename = "exchangeAssets")]
    pub exchange_assets: AssetSummary,
    #[serde(rename = "internalMargin")]
    pub internal_margin: InternalMargin,
    #[serde(rename = "exchangeMargin")]
    pub exchange_margin: ExchangeMargin,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct AssetSummary {
    pub cash: DecimalNumber,
    #[serde(rename = "validNonCash")]
    pub valid_non_cash: DecimalNumber,
    #[serde(rename = "totalValue")]
    pub total_value: DecimalNumber,
    #[serde(rename = "maxValidNonCash")]
    pub max_valid_non_cash: DecimalNumber,
    #[serde(rename = "cashWithdrawable")]
    pub cash_withdrawable: DecimalNumber,
    pub ee: DecimalNumber,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct InternalMargin {
    #[serde(rename = "initialMargin")]
    pub initial_margin: DecimalNumber,
    #[serde(rename = "deliveryMargin")]
    pub delivery_margin: DecimalNumber,
    #[serde(rename = "marginReq")]
    pub margin_requirement: DecimalNumber,
    #[serde(rename = "accountRatio")]
    pub account_ratio: DecimalNumber,
    #[serde(rename = "usedLimitWarningLevel1")]
    pub used_limit_warning_level_1: DecimalNumber,
    #[serde(rename = "usedLimitWarningLevel2")]
    pub used_limit_warning_level_2: DecimalNumber,
    #[serde(rename = "usedLimitWarningLevel3")]
    pub used_limit_warning_level_3: DecimalNumber,
    #[serde(rename = "marginCall")]
    pub margin_call: DecimalNumber,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct ExchangeMargin {
    #[serde(rename = "marginReq")]
    pub margin_requirement: DecimalNumber,
    #[serde(rename = "accountRatio")]
    pub account_ratio: DecimalNumber,
    #[serde(rename = "usedLimitWarningLevel1")]
    pub used_limit_warning_level_1: DecimalNumber,
    #[serde(rename = "usedLimitWarningLevel2")]
    pub used_limit_warning_level_2: DecimalNumber,
    #[serde(rename = "usedLimitWarningLevel3")]
    pub used_limit_warning_level_3: DecimalNumber,
    #[serde(rename = "marginCall")]
    pub margin_call: DecimalNumber,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[non_exhaustive]
pub struct PpmmrAccountBalance {
    #[serde(rename = "collateralAsset")]
    pub collateral_asset: DecimalNumber,
    #[serde(rename = "callLMW")]
    pub call_lmw: DecimalNumber,
    pub liability: DecimalNumber,
    #[serde(rename = "eeOrigin")]
    pub ee_origin: DecimalNumber,
    #[serde(rename = "forceLMV")]
    pub force_lmv: DecimalNumber,
    pub equity: DecimalNumber,
    pub ee: DecimalNumber,
    #[serde(rename = "callMargin")]
    pub call_margin: DecimalNumber,
    #[serde(rename = "cashBalance")]
    pub cash_balance: DecimalNumber,
    #[serde(rename = "purchasingPower")]
    pub purchasing_power: DecimalNumber,
    #[serde(rename = "callForcesell")]
    pub call_force_sell: DecimalNumber,
    pub lmv: DecimalNumber,
    #[serde(rename = "marginCall")]
    pub margin_call: DecimalNumber,
    pub withdrawal: DecimalNumber,
    #[serde(rename = "collateralA")]
    pub collateral_a: DecimalNumber,
    pub action: String,
    #[serde(rename = "marginRatio")]
    pub margin_ratio: DecimalNumber,
    pub debt: DecimalNumber,
    #[serde(rename = "accruedlongerest", alias = "accruedInterest")]
    pub accrued_interest: DecimalNumber,
    #[serde(rename = "holdRight")]
    pub hold_right: DecimalNumber,
    #[serde(rename = "preLoan")]
    pub pre_loan: DecimalNumber,
    pub fees: DecimalNumber,
    #[serde(rename = "buyUnmatch")]
    pub buy_unmatched: DecimalNumber,
    pub ap: DecimalNumber,
    #[serde(rename = "apT1")]
    pub ap_t1: DecimalNumber,
    #[serde(rename = "sellUnmatch")]
    pub sell_unmatched: DecimalNumber,
    pub cia: DecimalNumber,
    pub ar: DecimalNumber,
    #[serde(rename = "arT1")]
    pub ar_t1: DecimalNumber,
    #[serde(rename = "ppCredit")]
    pub pp_credit: DecimalNumber,
    #[serde(rename = "creditLimit")]
    pub credit_limit: DecimalNumber,
    #[serde(rename = "totalAssets")]
    pub total_assets: DecimalNumber,
    #[serde(rename = "marginCallLMVSold")]
    pub margin_call_lmv_sold: DecimalNumber,
    #[serde(rename = "lmvNonMarginable")]
    pub lmv_non_marginable: DecimalNumber,
    #[serde(rename = "eeCredit")]
    pub ee_credit: DecimalNumber,
    #[serde(rename = "totalEquity")]
    pub total_equity: DecimalNumber,
    #[serde(rename = "eE90")]
    pub ee_90: DecimalNumber,
    #[serde(rename = "eE80")]
    pub ee_80: DecimalNumber,
    #[serde(rename = "eE70")]
    pub ee_70: DecimalNumber,
    #[serde(rename = "eE60")]
    pub ee_60: DecimalNumber,
    #[serde(rename = "eE50")]
    pub ee_50: DecimalNumber,
}
