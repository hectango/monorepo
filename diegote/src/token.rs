pub use f_da_ix::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod f_da_ix {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\": [{\"internalType\": \"contract ISuperfluid\", \"name\": \"host\", \"type\": \"address\"}, {\"internalType\": \"contract IConstantOutflowNFT\", \"name\": \"constantOutflowNFTLogic\", \"type\": \"address\"}, {\"internalType\": \"contract IConstantInflowNFT\", \"name\": \"constantInflowNFTLogic\", \"type\": \"address\"}], \"stateMutability\": \"nonpayable\", \"type\": \"constructor\"}, {\"inputs\": [], \"name\": \"SF_TOKEN_AGREEMENT_ALREADY_EXISTS\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SF_TOKEN_AGREEMENT_DOES_NOT_EXIST\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SF_TOKEN_BURN_INSUFFICIENT_BALANCE\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SF_TOKEN_MOVE_INSUFFICIENT_BALANCE\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SF_TOKEN_ONLY_HOST\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SF_TOKEN_ONLY_LISTED_AGREEMENT\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_MINT_TO_ZERO_ADDRESS\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_NFT_PROXY_ALREADY_SET\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_NO_UNDERLYING_TOKEN\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_ONLY_GOV_OWNER\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_ONLY_HOST\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_ONLY_SELF\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS\", \"type\": \"error\"}, {\"inputs\": [], \"name\": \"SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS\", \"type\": \"error\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"indexed\": false, \"internalType\": \"bytes32[]\", \"name\": \"data\", \"type\": \"bytes32[]\"}], \"name\": \"AgreementCreated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"penaltyAccount\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"rewardAccount\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"rewardAmount\", \"type\": \"uint256\"}], \"name\": \"AgreementLiquidated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": false, \"internalType\": \"address\", \"name\": \"liquidatorAccount\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"penaltyAccount\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"bondAccount\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"rewardAmount\", \"type\": \"uint256\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"bailoutAmount\", \"type\": \"uint256\"}], \"name\": \"AgreementLiquidatedBy\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"liquidatorAccount\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"targetAccount\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"address\", \"name\": \"rewardAmountReceiver\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"rewardAmount\", \"type\": \"uint256\"}, {\"indexed\": false, \"internalType\": \"int256\", \"name\": \"targetAccountBalanceDelta\", \"type\": \"int256\"}, {\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"liquidationTypeData\", \"type\": \"bytes\"}], \"name\": \"AgreementLiquidatedV2\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"slotId\", \"type\": \"uint256\"}], \"name\": \"AgreementStateUpdated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}], \"name\": \"AgreementTerminated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"indexed\": false, \"internalType\": \"bytes32[]\", \"name\": \"data\", \"type\": \"bytes32[]\"}], \"name\": \"AgreementUpdated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"owner\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"value\", \"type\": \"uint256\"}], \"name\": \"Approval\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"tokenHolder\", \"type\": \"address\"}], \"name\": \"AuthorizedOperator\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"bailoutAccount\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"bailoutAmount\", \"type\": \"uint256\"}], \"name\": \"Bailout\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}, {\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"operatorData\", \"type\": \"bytes\"}], \"name\": \"Burned\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": false, \"internalType\": \"bytes32\", \"name\": \"uuid\", \"type\": \"bytes32\"}, {\"indexed\": false, \"internalType\": \"address\", \"name\": \"codeAddress\", \"type\": \"address\"}], \"name\": \"CodeUpdated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"contract IConstantInflowNFT\", \"name\": \"constantInflowNFT\", \"type\": \"address\"}], \"name\": \"ConstantInflowNFTCreated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"contract IConstantOutflowNFT\", \"name\": \"constantOutflowNFT\", \"type\": \"address\"}], \"name\": \"ConstantOutflowNFTCreated\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": false, \"internalType\": \"uint8\", \"name\": \"version\", \"type\": \"uint8\"}], \"name\": \"Initialized\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}, {\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"operatorData\", \"type\": \"bytes\"}], \"name\": \"Minted\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"tokenHolder\", \"type\": \"address\"}], \"name\": \"RevokedOperator\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}, {\"indexed\": false, \"internalType\": \"bytes\", \"name\": \"operatorData\", \"type\": \"bytes\"}], \"name\": \"Sent\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"TokenDowngraded\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"TokenUpgraded\", \"type\": \"event\"}, {\"anonymous\": false, \"inputs\": [{\"indexed\": true, \"internalType\": \"address\", \"name\": \"from\", \"type\": \"address\"}, {\"indexed\": true, \"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\"}, {\"indexed\": false, \"internalType\": \"uint256\", \"name\": \"value\", \"type\": \"uint256\"}], \"name\": \"Transfer\", \"type\": \"event\"}, {\"inputs\": [], \"name\": \"CONSTANT_INFLOW_NFT_LOGIC\", \"outputs\": [{\"internalType\": \"contract IConstantInflowNFT\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"CONSTANT_OUTFLOW_NFT_LOGIC\", \"outputs\": [{\"internalType\": \"contract IConstantOutflowNFT\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS\", \"outputs\": [{\"internalType\": \"address\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}], \"name\": \"allowance\", \"outputs\": [{\"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"approve\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\"}], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}], \"name\": \"authorizeOperator\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}], \"name\": \"balanceOf\", \"outputs\": [{\"internalType\": \"uint256\", \"name\": \"balance\", \"type\": \"uint256\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}], \"name\": \"burn\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"castrate\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"constantInflowNFT\", \"outputs\": [{\"internalType\": \"contract IConstantInflowNFT\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"constantOutflowNFT\", \"outputs\": [{\"internalType\": \"contract IConstantOutflowNFT\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"internalType\": \"bytes32[]\", \"name\": \"data\", \"type\": \"bytes32[]\"}], \"name\": \"createAgreement\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"decimals\", \"outputs\": [{\"internalType\": \"uint8\", \"name\": \"\", \"type\": \"uint8\"}], \"stateMutability\": \"pure\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"subtractedValue\", \"type\": \"uint256\"}], \"name\": \"decreaseAllowance\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\"}], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"defaultOperators\", \"outputs\": [{\"internalType\": \"address[]\", \"name\": \"\", \"type\": \"address[]\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"downgrade\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"downgradeTo\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}], \"name\": \"getAccountActiveAgreements\", \"outputs\": [{\"internalType\": \"contract ISuperAgreement[]\", \"name\": \"\", \"type\": \"address[]\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"internalType\": \"uint256\", \"name\": \"dataLength\", \"type\": \"uint256\"}], \"name\": \"getAgreementData\", \"outputs\": [{\"internalType\": \"bytes32[]\", \"name\": \"data\", \"type\": \"bytes32[]\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"agreementClass\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"slotId\", \"type\": \"uint256\"}, {\"internalType\": \"uint256\", \"name\": \"dataLength\", \"type\": \"uint256\"}], \"name\": \"getAgreementStateSlot\", \"outputs\": [{\"internalType\": \"bytes32[]\", \"name\": \"slotData\", \"type\": \"bytes32[]\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"getCodeAddress\", \"outputs\": [{\"internalType\": \"address\", \"name\": \"codeAddress\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"getHost\", \"outputs\": [{\"internalType\": \"address\", \"name\": \"host\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"getUnderlyingToken\", \"outputs\": [{\"internalType\": \"address\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"granularity\", \"outputs\": [{\"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\"}], \"stateMutability\": \"pure\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"addedValue\", \"type\": \"uint256\"}], \"name\": \"increaseAllowance\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\"}], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"contract IERC20\", \"name\": \"underlyingToken\", \"type\": \"address\"}, {\"internalType\": \"uint8\", \"name\": \"underlyingDecimals\", \"type\": \"uint8\"}, {\"internalType\": \"string\", \"name\": \"n\", \"type\": \"string\"}, {\"internalType\": \"string\", \"name\": \"s\", \"type\": \"string\"}], \"name\": \"initialize\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"timestamp\", \"type\": \"uint256\"}], \"name\": \"isAccountCritical\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"isCritical\", \"type\": \"bool\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}], \"name\": \"isAccountCriticalNow\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"isCritical\", \"type\": \"bool\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"timestamp\", \"type\": \"uint256\"}], \"name\": \"isAccountSolvent\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"isSolvent\", \"type\": \"bool\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}], \"name\": \"isAccountSolventNow\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"isSolvent\", \"type\": \"bool\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"tokenHolder\", \"type\": \"address\"}], \"name\": \"isOperatorFor\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"internalType\": \"bytes\", \"name\": \"liquidationTypeData\", \"type\": \"bytes\"}, {\"internalType\": \"address\", \"name\": \"liquidatorAccount\", \"type\": \"address\"}, {\"internalType\": \"bool\", \"name\": \"useDefaultRewardAccount\", \"type\": \"bool\"}, {\"internalType\": \"address\", \"name\": \"targetAccount\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"rewardAmount\", \"type\": \"uint256\"}, {\"internalType\": \"int256\", \"name\": \"targetAccountBalanceDelta\", \"type\": \"int256\"}], \"name\": \"makeLiquidationPayoutsV2\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"name\", \"outputs\": [{\"internalType\": \"string\", \"name\": \"\", \"type\": \"string\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"operationApprove\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"subtractedValue\", \"type\": \"uint256\"}], \"name\": \"operationDecreaseAllowance\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"operationDowngrade\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"addedValue\", \"type\": \"uint256\"}], \"name\": \"operationIncreaseAllowance\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"userData\", \"type\": \"bytes\"}], \"name\": \"operationSend\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"operationTransferFrom\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"operationUpgrade\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}, {\"internalType\": \"bytes\", \"name\": \"operatorData\", \"type\": \"bytes\"}], \"name\": \"operatorBurn\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"sender\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}, {\"internalType\": \"bytes\", \"name\": \"operatorData\", \"type\": \"bytes\"}], \"name\": \"operatorSend\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"poolAdminNFT\", \"outputs\": [{\"internalType\": \"contract IPoolAdminNFT\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"poolMemberNFT\", \"outputs\": [{\"internalType\": \"contract IPoolMemberNFT\", \"name\": \"\", \"type\": \"address\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"proxiableUUID\", \"outputs\": [{\"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\"}], \"stateMutability\": \"pure\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"timestamp\", \"type\": \"uint256\"}], \"name\": \"realtimeBalanceOf\", \"outputs\": [{\"internalType\": \"int256\", \"name\": \"availableBalance\", \"type\": \"int256\"}, {\"internalType\": \"uint256\", \"name\": \"deposit\", \"type\": \"uint256\"}, {\"internalType\": \"uint256\", \"name\": \"owedDeposit\", \"type\": \"uint256\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}], \"name\": \"realtimeBalanceOfNow\", \"outputs\": [{\"internalType\": \"int256\", \"name\": \"availableBalance\", \"type\": \"int256\"}, {\"internalType\": \"uint256\", \"name\": \"deposit\", \"type\": \"uint256\"}, {\"internalType\": \"uint256\", \"name\": \"owedDeposit\", \"type\": \"uint256\"}, {\"internalType\": \"uint256\", \"name\": \"timestamp\", \"type\": \"uint256\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"operator\", \"type\": \"address\"}], \"name\": \"revokeOperator\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"selfApproveFor\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"userData\", \"type\": \"bytes\"}], \"name\": \"selfBurn\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"userData\", \"type\": \"bytes\"}], \"name\": \"selfMint\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"holder\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"spender\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"selfTransferFrom\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}], \"name\": \"send\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"int256\", \"name\": \"delta\", \"type\": \"int256\"}], \"name\": \"settleBalance\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"symbol\", \"outputs\": [{\"internalType\": \"string\", \"name\": \"\", \"type\": \"string\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"internalType\": \"uint256\", \"name\": \"dataLength\", \"type\": \"uint256\"}], \"name\": \"terminateAgreement\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [], \"name\": \"totalSupply\", \"outputs\": [{\"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\"}], \"stateMutability\": \"view\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"transfer\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\"}], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}], \"name\": \"transferAll\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"holder\", \"type\": \"address\"}, {\"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"transferFrom\", \"outputs\": [{\"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\"}], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"bytes32\", \"name\": \"id\", \"type\": \"bytes32\"}, {\"internalType\": \"bytes32[]\", \"name\": \"data\", \"type\": \"bytes32[]\"}], \"name\": \"updateAgreementData\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"account\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"slotId\", \"type\": \"uint256\"}, {\"internalType\": \"bytes32[]\", \"name\": \"slotData\", \"type\": \"bytes32[]\"}], \"name\": \"updateAgreementStateSlot\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"newAddress\", \"type\": \"address\"}], \"name\": \"updateCode\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}], \"name\": \"upgrade\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}, {\"inputs\": [{\"internalType\": \"address\", \"name\": \"to\", \"type\": \"address\"}, {\"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\"}, {\"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\"}], \"name\": \"upgradeTo\", \"outputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"function\"}]";
    ///The parsed JSON ABI of the contract.
    pub static FDAIX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct fDAIx<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for fDAIx<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for fDAIx<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for fDAIx<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for fDAIx<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(fDAIx)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> fDAIx<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FDAIX_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `CONSTANT_INFLOW_NFT_LOGIC` (0x5a7792df) function
        pub fn constant_inflow_nft_logic(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([90, 119, 146, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CONSTANT_OUTFLOW_NFT_LOGIC` (0x5365d19f) function
        pub fn constant_outflow_nft_logic(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([83, 101, 209, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS` (0x70d75815) function
        pub fn superfluid_nft_deployer_library_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([112, 215, 88, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            account: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (account, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authorizeOperator` (0x959b8c3f) function
        pub fn authorize_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 155, 140, 63], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0xfe9d9303) function
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 157, 147, 3], (amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castrate` (0x9903ad38) function
        pub fn castrate(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 3, 173, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `constantInflowNFT` (0xd50911cc) function
        pub fn constant_inflow_nft(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([213, 9, 17, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `constantOutflowNFT` (0x0d9c12b5) function
        pub fn constant_outflow_nft(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 156, 18, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAgreement` (0x12a6a3f8) function
        pub fn create_agreement(
            &self,
            id: [u8; 32],
            data: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 166, 163, 248], (id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultOperators` (0x06e48538) function
        pub fn default_operators(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([6, 228, 133, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `downgrade` (0x11bcc81e) function
        pub fn downgrade(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 188, 200, 30], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `downgradeTo` (0x83ba2525) function
        pub fn downgrade_to(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 186, 37, 37], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAccountActiveAgreements` (0x386fa221) function
        pub fn get_account_active_agreements(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([56, 111, 162, 33], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAgreementData` (0x6c2d9f2f) function
        pub fn get_agreement_data(
            &self,
            agreement_class: ::ethers::core::types::Address,
            id: [u8; 32],
            data_length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([108, 45, 159, 47], (agreement_class, id, data_length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAgreementStateSlot` (0x4b61cc33) function
        pub fn get_agreement_state_slot(
            &self,
            agreement_class: ::ethers::core::types::Address,
            account: ::ethers::core::types::Address,
            slot_id: ::ethers::core::types::U256,
            data_length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash(
                    [75, 97, 204, 51],
                    (agreement_class, account, slot_id, data_length),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCodeAddress` (0x50d75d25) function
        pub fn get_code_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 215, 93, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHost` (0x20bc4425) function
        pub fn get_host(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([32, 188, 68, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnderlyingToken` (0xee719bc8) function
        pub fn get_underlying_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([238, 113, 155, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `granularity` (0x556f0dc7) function
        pub fn granularity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 111, 13, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x42fe0980) function
        pub fn initialize(
            &self,
            underlying_token: ::ethers::core::types::Address,
            underlying_decimals: u8,
            n: ::std::string::String,
            s: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [66, 254, 9, 128],
                    (underlying_token, underlying_decimals, n, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAccountCritical` (0xd9d078d6) function
        pub fn is_account_critical(
            &self,
            account: ::ethers::core::types::Address,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 208, 120, 214], (account, timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAccountCriticalNow` (0x79359f6f) function
        pub fn is_account_critical_now(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([121, 53, 159, 111], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAccountSolvent` (0xb84cdd4a) function
        pub fn is_account_solvent(
            &self,
            account: ::ethers::core::types::Address,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([184, 76, 221, 74], (account, timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAccountSolventNow` (0xbb0d196e) function
        pub fn is_account_solvent_now(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([187, 13, 25, 110], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperatorFor` (0xd95b6371) function
        pub fn is_operator_for(
            &self,
            operator: ::ethers::core::types::Address,
            token_holder: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 91, 99, 113], (operator, token_holder))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makeLiquidationPayoutsV2` (0x1863e809) function
        pub fn make_liquidation_payouts_v2(
            &self,
            id: [u8; 32],
            liquidation_type_data: ::ethers::core::types::Bytes,
            liquidator_account: ::ethers::core::types::Address,
            use_default_reward_account: bool,
            target_account: ::ethers::core::types::Address,
            reward_amount: ::ethers::core::types::U256,
            target_account_balance_delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [24, 99, 232, 9],
                    (
                        id,
                        liquidation_type_data,
                        liquidator_account,
                        use_default_reward_account,
                        target_account,
                        reward_amount,
                        target_account_balance_delta,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operationApprove` (0x62aa5287) function
        pub fn operation_approve(
            &self,
            account: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 170, 82, 135], (account, spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operationDecreaseAllowance` (0xc780fd82) function
        pub fn operation_decrease_allowance(
            &self,
            account: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 128, 253, 130], (account, spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operationDowngrade` (0x245887fc) function
        pub fn operation_downgrade(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 88, 135, 252], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operationIncreaseAllowance` (0x4b2763b3) function
        pub fn operation_increase_allowance(
            &self,
            account: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 39, 99, 179], (account, spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operationSend` (0xca0c1e7f) function
        pub fn operation_send(
            &self,
            spender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 12, 30, 127], (spender, recipient, amount, user_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operationTransferFrom` (0x16d055d6) function
        pub fn operation_transfer_from(
            &self,
            account: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 208, 85, 214], (account, spender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operationUpgrade` (0xca789464) function
        pub fn operation_upgrade(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 120, 148, 100], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorBurn` (0xfc673c4f) function
        pub fn operator_burn(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 103, 60, 79], (account, amount, data, operator_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorSend` (0x62ad1b83) function
        pub fn operator_send(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            operator_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [98, 173, 27, 131],
                    (sender, recipient, amount, data, operator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolAdminNFT` (0xc76058fc) function
        pub fn pool_admin_nft(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([199, 96, 88, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolMemberNFT` (0xc4b1584c) function
        pub fn pool_member_nft(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 177, 88, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `realtimeBalanceOf` (0xeb3537cc) function
        pub fn realtime_balance_of(
            &self,
            account: ::ethers::core::types::Address,
            timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([235, 53, 55, 204], (account, timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `realtimeBalanceOfNow` (0x2ec8eec7) function
        pub fn realtime_balance_of_now(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([46, 200, 238, 199], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeOperator` (0xfad8b32a) function
        pub fn revoke_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 216, 179, 42], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfApproveFor` (0x66a12fb6) function
        pub fn self_approve_for(
            &self,
            account: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 161, 47, 182], (account, spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfBurn` (0x9d876741) function
        pub fn self_burn(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 135, 103, 65], (account, amount, user_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfMint` (0xc68d4283) function
        pub fn self_mint(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 141, 66, 131], (account, amount, user_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfTransferFrom` (0x41b706be) function
        pub fn self_transfer_from(
            &self,
            holder: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 183, 6, 190], (holder, spender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `send` (0x9bd9bbc6) function
        pub fn send(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 217, 187, 198], (recipient, amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settleBalance` (0xcf97256d) function
        pub fn settle_balance(
            &self,
            account: ::ethers::core::types::Address,
            delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 151, 37, 109], (account, delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `terminateAgreement` (0x27048397) function
        pub fn terminate_agreement(
            &self,
            id: [u8; 32],
            data_length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 4, 131, 151], (id, data_length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferAll` (0xa3a7e7f3) function
        pub fn transfer_all(
            &self,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 167, 231, 243], recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            holder: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (holder, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateAgreementData` (0xa1b2bf8b) function
        pub fn update_agreement_data(
            &self,
            id: [u8; 32],
            data: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 178, 191, 139], (id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateAgreementStateSlot` (0x090c415e) function
        pub fn update_agreement_state_slot(
            &self,
            account: ::ethers::core::types::Address,
            slot_id: ::ethers::core::types::U256,
            slot_data: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 12, 65, 94], (account, slot_id, slot_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateCode` (0x46951954) function
        pub fn update_code(
            &self,
            new_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 149, 25, 84], new_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrade` (0x45977d03) function
        pub fn upgrade(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 151, 125, 3], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeTo` (0x5b9d09cc) function
        pub fn upgrade_to(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 157, 9, 204], (to, amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AgreementCreated` event
        pub fn agreement_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AgreementLiquidated` event
        pub fn agreement_liquidated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementLiquidatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AgreementLiquidatedBy` event
        pub fn agreement_liquidated_by_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementLiquidatedByFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AgreementLiquidatedV2` event
        pub fn agreement_liquidated_v2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementLiquidatedV2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AgreementStateUpdated` event
        pub fn agreement_state_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementStateUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AgreementTerminated` event
        pub fn agreement_terminated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementTerminatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AgreementUpdated` event
        pub fn agreement_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AgreementUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AuthorizedOperator` event
        pub fn authorized_operator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorizedOperatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Bailout` event
        pub fn bailout_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BailoutFilter> {
            self.0.event()
        }
        ///Gets the contract's `Burned` event
        pub fn burned_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnedFilter> {
            self.0.event()
        }
        ///Gets the contract's `CodeUpdated` event
        pub fn code_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CodeUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ConstantInflowNFTCreated` event
        pub fn constant_inflow_nft_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ConstantInflowNFTCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ConstantOutflowNFTCreated` event
        pub fn constant_outflow_nft_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ConstantOutflowNFTCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Minted` event
        pub fn minted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RevokedOperator` event
        pub fn revoked_operator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RevokedOperatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Sent` event
        pub fn sent_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SentFilter> {
            self.0.event()
        }
        ///Gets the contract's `TokenDowngraded` event
        pub fn token_downgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenDowngradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenUpgraded` event
        pub fn token_upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenUpgradedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, fDAIxEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for fDAIx<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `SF_TOKEN_AGREEMENT_ALREADY_EXISTS` with signature `SF_TOKEN_AGREEMENT_ALREADY_EXISTS()` and selector `0xf05521f6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SF_TOKEN_AGREEMENT_ALREADY_EXISTS",
        abi = "SF_TOKEN_AGREEMENT_ALREADY_EXISTS()"
    )]
    pub struct SF_TOKEN_AGREEMENT_ALREADY_EXISTS;
    ///Custom Error type `SF_TOKEN_AGREEMENT_DOES_NOT_EXIST` with signature `SF_TOKEN_AGREEMENT_DOES_NOT_EXIST()` and selector `0xdae18809`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SF_TOKEN_AGREEMENT_DOES_NOT_EXIST",
        abi = "SF_TOKEN_AGREEMENT_DOES_NOT_EXIST()"
    )]
    pub struct SF_TOKEN_AGREEMENT_DOES_NOT_EXIST;
    ///Custom Error type `SF_TOKEN_BURN_INSUFFICIENT_BALANCE` with signature `SF_TOKEN_BURN_INSUFFICIENT_BALANCE()` and selector `0x10ecdf44`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SF_TOKEN_BURN_INSUFFICIENT_BALANCE",
        abi = "SF_TOKEN_BURN_INSUFFICIENT_BALANCE()"
    )]
    pub struct SF_TOKEN_BURN_INSUFFICIENT_BALANCE;
    ///Custom Error type `SF_TOKEN_MOVE_INSUFFICIENT_BALANCE` with signature `SF_TOKEN_MOVE_INSUFFICIENT_BALANCE()` and selector `0x2f4cb941`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SF_TOKEN_MOVE_INSUFFICIENT_BALANCE",
        abi = "SF_TOKEN_MOVE_INSUFFICIENT_BALANCE()"
    )]
    pub struct SF_TOKEN_MOVE_INSUFFICIENT_BALANCE;
    ///Custom Error type `SF_TOKEN_ONLY_HOST` with signature `SF_TOKEN_ONLY_HOST()` and selector `0xc51efddd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SF_TOKEN_ONLY_HOST", abi = "SF_TOKEN_ONLY_HOST()")]
    pub struct SF_TOKEN_ONLY_HOST;
    ///Custom Error type `SF_TOKEN_ONLY_LISTED_AGREEMENT` with signature `SF_TOKEN_ONLY_LISTED_AGREEMENT()` and selector `0xc9ff6644`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SF_TOKEN_ONLY_LISTED_AGREEMENT",
        abi = "SF_TOKEN_ONLY_LISTED_AGREEMENT()"
    )]
    pub struct SF_TOKEN_ONLY_LISTED_AGREEMENT;
    ///Custom Error type `SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS` with signature `SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS()` and selector `0x81638627`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS",
        abi = "SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS()"
    )]
    pub struct SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS;
    ///Custom Error type `SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS` with signature `SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS()` and selector `0xdf070274`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS",
        abi = "SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS()"
    )]
    pub struct SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS;
    ///Custom Error type `SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS` with signature `SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS()` and selector `0xba2ab184`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS",
        abi = "SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS()"
    )]
    pub struct SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS;
    ///Custom Error type `SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER` with signature `SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER()` and selector `0xf7f02227`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER",
        abi = "SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER()"
    )]
    pub struct SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER;
    ///Custom Error type `SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED` with signature `SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED()` and selector `0xe3e13698`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED",
        abi = "SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED()"
    )]
    pub struct SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED;
    ///Custom Error type `SUPER_TOKEN_MINT_TO_ZERO_ADDRESS` with signature `SUPER_TOKEN_MINT_TO_ZERO_ADDRESS()` and selector `0x0d243157`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_MINT_TO_ZERO_ADDRESS",
        abi = "SUPER_TOKEN_MINT_TO_ZERO_ADDRESS()"
    )]
    pub struct SUPER_TOKEN_MINT_TO_ZERO_ADDRESS;
    ///Custom Error type `SUPER_TOKEN_NFT_PROXY_ALREADY_SET` with signature `SUPER_TOKEN_NFT_PROXY_ALREADY_SET()` and selector `0x6bef249d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_NFT_PROXY_ALREADY_SET",
        abi = "SUPER_TOKEN_NFT_PROXY_ALREADY_SET()"
    )]
    pub struct SUPER_TOKEN_NFT_PROXY_ALREADY_SET;
    ///Custom Error type `SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT` with signature `SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT()` and selector `0xfe737d05`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT",
        abi = "SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT()"
    )]
    pub struct SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT;
    ///Custom Error type `SUPER_TOKEN_NO_UNDERLYING_TOKEN` with signature `SUPER_TOKEN_NO_UNDERLYING_TOKEN()` and selector `0xf79cf656`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_NO_UNDERLYING_TOKEN",
        abi = "SUPER_TOKEN_NO_UNDERLYING_TOKEN()"
    )]
    pub struct SUPER_TOKEN_NO_UNDERLYING_TOKEN;
    ///Custom Error type `SUPER_TOKEN_ONLY_GOV_OWNER` with signature `SUPER_TOKEN_ONLY_GOV_OWNER()` and selector `0xd9c7ed08`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_ONLY_GOV_OWNER",
        abi = "SUPER_TOKEN_ONLY_GOV_OWNER()"
    )]
    pub struct SUPER_TOKEN_ONLY_GOV_OWNER;
    ///Custom Error type `SUPER_TOKEN_ONLY_HOST` with signature `SUPER_TOKEN_ONLY_HOST()` and selector `0x98f73704`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SUPER_TOKEN_ONLY_HOST", abi = "SUPER_TOKEN_ONLY_HOST()")]
    pub struct SUPER_TOKEN_ONLY_HOST;
    ///Custom Error type `SUPER_TOKEN_ONLY_SELF` with signature `SUPER_TOKEN_ONLY_SELF()` and selector `0x7ffa6648`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SUPER_TOKEN_ONLY_SELF", abi = "SUPER_TOKEN_ONLY_SELF()")]
    pub struct SUPER_TOKEN_ONLY_SELF;
    ///Custom Error type `SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS` with signature `SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS()` and selector `0xeecd6c9b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS",
        abi = "SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS()"
    )]
    pub struct SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS;
    ///Custom Error type `SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS` with signature `SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS()` and selector `0xe219bd39`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS",
        abi = "SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS()"
    )]
    pub struct SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum fDAIxErrors {
        SF_TOKEN_AGREEMENT_ALREADY_EXISTS(SF_TOKEN_AGREEMENT_ALREADY_EXISTS),
        SF_TOKEN_AGREEMENT_DOES_NOT_EXIST(SF_TOKEN_AGREEMENT_DOES_NOT_EXIST),
        SF_TOKEN_BURN_INSUFFICIENT_BALANCE(SF_TOKEN_BURN_INSUFFICIENT_BALANCE),
        SF_TOKEN_MOVE_INSUFFICIENT_BALANCE(SF_TOKEN_MOVE_INSUFFICIENT_BALANCE),
        SF_TOKEN_ONLY_HOST(SF_TOKEN_ONLY_HOST),
        SF_TOKEN_ONLY_LISTED_AGREEMENT(SF_TOKEN_ONLY_LISTED_AGREEMENT),
        SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS(SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS),
        SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS(SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS),
        SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS(SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS),
        SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER(
            SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER,
        ),
        SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED(
            SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED,
        ),
        SUPER_TOKEN_MINT_TO_ZERO_ADDRESS(SUPER_TOKEN_MINT_TO_ZERO_ADDRESS),
        SUPER_TOKEN_NFT_PROXY_ALREADY_SET(SUPER_TOKEN_NFT_PROXY_ALREADY_SET),
        SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT(SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT),
        SUPER_TOKEN_NO_UNDERLYING_TOKEN(SUPER_TOKEN_NO_UNDERLYING_TOKEN),
        SUPER_TOKEN_ONLY_GOV_OWNER(SUPER_TOKEN_ONLY_GOV_OWNER),
        SUPER_TOKEN_ONLY_HOST(SUPER_TOKEN_ONLY_HOST),
        SUPER_TOKEN_ONLY_SELF(SUPER_TOKEN_ONLY_SELF),
        SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS(SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS),
        SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS(SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for fDAIxErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <SF_TOKEN_AGREEMENT_ALREADY_EXISTS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SF_TOKEN_AGREEMENT_ALREADY_EXISTS(decoded));
            }
            if let Ok(decoded)
                = <SF_TOKEN_AGREEMENT_DOES_NOT_EXIST as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SF_TOKEN_AGREEMENT_DOES_NOT_EXIST(decoded));
            }
            if let Ok(decoded)
                = <SF_TOKEN_BURN_INSUFFICIENT_BALANCE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SF_TOKEN_BURN_INSUFFICIENT_BALANCE(decoded));
            }
            if let Ok(decoded)
                = <SF_TOKEN_MOVE_INSUFFICIENT_BALANCE as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SF_TOKEN_MOVE_INSUFFICIENT_BALANCE(decoded));
            }
            if let Ok(decoded)
                = <SF_TOKEN_ONLY_HOST as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SF_TOKEN_ONLY_HOST(decoded));
            }
            if let Ok(decoded)
                = <SF_TOKEN_ONLY_LISTED_AGREEMENT as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SF_TOKEN_ONLY_LISTED_AGREEMENT(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED(decoded),
                );
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_MINT_TO_ZERO_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_MINT_TO_ZERO_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_NFT_PROXY_ALREADY_SET as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_NFT_PROXY_ALREADY_SET(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_NO_UNDERLYING_TOKEN as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_NO_UNDERLYING_TOKEN(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_ONLY_GOV_OWNER as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_ONLY_GOV_OWNER(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_ONLY_HOST as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_ONLY_HOST(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_ONLY_SELF as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_ONLY_SELF(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS(decoded));
            }
            if let Ok(decoded)
                = <SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for fDAIxErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::SF_TOKEN_AGREEMENT_ALREADY_EXISTS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SF_TOKEN_AGREEMENT_DOES_NOT_EXIST(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SF_TOKEN_BURN_INSUFFICIENT_BALANCE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SF_TOKEN_MOVE_INSUFFICIENT_BALANCE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SF_TOKEN_ONLY_HOST(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SF_TOKEN_ONLY_LISTED_AGREEMENT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_MINT_TO_ZERO_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_NFT_PROXY_ALREADY_SET(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_NO_UNDERLYING_TOKEN(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_ONLY_GOV_OWNER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_ONLY_HOST(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_ONLY_SELF(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for fDAIxErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <SF_TOKEN_AGREEMENT_ALREADY_EXISTS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SF_TOKEN_AGREEMENT_DOES_NOT_EXIST as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SF_TOKEN_BURN_INSUFFICIENT_BALANCE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SF_TOKEN_MOVE_INSUFFICIENT_BALANCE as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SF_TOKEN_ONLY_HOST as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SF_TOKEN_ONLY_LISTED_AGREEMENT as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_MINT_TO_ZERO_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_NFT_PROXY_ALREADY_SET as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_NO_UNDERLYING_TOKEN as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_ONLY_GOV_OWNER as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_ONLY_HOST as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_ONLY_SELF as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for fDAIxErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SF_TOKEN_AGREEMENT_ALREADY_EXISTS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SF_TOKEN_AGREEMENT_DOES_NOT_EXIST(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SF_TOKEN_BURN_INSUFFICIENT_BALANCE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SF_TOKEN_MOVE_INSUFFICIENT_BALANCE(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SF_TOKEN_ONLY_HOST(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SF_TOKEN_ONLY_LISTED_AGREEMENT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_MINT_TO_ZERO_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_NFT_PROXY_ALREADY_SET(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_NO_UNDERLYING_TOKEN(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_ONLY_GOV_OWNER(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_ONLY_HOST(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_ONLY_SELF(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for fDAIxErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<SF_TOKEN_AGREEMENT_ALREADY_EXISTS> for fDAIxErrors {
        fn from(value: SF_TOKEN_AGREEMENT_ALREADY_EXISTS) -> Self {
            Self::SF_TOKEN_AGREEMENT_ALREADY_EXISTS(value)
        }
    }
    impl ::core::convert::From<SF_TOKEN_AGREEMENT_DOES_NOT_EXIST> for fDAIxErrors {
        fn from(value: SF_TOKEN_AGREEMENT_DOES_NOT_EXIST) -> Self {
            Self::SF_TOKEN_AGREEMENT_DOES_NOT_EXIST(value)
        }
    }
    impl ::core::convert::From<SF_TOKEN_BURN_INSUFFICIENT_BALANCE> for fDAIxErrors {
        fn from(value: SF_TOKEN_BURN_INSUFFICIENT_BALANCE) -> Self {
            Self::SF_TOKEN_BURN_INSUFFICIENT_BALANCE(value)
        }
    }
    impl ::core::convert::From<SF_TOKEN_MOVE_INSUFFICIENT_BALANCE> for fDAIxErrors {
        fn from(value: SF_TOKEN_MOVE_INSUFFICIENT_BALANCE) -> Self {
            Self::SF_TOKEN_MOVE_INSUFFICIENT_BALANCE(value)
        }
    }
    impl ::core::convert::From<SF_TOKEN_ONLY_HOST> for fDAIxErrors {
        fn from(value: SF_TOKEN_ONLY_HOST) -> Self {
            Self::SF_TOKEN_ONLY_HOST(value)
        }
    }
    impl ::core::convert::From<SF_TOKEN_ONLY_LISTED_AGREEMENT> for fDAIxErrors {
        fn from(value: SF_TOKEN_ONLY_LISTED_AGREEMENT) -> Self {
            Self::SF_TOKEN_ONLY_LISTED_AGREEMENT(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS) -> Self {
            Self::SUPER_TOKEN_APPROVE_FROM_ZERO_ADDRESS(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS) -> Self {
            Self::SUPER_TOKEN_APPROVE_TO_ZERO_ADDRESS(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS) -> Self {
            Self::SUPER_TOKEN_BURN_FROM_ZERO_ADDRESS(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER>
    for fDAIxErrors {
        fn from(value: SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER) -> Self {
            Self::SUPER_TOKEN_CALLER_IS_NOT_OPERATOR_FOR_HOLDER(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED>
    for fDAIxErrors {
        fn from(value: SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED) -> Self {
            Self::SUPER_TOKEN_INFLATIONARY_DEFLATIONARY_NOT_SUPPORTED(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_MINT_TO_ZERO_ADDRESS> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_MINT_TO_ZERO_ADDRESS) -> Self {
            Self::SUPER_TOKEN_MINT_TO_ZERO_ADDRESS(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_NFT_PROXY_ALREADY_SET> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_NFT_PROXY_ALREADY_SET) -> Self {
            Self::SUPER_TOKEN_NFT_PROXY_ALREADY_SET(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT) -> Self {
            Self::SUPER_TOKEN_NOT_ERC777_TOKENS_RECIPIENT(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_NO_UNDERLYING_TOKEN> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_NO_UNDERLYING_TOKEN) -> Self {
            Self::SUPER_TOKEN_NO_UNDERLYING_TOKEN(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_ONLY_GOV_OWNER> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_ONLY_GOV_OWNER) -> Self {
            Self::SUPER_TOKEN_ONLY_GOV_OWNER(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_ONLY_HOST> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_ONLY_HOST) -> Self {
            Self::SUPER_TOKEN_ONLY_HOST(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_ONLY_SELF> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_ONLY_SELF) -> Self {
            Self::SUPER_TOKEN_ONLY_SELF(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS) -> Self {
            Self::SUPER_TOKEN_TRANSFER_FROM_ZERO_ADDRESS(value)
        }
    }
    impl ::core::convert::From<SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS> for fDAIxErrors {
        fn from(value: SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS) -> Self {
            Self::SUPER_TOKEN_TRANSFER_TO_ZERO_ADDRESS(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AgreementCreated",
        abi = "AgreementCreated(address,bytes32,bytes32[])"
    )]
    pub struct AgreementCreatedFilter {
        #[ethevent(indexed)]
        pub agreement_class: ::ethers::core::types::Address,
        pub id: [u8; 32],
        pub data: ::std::vec::Vec<[u8; 32]>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AgreementLiquidated",
        abi = "AgreementLiquidated(address,bytes32,address,address,uint256)"
    )]
    pub struct AgreementLiquidatedFilter {
        #[ethevent(indexed)]
        pub agreement_class: ::ethers::core::types::Address,
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub penalty_account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward_account: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AgreementLiquidatedBy",
        abi = "AgreementLiquidatedBy(address,address,bytes32,address,address,uint256,uint256)"
    )]
    pub struct AgreementLiquidatedByFilter {
        pub liquidator_account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub agreement_class: ::ethers::core::types::Address,
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub penalty_account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub bond_account: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub bailout_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AgreementLiquidatedV2",
        abi = "AgreementLiquidatedV2(address,bytes32,address,address,address,uint256,int256,bytes)"
    )]
    pub struct AgreementLiquidatedV2Filter {
        #[ethevent(indexed)]
        pub agreement_class: ::ethers::core::types::Address,
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub liquidator_account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub target_account: ::ethers::core::types::Address,
        pub reward_amount_receiver: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub target_account_balance_delta: ::ethers::core::types::I256,
        pub liquidation_type_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AgreementStateUpdated",
        abi = "AgreementStateUpdated(address,address,uint256)"
    )]
    pub struct AgreementStateUpdatedFilter {
        #[ethevent(indexed)]
        pub agreement_class: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub slot_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AgreementTerminated",
        abi = "AgreementTerminated(address,bytes32)"
    )]
    pub struct AgreementTerminatedFilter {
        #[ethevent(indexed)]
        pub agreement_class: ::ethers::core::types::Address,
        pub id: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "AgreementUpdated",
        abi = "AgreementUpdated(address,bytes32,bytes32[])"
    )]
    pub struct AgreementUpdatedFilter {
        #[ethevent(indexed)]
        pub agreement_class: ::ethers::core::types::Address,
        pub id: [u8; 32],
        pub data: ::std::vec::Vec<[u8; 32]>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "AuthorizedOperator", abi = "AuthorizedOperator(address,address)")]
    pub struct AuthorizedOperatorFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Bailout", abi = "Bailout(address,uint256)")]
    pub struct BailoutFilter {
        #[ethevent(indexed)]
        pub bailout_account: ::ethers::core::types::Address,
        pub bailout_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Burned", abi = "Burned(address,address,uint256,bytes,bytes)")]
    pub struct BurnedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "CodeUpdated", abi = "CodeUpdated(bytes32,address)")]
    pub struct CodeUpdatedFilter {
        pub uuid: [u8; 32],
        pub code_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ConstantInflowNFTCreated",
        abi = "ConstantInflowNFTCreated(address)"
    )]
    pub struct ConstantInflowNFTCreatedFilter {
        #[ethevent(indexed)]
        pub constant_inflow_nft: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ConstantOutflowNFTCreated",
        abi = "ConstantOutflowNFTCreated(address)"
    )]
    pub struct ConstantOutflowNFTCreatedFilter {
        #[ethevent(indexed)]
        pub constant_outflow_nft: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Minted", abi = "Minted(address,address,uint256,bytes,bytes)")]
    pub struct MintedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RevokedOperator", abi = "RevokedOperator(address,address)")]
    pub struct RevokedOperatorFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Sent", abi = "Sent(address,address,address,uint256,bytes,bytes)")]
    pub struct SentFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "TokenDowngraded", abi = "TokenDowngraded(address,uint256)")]
    pub struct TokenDowngradedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "TokenUpgraded", abi = "TokenUpgraded(address,uint256)")]
    pub struct TokenUpgradedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum fDAIxEvents {
        AgreementCreatedFilter(AgreementCreatedFilter),
        AgreementLiquidatedFilter(AgreementLiquidatedFilter),
        AgreementLiquidatedByFilter(AgreementLiquidatedByFilter),
        AgreementLiquidatedV2Filter(AgreementLiquidatedV2Filter),
        AgreementStateUpdatedFilter(AgreementStateUpdatedFilter),
        AgreementTerminatedFilter(AgreementTerminatedFilter),
        AgreementUpdatedFilter(AgreementUpdatedFilter),
        ApprovalFilter(ApprovalFilter),
        AuthorizedOperatorFilter(AuthorizedOperatorFilter),
        BailoutFilter(BailoutFilter),
        BurnedFilter(BurnedFilter),
        CodeUpdatedFilter(CodeUpdatedFilter),
        ConstantInflowNFTCreatedFilter(ConstantInflowNFTCreatedFilter),
        ConstantOutflowNFTCreatedFilter(ConstantOutflowNFTCreatedFilter),
        InitializedFilter(InitializedFilter),
        MintedFilter(MintedFilter),
        RevokedOperatorFilter(RevokedOperatorFilter),
        SentFilter(SentFilter),
        TokenDowngradedFilter(TokenDowngradedFilter),
        TokenUpgradedFilter(TokenUpgradedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for fDAIxEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AgreementCreatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::AgreementCreatedFilter(decoded));
            }
            if let Ok(decoded) = AgreementLiquidatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::AgreementLiquidatedFilter(decoded));
            }
            if let Ok(decoded) = AgreementLiquidatedByFilter::decode_log(log) {
                return Ok(fDAIxEvents::AgreementLiquidatedByFilter(decoded));
            }
            if let Ok(decoded) = AgreementLiquidatedV2Filter::decode_log(log) {
                return Ok(fDAIxEvents::AgreementLiquidatedV2Filter(decoded));
            }
            if let Ok(decoded) = AgreementStateUpdatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::AgreementStateUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AgreementTerminatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::AgreementTerminatedFilter(decoded));
            }
            if let Ok(decoded) = AgreementUpdatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::AgreementUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(fDAIxEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = AuthorizedOperatorFilter::decode_log(log) {
                return Ok(fDAIxEvents::AuthorizedOperatorFilter(decoded));
            }
            if let Ok(decoded) = BailoutFilter::decode_log(log) {
                return Ok(fDAIxEvents::BailoutFilter(decoded));
            }
            if let Ok(decoded) = BurnedFilter::decode_log(log) {
                return Ok(fDAIxEvents::BurnedFilter(decoded));
            }
            if let Ok(decoded) = CodeUpdatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::CodeUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ConstantInflowNFTCreatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::ConstantInflowNFTCreatedFilter(decoded));
            }
            if let Ok(decoded) = ConstantOutflowNFTCreatedFilter::decode_log(log) {
                return Ok(fDAIxEvents::ConstantOutflowNFTCreatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(fDAIxEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MintedFilter::decode_log(log) {
                return Ok(fDAIxEvents::MintedFilter(decoded));
            }
            if let Ok(decoded) = RevokedOperatorFilter::decode_log(log) {
                return Ok(fDAIxEvents::RevokedOperatorFilter(decoded));
            }
            if let Ok(decoded) = SentFilter::decode_log(log) {
                return Ok(fDAIxEvents::SentFilter(decoded));
            }
            if let Ok(decoded) = TokenDowngradedFilter::decode_log(log) {
                return Ok(fDAIxEvents::TokenDowngradedFilter(decoded));
            }
            if let Ok(decoded) = TokenUpgradedFilter::decode_log(log) {
                return Ok(fDAIxEvents::TokenUpgradedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(fDAIxEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for fDAIxEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AgreementCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AgreementLiquidatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AgreementLiquidatedByFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AgreementLiquidatedV2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AgreementStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AgreementTerminatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AgreementUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizedOperatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BailoutFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CodeUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConstantInflowNFTCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConstantOutflowNFTCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokedOperatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenDowngradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenUpgradedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AgreementCreatedFilter> for fDAIxEvents {
        fn from(value: AgreementCreatedFilter) -> Self {
            Self::AgreementCreatedFilter(value)
        }
    }
    impl ::core::convert::From<AgreementLiquidatedFilter> for fDAIxEvents {
        fn from(value: AgreementLiquidatedFilter) -> Self {
            Self::AgreementLiquidatedFilter(value)
        }
    }
    impl ::core::convert::From<AgreementLiquidatedByFilter> for fDAIxEvents {
        fn from(value: AgreementLiquidatedByFilter) -> Self {
            Self::AgreementLiquidatedByFilter(value)
        }
    }
    impl ::core::convert::From<AgreementLiquidatedV2Filter> for fDAIxEvents {
        fn from(value: AgreementLiquidatedV2Filter) -> Self {
            Self::AgreementLiquidatedV2Filter(value)
        }
    }
    impl ::core::convert::From<AgreementStateUpdatedFilter> for fDAIxEvents {
        fn from(value: AgreementStateUpdatedFilter) -> Self {
            Self::AgreementStateUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AgreementTerminatedFilter> for fDAIxEvents {
        fn from(value: AgreementTerminatedFilter) -> Self {
            Self::AgreementTerminatedFilter(value)
        }
    }
    impl ::core::convert::From<AgreementUpdatedFilter> for fDAIxEvents {
        fn from(value: AgreementUpdatedFilter) -> Self {
            Self::AgreementUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for fDAIxEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<AuthorizedOperatorFilter> for fDAIxEvents {
        fn from(value: AuthorizedOperatorFilter) -> Self {
            Self::AuthorizedOperatorFilter(value)
        }
    }
    impl ::core::convert::From<BailoutFilter> for fDAIxEvents {
        fn from(value: BailoutFilter) -> Self {
            Self::BailoutFilter(value)
        }
    }
    impl ::core::convert::From<BurnedFilter> for fDAIxEvents {
        fn from(value: BurnedFilter) -> Self {
            Self::BurnedFilter(value)
        }
    }
    impl ::core::convert::From<CodeUpdatedFilter> for fDAIxEvents {
        fn from(value: CodeUpdatedFilter) -> Self {
            Self::CodeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ConstantInflowNFTCreatedFilter> for fDAIxEvents {
        fn from(value: ConstantInflowNFTCreatedFilter) -> Self {
            Self::ConstantInflowNFTCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ConstantOutflowNFTCreatedFilter> for fDAIxEvents {
        fn from(value: ConstantOutflowNFTCreatedFilter) -> Self {
            Self::ConstantOutflowNFTCreatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for fDAIxEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MintedFilter> for fDAIxEvents {
        fn from(value: MintedFilter) -> Self {
            Self::MintedFilter(value)
        }
    }
    impl ::core::convert::From<RevokedOperatorFilter> for fDAIxEvents {
        fn from(value: RevokedOperatorFilter) -> Self {
            Self::RevokedOperatorFilter(value)
        }
    }
    impl ::core::convert::From<SentFilter> for fDAIxEvents {
        fn from(value: SentFilter) -> Self {
            Self::SentFilter(value)
        }
    }
    impl ::core::convert::From<TokenDowngradedFilter> for fDAIxEvents {
        fn from(value: TokenDowngradedFilter) -> Self {
            Self::TokenDowngradedFilter(value)
        }
    }
    impl ::core::convert::From<TokenUpgradedFilter> for fDAIxEvents {
        fn from(value: TokenUpgradedFilter) -> Self {
            Self::TokenUpgradedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for fDAIxEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `CONSTANT_INFLOW_NFT_LOGIC` function with signature `CONSTANT_INFLOW_NFT_LOGIC()` and selector `0x5a7792df`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "CONSTANT_INFLOW_NFT_LOGIC", abi = "CONSTANT_INFLOW_NFT_LOGIC()")]
    pub struct ConstantInflowNftLogicCall;
    ///Container type for all input parameters for the `CONSTANT_OUTFLOW_NFT_LOGIC` function with signature `CONSTANT_OUTFLOW_NFT_LOGIC()` and selector `0x5365d19f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "CONSTANT_OUTFLOW_NFT_LOGIC", abi = "CONSTANT_OUTFLOW_NFT_LOGIC()")]
    pub struct ConstantOutflowNftLogicCall;
    ///Container type for all input parameters for the `SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS` function with signature `SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS()` and selector `0x70d75815`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS",
        abi = "SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS()"
    )]
    pub struct SuperfluidNftDeployerLibraryAddressCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub account: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `authorizeOperator` function with signature `authorizeOperator(address)` and selector `0x959b8c3f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "authorizeOperator", abi = "authorizeOperator(address)")]
    pub struct AuthorizeOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256,bytes)` and selector `0xfe9d9303`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "burn", abi = "burn(uint256,bytes)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `castrate` function with signature `castrate()` and selector `0x9903ad38`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "castrate", abi = "castrate()")]
    pub struct CastrateCall;
    ///Container type for all input parameters for the `constantInflowNFT` function with signature `constantInflowNFT()` and selector `0xd50911cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "constantInflowNFT", abi = "constantInflowNFT()")]
    pub struct ConstantInflowNFTCall;
    ///Container type for all input parameters for the `constantOutflowNFT` function with signature `constantOutflowNFT()` and selector `0x0d9c12b5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "constantOutflowNFT", abi = "constantOutflowNFT()")]
    pub struct ConstantOutflowNFTCall;
    ///Container type for all input parameters for the `createAgreement` function with signature `createAgreement(bytes32,bytes32[])` and selector `0x12a6a3f8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createAgreement", abi = "createAgreement(bytes32,bytes32[])")]
    pub struct CreateAgreementCall {
        pub id: [u8; 32],
        pub data: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `defaultOperators` function with signature `defaultOperators()` and selector `0x06e48538`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "defaultOperators", abi = "defaultOperators()")]
    pub struct DefaultOperatorsCall;
    ///Container type for all input parameters for the `downgrade` function with signature `downgrade(uint256)` and selector `0x11bcc81e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "downgrade", abi = "downgrade(uint256)")]
    pub struct DowngradeCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `downgradeTo` function with signature `downgradeTo(address,uint256)` and selector `0x83ba2525`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "downgradeTo", abi = "downgradeTo(address,uint256)")]
    pub struct DowngradeToCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAccountActiveAgreements` function with signature `getAccountActiveAgreements(address)` and selector `0x386fa221`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getAccountActiveAgreements",
        abi = "getAccountActiveAgreements(address)"
    )]
    pub struct GetAccountActiveAgreementsCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAgreementData` function with signature `getAgreementData(address,bytes32,uint256)` and selector `0x6c2d9f2f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getAgreementData",
        abi = "getAgreementData(address,bytes32,uint256)"
    )]
    pub struct GetAgreementDataCall {
        pub agreement_class: ::ethers::core::types::Address,
        pub id: [u8; 32],
        pub data_length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getAgreementStateSlot` function with signature `getAgreementStateSlot(address,address,uint256,uint256)` and selector `0x4b61cc33`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getAgreementStateSlot",
        abi = "getAgreementStateSlot(address,address,uint256,uint256)"
    )]
    pub struct GetAgreementStateSlotCall {
        pub agreement_class: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub slot_id: ::ethers::core::types::U256,
        pub data_length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getCodeAddress` function with signature `getCodeAddress()` and selector `0x50d75d25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCodeAddress", abi = "getCodeAddress()")]
    pub struct GetCodeAddressCall;
    ///Container type for all input parameters for the `getHost` function with signature `getHost()` and selector `0x20bc4425`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getHost", abi = "getHost()")]
    pub struct GetHostCall;
    ///Container type for all input parameters for the `getUnderlyingToken` function with signature `getUnderlyingToken()` and selector `0xee719bc8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getUnderlyingToken", abi = "getUnderlyingToken()")]
    pub struct GetUnderlyingTokenCall;
    ///Container type for all input parameters for the `granularity` function with signature `granularity()` and selector `0x556f0dc7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "granularity", abi = "granularity()")]
    pub struct GranularityCall;
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,uint8,string,string)` and selector `0x42fe0980`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,uint8,string,string)")]
    pub struct InitializeCall {
        pub underlying_token: ::ethers::core::types::Address,
        pub underlying_decimals: u8,
        pub n: ::std::string::String,
        pub s: ::std::string::String,
    }
    ///Container type for all input parameters for the `isAccountCritical` function with signature `isAccountCritical(address,uint256)` and selector `0xd9d078d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isAccountCritical", abi = "isAccountCritical(address,uint256)")]
    pub struct IsAccountCriticalCall {
        pub account: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isAccountCriticalNow` function with signature `isAccountCriticalNow(address)` and selector `0x79359f6f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isAccountCriticalNow", abi = "isAccountCriticalNow(address)")]
    pub struct IsAccountCriticalNowCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isAccountSolvent` function with signature `isAccountSolvent(address,uint256)` and selector `0xb84cdd4a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isAccountSolvent", abi = "isAccountSolvent(address,uint256)")]
    pub struct IsAccountSolventCall {
        pub account: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isAccountSolventNow` function with signature `isAccountSolventNow(address)` and selector `0xbb0d196e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isAccountSolventNow", abi = "isAccountSolventNow(address)")]
    pub struct IsAccountSolventNowCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOperatorFor` function with signature `isOperatorFor(address,address)` and selector `0xd95b6371`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isOperatorFor", abi = "isOperatorFor(address,address)")]
    pub struct IsOperatorForCall {
        pub operator: ::ethers::core::types::Address,
        pub token_holder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `makeLiquidationPayoutsV2` function with signature `makeLiquidationPayoutsV2(bytes32,bytes,address,bool,address,uint256,int256)` and selector `0x1863e809`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "makeLiquidationPayoutsV2",
        abi = "makeLiquidationPayoutsV2(bytes32,bytes,address,bool,address,uint256,int256)"
    )]
    pub struct MakeLiquidationPayoutsV2Call {
        pub id: [u8; 32],
        pub liquidation_type_data: ::ethers::core::types::Bytes,
        pub liquidator_account: ::ethers::core::types::Address,
        pub use_default_reward_account: bool,
        pub target_account: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub target_account_balance_delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `operationApprove` function with signature `operationApprove(address,address,uint256)` and selector `0x62aa5287`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "operationApprove",
        abi = "operationApprove(address,address,uint256)"
    )]
    pub struct OperationApproveCall {
        pub account: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operationDecreaseAllowance` function with signature `operationDecreaseAllowance(address,address,uint256)` and selector `0xc780fd82`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "operationDecreaseAllowance",
        abi = "operationDecreaseAllowance(address,address,uint256)"
    )]
    pub struct OperationDecreaseAllowanceCall {
        pub account: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operationDowngrade` function with signature `operationDowngrade(address,uint256)` and selector `0x245887fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "operationDowngrade", abi = "operationDowngrade(address,uint256)")]
    pub struct OperationDowngradeCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operationIncreaseAllowance` function with signature `operationIncreaseAllowance(address,address,uint256)` and selector `0x4b2763b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "operationIncreaseAllowance",
        abi = "operationIncreaseAllowance(address,address,uint256)"
    )]
    pub struct OperationIncreaseAllowanceCall {
        pub account: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operationSend` function with signature `operationSend(address,address,uint256,bytes)` and selector `0xca0c1e7f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "operationSend",
        abi = "operationSend(address,address,uint256,bytes)"
    )]
    pub struct OperationSendCall {
        pub spender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `operationTransferFrom` function with signature `operationTransferFrom(address,address,address,uint256)` and selector `0x16d055d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "operationTransferFrom",
        abi = "operationTransferFrom(address,address,address,uint256)"
    )]
    pub struct OperationTransferFromCall {
        pub account: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operationUpgrade` function with signature `operationUpgrade(address,uint256)` and selector `0xca789464`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "operationUpgrade", abi = "operationUpgrade(address,uint256)")]
    pub struct OperationUpgradeCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operatorBurn` function with signature `operatorBurn(address,uint256,bytes,bytes)` and selector `0xfc673c4f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "operatorBurn", abi = "operatorBurn(address,uint256,bytes,bytes)")]
    pub struct OperatorBurnCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `operatorSend` function with signature `operatorSend(address,address,uint256,bytes,bytes)` and selector `0x62ad1b83`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "operatorSend",
        abi = "operatorSend(address,address,uint256,bytes,bytes)"
    )]
    pub struct OperatorSendCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub operator_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `poolAdminNFT` function with signature `poolAdminNFT()` and selector `0xc76058fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "poolAdminNFT", abi = "poolAdminNFT()")]
    pub struct PoolAdminNFTCall;
    ///Container type for all input parameters for the `poolMemberNFT` function with signature `poolMemberNFT()` and selector `0xc4b1584c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "poolMemberNFT", abi = "poolMemberNFT()")]
    pub struct PoolMemberNFTCall;
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `realtimeBalanceOf` function with signature `realtimeBalanceOf(address,uint256)` and selector `0xeb3537cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "realtimeBalanceOf", abi = "realtimeBalanceOf(address,uint256)")]
    pub struct RealtimeBalanceOfCall {
        pub account: ::ethers::core::types::Address,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `realtimeBalanceOfNow` function with signature `realtimeBalanceOfNow(address)` and selector `0x2ec8eec7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "realtimeBalanceOfNow", abi = "realtimeBalanceOfNow(address)")]
    pub struct RealtimeBalanceOfNowCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeOperator` function with signature `revokeOperator(address)` and selector `0xfad8b32a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokeOperator", abi = "revokeOperator(address)")]
    pub struct RevokeOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `selfApproveFor` function with signature `selfApproveFor(address,address,uint256)` and selector `0x66a12fb6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "selfApproveFor", abi = "selfApproveFor(address,address,uint256)")]
    pub struct SelfApproveForCall {
        pub account: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `selfBurn` function with signature `selfBurn(address,uint256,bytes)` and selector `0x9d876741`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "selfBurn", abi = "selfBurn(address,uint256,bytes)")]
    pub struct SelfBurnCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `selfMint` function with signature `selfMint(address,uint256,bytes)` and selector `0xc68d4283`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "selfMint", abi = "selfMint(address,uint256,bytes)")]
    pub struct SelfMintCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `selfTransferFrom` function with signature `selfTransferFrom(address,address,address,uint256)` and selector `0x41b706be`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "selfTransferFrom",
        abi = "selfTransferFrom(address,address,address,uint256)"
    )]
    pub struct SelfTransferFromCall {
        pub holder: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `send` function with signature `send(address,uint256,bytes)` and selector `0x9bd9bbc6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "send", abi = "send(address,uint256,bytes)")]
    pub struct SendCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `settleBalance` function with signature `settleBalance(address,int256)` and selector `0xcf97256d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "settleBalance", abi = "settleBalance(address,int256)")]
    pub struct SettleBalanceCall {
        pub account: ::ethers::core::types::Address,
        pub delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `terminateAgreement` function with signature `terminateAgreement(bytes32,uint256)` and selector `0x27048397`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "terminateAgreement", abi = "terminateAgreement(bytes32,uint256)")]
    pub struct TerminateAgreementCall {
        pub id: [u8; 32],
        pub data_length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferAll` function with signature `transferAll(address)` and selector `0xa3a7e7f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferAll", abi = "transferAll(address)")]
    pub struct TransferAllCall {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub holder: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateAgreementData` function with signature `updateAgreementData(bytes32,bytes32[])` and selector `0xa1b2bf8b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "updateAgreementData",
        abi = "updateAgreementData(bytes32,bytes32[])"
    )]
    pub struct UpdateAgreementDataCall {
        pub id: [u8; 32],
        pub data: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `updateAgreementStateSlot` function with signature `updateAgreementStateSlot(address,uint256,bytes32[])` and selector `0x090c415e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "updateAgreementStateSlot",
        abi = "updateAgreementStateSlot(address,uint256,bytes32[])"
    )]
    pub struct UpdateAgreementStateSlotCall {
        pub account: ::ethers::core::types::Address,
        pub slot_id: ::ethers::core::types::U256,
        pub slot_data: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `updateCode` function with signature `updateCode(address)` and selector `0x46951954`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateCode", abi = "updateCode(address)")]
    pub struct UpdateCodeCall {
        pub new_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(uint256)` and selector `0x45977d03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "upgrade", abi = "upgrade(uint256)")]
    pub struct UpgradeCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `upgradeTo` function with signature `upgradeTo(address,uint256,bytes)` and selector `0x5b9d09cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "upgradeTo", abi = "upgradeTo(address,uint256,bytes)")]
    pub struct UpgradeToCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum fDAIxCalls {
        ConstantInflowNftLogic(ConstantInflowNftLogicCall),
        ConstantOutflowNftLogic(ConstantOutflowNftLogicCall),
        SuperfluidNftDeployerLibraryAddress(SuperfluidNftDeployerLibraryAddressCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        AuthorizeOperator(AuthorizeOperatorCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Castrate(CastrateCall),
        ConstantInflowNFT(ConstantInflowNFTCall),
        ConstantOutflowNFT(ConstantOutflowNFTCall),
        CreateAgreement(CreateAgreementCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DefaultOperators(DefaultOperatorsCall),
        Downgrade(DowngradeCall),
        DowngradeTo(DowngradeToCall),
        GetAccountActiveAgreements(GetAccountActiveAgreementsCall),
        GetAgreementData(GetAgreementDataCall),
        GetAgreementStateSlot(GetAgreementStateSlotCall),
        GetCodeAddress(GetCodeAddressCall),
        GetHost(GetHostCall),
        GetUnderlyingToken(GetUnderlyingTokenCall),
        Granularity(GranularityCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        IsAccountCritical(IsAccountCriticalCall),
        IsAccountCriticalNow(IsAccountCriticalNowCall),
        IsAccountSolvent(IsAccountSolventCall),
        IsAccountSolventNow(IsAccountSolventNowCall),
        IsOperatorFor(IsOperatorForCall),
        MakeLiquidationPayoutsV2(MakeLiquidationPayoutsV2Call),
        Name(NameCall),
        OperationApprove(OperationApproveCall),
        OperationDecreaseAllowance(OperationDecreaseAllowanceCall),
        OperationDowngrade(OperationDowngradeCall),
        OperationIncreaseAllowance(OperationIncreaseAllowanceCall),
        OperationSend(OperationSendCall),
        OperationTransferFrom(OperationTransferFromCall),
        OperationUpgrade(OperationUpgradeCall),
        OperatorBurn(OperatorBurnCall),
        OperatorSend(OperatorSendCall),
        PoolAdminNFT(PoolAdminNFTCall),
        PoolMemberNFT(PoolMemberNFTCall),
        ProxiableUUID(ProxiableUUIDCall),
        RealtimeBalanceOf(RealtimeBalanceOfCall),
        RealtimeBalanceOfNow(RealtimeBalanceOfNowCall),
        RevokeOperator(RevokeOperatorCall),
        SelfApproveFor(SelfApproveForCall),
        SelfBurn(SelfBurnCall),
        SelfMint(SelfMintCall),
        SelfTransferFrom(SelfTransferFromCall),
        Send(SendCall),
        SettleBalance(SettleBalanceCall),
        Symbol(SymbolCall),
        TerminateAgreement(TerminateAgreementCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferAll(TransferAllCall),
        TransferFrom(TransferFromCall),
        UpdateAgreementData(UpdateAgreementDataCall),
        UpdateAgreementStateSlot(UpdateAgreementStateSlotCall),
        UpdateCode(UpdateCodeCall),
        Upgrade(UpgradeCall),
        UpgradeTo(UpgradeToCall),
    }
    impl ::ethers::core::abi::AbiDecode for fDAIxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ConstantInflowNftLogicCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConstantInflowNftLogic(decoded));
            }
            if let Ok(decoded)
                = <ConstantOutflowNftLogicCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConstantOutflowNftLogic(decoded));
            }
            if let Ok(decoded)
                = <SuperfluidNftDeployerLibraryAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SuperfluidNftDeployerLibraryAddress(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <AuthorizeOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AuthorizeOperator(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <CastrateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Castrate(decoded));
            }
            if let Ok(decoded)
                = <ConstantInflowNFTCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConstantInflowNFT(decoded));
            }
            if let Ok(decoded)
                = <ConstantOutflowNFTCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConstantOutflowNFT(decoded));
            }
            if let Ok(decoded)
                = <CreateAgreementCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateAgreement(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <DefaultOperatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultOperators(decoded));
            }
            if let Ok(decoded)
                = <DowngradeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Downgrade(decoded));
            }
            if let Ok(decoded)
                = <DowngradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DowngradeTo(decoded));
            }
            if let Ok(decoded)
                = <GetAccountActiveAgreementsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAccountActiveAgreements(decoded));
            }
            if let Ok(decoded)
                = <GetAgreementDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAgreementData(decoded));
            }
            if let Ok(decoded)
                = <GetAgreementStateSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAgreementStateSlot(decoded));
            }
            if let Ok(decoded)
                = <GetCodeAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCodeAddress(decoded));
            }
            if let Ok(decoded)
                = <GetHostCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetHost(decoded));
            }
            if let Ok(decoded)
                = <GetUnderlyingTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUnderlyingToken(decoded));
            }
            if let Ok(decoded)
                = <GranularityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Granularity(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <IsAccountCriticalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAccountCritical(decoded));
            }
            if let Ok(decoded)
                = <IsAccountCriticalNowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAccountCriticalNow(decoded));
            }
            if let Ok(decoded)
                = <IsAccountSolventCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAccountSolvent(decoded));
            }
            if let Ok(decoded)
                = <IsAccountSolventNowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsAccountSolventNow(decoded));
            }
            if let Ok(decoded)
                = <IsOperatorForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOperatorFor(decoded));
            }
            if let Ok(decoded)
                = <MakeLiquidationPayoutsV2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MakeLiquidationPayoutsV2(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OperationApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationApprove(decoded));
            }
            if let Ok(decoded)
                = <OperationDecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationDecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <OperationDowngradeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationDowngrade(decoded));
            }
            if let Ok(decoded)
                = <OperationIncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationIncreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <OperationSendCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OperationSend(decoded));
            }
            if let Ok(decoded)
                = <OperationTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <OperationUpgradeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OperationUpgrade(decoded));
            }
            if let Ok(decoded)
                = <OperatorBurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OperatorBurn(decoded));
            }
            if let Ok(decoded)
                = <OperatorSendCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OperatorSend(decoded));
            }
            if let Ok(decoded)
                = <PoolAdminNFTCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolAdminNFT(decoded));
            }
            if let Ok(decoded)
                = <PoolMemberNFTCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolMemberNFT(decoded));
            }
            if let Ok(decoded)
                = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded)
                = <RealtimeBalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RealtimeBalanceOf(decoded));
            }
            if let Ok(decoded)
                = <RealtimeBalanceOfNowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RealtimeBalanceOfNow(decoded));
            }
            if let Ok(decoded)
                = <RevokeOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeOperator(decoded));
            }
            if let Ok(decoded)
                = <SelfApproveForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SelfApproveFor(decoded));
            }
            if let Ok(decoded)
                = <SelfBurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SelfBurn(decoded));
            }
            if let Ok(decoded)
                = <SelfMintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SelfMint(decoded));
            }
            if let Ok(decoded)
                = <SelfTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SelfTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <SendCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Send(decoded));
            }
            if let Ok(decoded)
                = <SettleBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SettleBalance(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TerminateAgreementCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TerminateAgreement(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferAll(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <UpdateAgreementDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateAgreementData(decoded));
            }
            if let Ok(decoded)
                = <UpdateAgreementStateSlotCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateAgreementStateSlot(decoded));
            }
            if let Ok(decoded)
                = <UpdateCodeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateCode(decoded));
            }
            if let Ok(decoded)
                = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Upgrade(decoded));
            }
            if let Ok(decoded)
                = <UpgradeToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpgradeTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for fDAIxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConstantInflowNftLogic(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConstantOutflowNftLogic(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SuperfluidNftDeployerLibraryAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AuthorizeOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Castrate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConstantInflowNFT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConstantOutflowNFT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateAgreement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultOperators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Downgrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DowngradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAccountActiveAgreements(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAgreementData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAgreementStateSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCodeAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHost(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetUnderlyingToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Granularity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAccountCritical(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAccountCriticalNow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAccountSolvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAccountSolventNow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperatorFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakeLiquidationPayoutsV2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperationApprove(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationDecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationDowngrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationIncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationSend(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperationUpgrade(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSend(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolAdminNFT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolMemberNFT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RealtimeBalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RealtimeBalanceOfNow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfApproveFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Send(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SettleBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TerminateAgreement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAgreementData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAgreementStateSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for fDAIxCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConstantInflowNftLogic(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConstantOutflowNftLogic(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SuperfluidNftDeployerLibraryAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::AuthorizeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Castrate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConstantInflowNFT(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConstantOutflowNFT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateAgreement(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultOperators(element) => ::core::fmt::Display::fmt(element, f),
                Self::Downgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::DowngradeTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAccountActiveAgreements(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAgreementData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAgreementStateSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCodeAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHost(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUnderlyingToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Granularity(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAccountCritical(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAccountCriticalNow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsAccountSolvent(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAccountSolventNow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsOperatorFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakeLiquidationPayoutsV2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperationApprove(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperationDecreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperationDowngrade(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperationIncreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperationSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperationTransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperationUpgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolAdminNFT(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolMemberNFT(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RealtimeBalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::RealtimeBalanceOfNow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfApproveFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Send(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TerminateAgreement(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAgreementData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateAgreementStateSlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConstantInflowNftLogicCall> for fDAIxCalls {
        fn from(value: ConstantInflowNftLogicCall) -> Self {
            Self::ConstantInflowNftLogic(value)
        }
    }
    impl ::core::convert::From<ConstantOutflowNftLogicCall> for fDAIxCalls {
        fn from(value: ConstantOutflowNftLogicCall) -> Self {
            Self::ConstantOutflowNftLogic(value)
        }
    }
    impl ::core::convert::From<SuperfluidNftDeployerLibraryAddressCall> for fDAIxCalls {
        fn from(value: SuperfluidNftDeployerLibraryAddressCall) -> Self {
            Self::SuperfluidNftDeployerLibraryAddress(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for fDAIxCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for fDAIxCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AuthorizeOperatorCall> for fDAIxCalls {
        fn from(value: AuthorizeOperatorCall) -> Self {
            Self::AuthorizeOperator(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for fDAIxCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for fDAIxCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<CastrateCall> for fDAIxCalls {
        fn from(value: CastrateCall) -> Self {
            Self::Castrate(value)
        }
    }
    impl ::core::convert::From<ConstantInflowNFTCall> for fDAIxCalls {
        fn from(value: ConstantInflowNFTCall) -> Self {
            Self::ConstantInflowNFT(value)
        }
    }
    impl ::core::convert::From<ConstantOutflowNFTCall> for fDAIxCalls {
        fn from(value: ConstantOutflowNFTCall) -> Self {
            Self::ConstantOutflowNFT(value)
        }
    }
    impl ::core::convert::From<CreateAgreementCall> for fDAIxCalls {
        fn from(value: CreateAgreementCall) -> Self {
            Self::CreateAgreement(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for fDAIxCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for fDAIxCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DefaultOperatorsCall> for fDAIxCalls {
        fn from(value: DefaultOperatorsCall) -> Self {
            Self::DefaultOperators(value)
        }
    }
    impl ::core::convert::From<DowngradeCall> for fDAIxCalls {
        fn from(value: DowngradeCall) -> Self {
            Self::Downgrade(value)
        }
    }
    impl ::core::convert::From<DowngradeToCall> for fDAIxCalls {
        fn from(value: DowngradeToCall) -> Self {
            Self::DowngradeTo(value)
        }
    }
    impl ::core::convert::From<GetAccountActiveAgreementsCall> for fDAIxCalls {
        fn from(value: GetAccountActiveAgreementsCall) -> Self {
            Self::GetAccountActiveAgreements(value)
        }
    }
    impl ::core::convert::From<GetAgreementDataCall> for fDAIxCalls {
        fn from(value: GetAgreementDataCall) -> Self {
            Self::GetAgreementData(value)
        }
    }
    impl ::core::convert::From<GetAgreementStateSlotCall> for fDAIxCalls {
        fn from(value: GetAgreementStateSlotCall) -> Self {
            Self::GetAgreementStateSlot(value)
        }
    }
    impl ::core::convert::From<GetCodeAddressCall> for fDAIxCalls {
        fn from(value: GetCodeAddressCall) -> Self {
            Self::GetCodeAddress(value)
        }
    }
    impl ::core::convert::From<GetHostCall> for fDAIxCalls {
        fn from(value: GetHostCall) -> Self {
            Self::GetHost(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingTokenCall> for fDAIxCalls {
        fn from(value: GetUnderlyingTokenCall) -> Self {
            Self::GetUnderlyingToken(value)
        }
    }
    impl ::core::convert::From<GranularityCall> for fDAIxCalls {
        fn from(value: GranularityCall) -> Self {
            Self::Granularity(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for fDAIxCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for fDAIxCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsAccountCriticalCall> for fDAIxCalls {
        fn from(value: IsAccountCriticalCall) -> Self {
            Self::IsAccountCritical(value)
        }
    }
    impl ::core::convert::From<IsAccountCriticalNowCall> for fDAIxCalls {
        fn from(value: IsAccountCriticalNowCall) -> Self {
            Self::IsAccountCriticalNow(value)
        }
    }
    impl ::core::convert::From<IsAccountSolventCall> for fDAIxCalls {
        fn from(value: IsAccountSolventCall) -> Self {
            Self::IsAccountSolvent(value)
        }
    }
    impl ::core::convert::From<IsAccountSolventNowCall> for fDAIxCalls {
        fn from(value: IsAccountSolventNowCall) -> Self {
            Self::IsAccountSolventNow(value)
        }
    }
    impl ::core::convert::From<IsOperatorForCall> for fDAIxCalls {
        fn from(value: IsOperatorForCall) -> Self {
            Self::IsOperatorFor(value)
        }
    }
    impl ::core::convert::From<MakeLiquidationPayoutsV2Call> for fDAIxCalls {
        fn from(value: MakeLiquidationPayoutsV2Call) -> Self {
            Self::MakeLiquidationPayoutsV2(value)
        }
    }
    impl ::core::convert::From<NameCall> for fDAIxCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OperationApproveCall> for fDAIxCalls {
        fn from(value: OperationApproveCall) -> Self {
            Self::OperationApprove(value)
        }
    }
    impl ::core::convert::From<OperationDecreaseAllowanceCall> for fDAIxCalls {
        fn from(value: OperationDecreaseAllowanceCall) -> Self {
            Self::OperationDecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<OperationDowngradeCall> for fDAIxCalls {
        fn from(value: OperationDowngradeCall) -> Self {
            Self::OperationDowngrade(value)
        }
    }
    impl ::core::convert::From<OperationIncreaseAllowanceCall> for fDAIxCalls {
        fn from(value: OperationIncreaseAllowanceCall) -> Self {
            Self::OperationIncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<OperationSendCall> for fDAIxCalls {
        fn from(value: OperationSendCall) -> Self {
            Self::OperationSend(value)
        }
    }
    impl ::core::convert::From<OperationTransferFromCall> for fDAIxCalls {
        fn from(value: OperationTransferFromCall) -> Self {
            Self::OperationTransferFrom(value)
        }
    }
    impl ::core::convert::From<OperationUpgradeCall> for fDAIxCalls {
        fn from(value: OperationUpgradeCall) -> Self {
            Self::OperationUpgrade(value)
        }
    }
    impl ::core::convert::From<OperatorBurnCall> for fDAIxCalls {
        fn from(value: OperatorBurnCall) -> Self {
            Self::OperatorBurn(value)
        }
    }
    impl ::core::convert::From<OperatorSendCall> for fDAIxCalls {
        fn from(value: OperatorSendCall) -> Self {
            Self::OperatorSend(value)
        }
    }
    impl ::core::convert::From<PoolAdminNFTCall> for fDAIxCalls {
        fn from(value: PoolAdminNFTCall) -> Self {
            Self::PoolAdminNFT(value)
        }
    }
    impl ::core::convert::From<PoolMemberNFTCall> for fDAIxCalls {
        fn from(value: PoolMemberNFTCall) -> Self {
            Self::PoolMemberNFT(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for fDAIxCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RealtimeBalanceOfCall> for fDAIxCalls {
        fn from(value: RealtimeBalanceOfCall) -> Self {
            Self::RealtimeBalanceOf(value)
        }
    }
    impl ::core::convert::From<RealtimeBalanceOfNowCall> for fDAIxCalls {
        fn from(value: RealtimeBalanceOfNowCall) -> Self {
            Self::RealtimeBalanceOfNow(value)
        }
    }
    impl ::core::convert::From<RevokeOperatorCall> for fDAIxCalls {
        fn from(value: RevokeOperatorCall) -> Self {
            Self::RevokeOperator(value)
        }
    }
    impl ::core::convert::From<SelfApproveForCall> for fDAIxCalls {
        fn from(value: SelfApproveForCall) -> Self {
            Self::SelfApproveFor(value)
        }
    }
    impl ::core::convert::From<SelfBurnCall> for fDAIxCalls {
        fn from(value: SelfBurnCall) -> Self {
            Self::SelfBurn(value)
        }
    }
    impl ::core::convert::From<SelfMintCall> for fDAIxCalls {
        fn from(value: SelfMintCall) -> Self {
            Self::SelfMint(value)
        }
    }
    impl ::core::convert::From<SelfTransferFromCall> for fDAIxCalls {
        fn from(value: SelfTransferFromCall) -> Self {
            Self::SelfTransferFrom(value)
        }
    }
    impl ::core::convert::From<SendCall> for fDAIxCalls {
        fn from(value: SendCall) -> Self {
            Self::Send(value)
        }
    }
    impl ::core::convert::From<SettleBalanceCall> for fDAIxCalls {
        fn from(value: SettleBalanceCall) -> Self {
            Self::SettleBalance(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for fDAIxCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TerminateAgreementCall> for fDAIxCalls {
        fn from(value: TerminateAgreementCall) -> Self {
            Self::TerminateAgreement(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for fDAIxCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for fDAIxCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferAllCall> for fDAIxCalls {
        fn from(value: TransferAllCall) -> Self {
            Self::TransferAll(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for fDAIxCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UpdateAgreementDataCall> for fDAIxCalls {
        fn from(value: UpdateAgreementDataCall) -> Self {
            Self::UpdateAgreementData(value)
        }
    }
    impl ::core::convert::From<UpdateAgreementStateSlotCall> for fDAIxCalls {
        fn from(value: UpdateAgreementStateSlotCall) -> Self {
            Self::UpdateAgreementStateSlot(value)
        }
    }
    impl ::core::convert::From<UpdateCodeCall> for fDAIxCalls {
        fn from(value: UpdateCodeCall) -> Self {
            Self::UpdateCode(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for fDAIxCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    impl ::core::convert::From<UpgradeToCall> for fDAIxCalls {
        fn from(value: UpgradeToCall) -> Self {
            Self::UpgradeTo(value)
        }
    }
    ///Container type for all return fields from the `CONSTANT_INFLOW_NFT_LOGIC` function with signature `CONSTANT_INFLOW_NFT_LOGIC()` and selector `0x5a7792df`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ConstantInflowNftLogicReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `CONSTANT_OUTFLOW_NFT_LOGIC` function with signature `CONSTANT_OUTFLOW_NFT_LOGIC()` and selector `0x5365d19f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ConstantOutflowNftLogicReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS` function with signature `SUPERFLUID_NFT_DEPLOYER_LIBRARY_ADDRESS()` and selector `0x70d75815`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SuperfluidNftDeployerLibraryAddressReturn(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `constantInflowNFT` function with signature `constantInflowNFT()` and selector `0xd50911cc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ConstantInflowNFTReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `constantOutflowNFT` function with signature `constantOutflowNFT()` and selector `0x0d9c12b5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ConstantOutflowNFTReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `defaultOperators` function with signature `defaultOperators()` and selector `0x06e48538`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultOperatorsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getAccountActiveAgreements` function with signature `getAccountActiveAgreements(address)` and selector `0x386fa221`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAccountActiveAgreementsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getAgreementData` function with signature `getAgreementData(address,bytes32,uint256)` and selector `0x6c2d9f2f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAgreementDataReturn {
        pub data: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `getAgreementStateSlot` function with signature `getAgreementStateSlot(address,address,uint256,uint256)` and selector `0x4b61cc33`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAgreementStateSlotReturn {
        pub slot_data: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `getCodeAddress` function with signature `getCodeAddress()` and selector `0x50d75d25`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetCodeAddressReturn {
        pub code_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getHost` function with signature `getHost()` and selector `0x20bc4425`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetHostReturn {
        pub host: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getUnderlyingToken` function with signature `getUnderlyingToken()` and selector `0xee719bc8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetUnderlyingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `granularity` function with signature `granularity()` and selector `0x556f0dc7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GranularityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `isAccountCritical` function with signature `isAccountCritical(address,uint256)` and selector `0xd9d078d6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsAccountCriticalReturn {
        pub is_critical: bool,
    }
    ///Container type for all return fields from the `isAccountCriticalNow` function with signature `isAccountCriticalNow(address)` and selector `0x79359f6f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsAccountCriticalNowReturn {
        pub is_critical: bool,
    }
    ///Container type for all return fields from the `isAccountSolvent` function with signature `isAccountSolvent(address,uint256)` and selector `0xb84cdd4a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsAccountSolventReturn {
        pub is_solvent: bool,
    }
    ///Container type for all return fields from the `isAccountSolventNow` function with signature `isAccountSolventNow(address)` and selector `0xbb0d196e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsAccountSolventNowReturn {
        pub is_solvent: bool,
    }
    ///Container type for all return fields from the `isOperatorFor` function with signature `isOperatorFor(address,address)` and selector `0xd95b6371`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsOperatorForReturn(pub bool);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `poolAdminNFT` function with signature `poolAdminNFT()` and selector `0xc76058fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolAdminNFTReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `poolMemberNFT` function with signature `poolMemberNFT()` and selector `0xc4b1584c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolMemberNFTReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `realtimeBalanceOf` function with signature `realtimeBalanceOf(address,uint256)` and selector `0xeb3537cc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RealtimeBalanceOfReturn {
        pub available_balance: ::ethers::core::types::I256,
        pub deposit: ::ethers::core::types::U256,
        pub owed_deposit: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `realtimeBalanceOfNow` function with signature `realtimeBalanceOfNow(address)` and selector `0x2ec8eec7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RealtimeBalanceOfNowReturn {
        pub available_balance: ::ethers::core::types::I256,
        pub deposit: ::ethers::core::types::U256,
        pub owed_deposit: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferFromReturn(pub bool);
}
