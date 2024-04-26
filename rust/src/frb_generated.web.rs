// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// Section: imports

use super::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::for_generated::wasm_bindgen;
use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_web!();

// Section: dart2rust

impl CstDecode<String> for String {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        self
    }
}
impl CstDecode<crate::api::types::AllFees>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::AllFees {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            8,
            "Expected 8 elements, got {}",
            self_.length()
        );
        crate::api::types::AllFees {
            btc_limits: self_.get(0).cst_decode(),
            lbtc_limits: self_.get(1).cst_decode(),
            btc_submarine: self_.get(2).cst_decode(),
            btc_reverse: self_.get(3).cst_decode(),
            lbtc_submarine: self_.get(4).cst_decode(),
            lbtc_reverse: self_.get(5).cst_decode(),
            btc_pair_hash: self_.get(6).cst_decode(),
            lbtc_pair_hash: self_.get(7).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::error::BoltzError>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::error::BoltzError {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::error::BoltzError {
            kind: self_.get(0).cst_decode(),
            message: self_.get(1).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::btc_ln::BtcLnV1Swap>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::btc_ln::BtcLnV1Swap {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            11,
            "Expected 11 elements, got {}",
            self_.length()
        );
        crate::api::btc_ln::BtcLnV1Swap {
            id: self_.get(0).cst_decode(),
            kind: self_.get(1).cst_decode(),
            network: self_.get(2).cst_decode(),
            keys: self_.get(3).cst_decode(),
            preimage: self_.get(4).cst_decode(),
            redeem_script: self_.get(5).cst_decode(),
            invoice: self_.get(6).cst_decode(),
            script_address: self_.get(7).cst_decode(),
            out_amount: self_.get(8).cst_decode(),
            electrum_url: self_.get(9).cst_decode(),
            boltz_url: self_.get(10).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::btc_ln::BtcLnV2Swap>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::btc_ln::BtcLnV2Swap {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            11,
            "Expected 11 elements, got {}",
            self_.length()
        );
        crate::api::btc_ln::BtcLnV2Swap {
            id: self_.get(0).cst_decode(),
            kind: self_.get(1).cst_decode(),
            network: self_.get(2).cst_decode(),
            keys: self_.get(3).cst_decode(),
            preimage: self_.get(4).cst_decode(),
            swap_script: self_.get(5).cst_decode(),
            invoice: self_.get(6).cst_decode(),
            script_address: self_.get(7).cst_decode(),
            out_amount: self_.get(8).cst_decode(),
            electrum_url: self_.get(9).cst_decode(),
            boltz_url: self_.get(10).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::BtcSwapScriptV2Str>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::BtcSwapScriptV2Str {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            6,
            "Expected 6 elements, got {}",
            self_.length()
        );
        crate::api::types::BtcSwapScriptV2Str {
            swap_type: self_.get(0).cst_decode(),
            funding_addrs: self_.get(1).cst_decode(),
            hashlock: self_.get(2).cst_decode(),
            receiver_pubkey: self_.get(3).cst_decode(),
            locktime: self_.get(4).cst_decode(),
            sender_pubkey: self_.get(5).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::DecodedInvoice>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::DecodedInvoice {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            7,
            "Expected 7 elements, got {}",
            self_.length()
        );
        crate::api::types::DecodedInvoice {
            msats: self_.get(0).cst_decode(),
            expiry: self_.get(1).cst_decode(),
            expires_in: self_.get(2).cst_decode(),
            expires_at: self_.get(3).cst_decode(),
            is_expired: self_.get(4).cst_decode(),
            network: self_.get(5).cst_decode(),
            cltv_exp_delta: self_.get(6).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::KeyPair>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::KeyPair {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::types::KeyPair {
            secret_key: self_.get(0).cst_decode(),
            public_key: self_.get(1).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::LBtcSwapScriptV2Str>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::LBtcSwapScriptV2Str {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            7,
            "Expected 7 elements, got {}",
            self_.length()
        );
        crate::api::types::LBtcSwapScriptV2Str {
            swap_type: self_.get(0).cst_decode(),
            funding_addrs: self_.get(1).cst_decode(),
            hashlock: self_.get(2).cst_decode(),
            receiver_pubkey: self_.get(3).cst_decode(),
            locktime: self_.get(4).cst_decode(),
            sender_pubkey: self_.get(5).cst_decode(),
            blinding_key: self_.get(6).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::lbtc_ln::LbtcLnV1Swap>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::lbtc_ln::LbtcLnV1Swap {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            12,
            "Expected 12 elements, got {}",
            self_.length()
        );
        crate::api::lbtc_ln::LbtcLnV1Swap {
            id: self_.get(0).cst_decode(),
            kind: self_.get(1).cst_decode(),
            network: self_.get(2).cst_decode(),
            keys: self_.get(3).cst_decode(),
            preimage: self_.get(4).cst_decode(),
            redeem_script: self_.get(5).cst_decode(),
            invoice: self_.get(6).cst_decode(),
            out_amount: self_.get(7).cst_decode(),
            script_address: self_.get(8).cst_decode(),
            blinding_key: self_.get(9).cst_decode(),
            electrum_url: self_.get(10).cst_decode(),
            boltz_url: self_.get(11).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::lbtc_ln::LbtcLnV2Swap>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::lbtc_ln::LbtcLnV2Swap {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            12,
            "Expected 12 elements, got {}",
            self_.length()
        );
        crate::api::lbtc_ln::LbtcLnV2Swap {
            id: self_.get(0).cst_decode(),
            kind: self_.get(1).cst_decode(),
            network: self_.get(2).cst_decode(),
            keys: self_.get(3).cst_decode(),
            preimage: self_.get(4).cst_decode(),
            swap_script: self_.get(5).cst_decode(),
            invoice: self_.get(6).cst_decode(),
            out_amount: self_.get(7).cst_decode(),
            script_address: self_.get(8).cst_decode(),
            blinding_key: self_.get(9).cst_decode(),
            electrum_url: self_.get(10).cst_decode(),
            boltz_url: self_.get(11).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::Limits>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::Limits {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::types::Limits {
            minimal: self_.get(0).cst_decode(),
            maximal: self_.get(1).cst_decode(),
        }
    }
}
impl CstDecode<Vec<u8>> for Box<[u8]> {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl CstDecode<Option<String>> for Option<String> {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Option<String> {
        self.map(CstDecode::cst_decode)
    }
}
impl CstDecode<crate::api::types::PreImage>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::PreImage {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        crate::api::types::PreImage {
            value: self_.get(0).cst_decode(),
            sha256: self_.get(1).cst_decode(),
            hash160: self_.get(2).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::ReverseSwapFees>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::ReverseSwapFees {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        crate::api::types::ReverseSwapFees {
            boltz_fees_rate: self_.get(0).cst_decode(),
            lockup_fees: self_.get(1).cst_decode(),
            claim_fees_estimate: self_.get(2).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::types::SubmarineSwapFees>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::SubmarineSwapFees {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        crate::api::types::SubmarineSwapFees {
            boltz_fees_rate: self_.get(0).cst_decode(),
            claim_fees: self_.get(1).cst_decode(),
            lockup_fees_estimate: self_.get(2).cst_decode(),
        }
    }
}
impl CstDecode<String> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl CstDecode<bool> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> bool {
        self.is_truthy()
    }
}
impl CstDecode<crate::api::types::Chain>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::Chain {
        (self.unchecked_into_f64() as i32).cst_decode()
    }
}
impl CstDecode<f64> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<i32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<Vec<u8>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> Vec<u8> {
        self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>()
            .to_vec()
            .into()
    }
}
impl CstDecode<crate::api::types::SwapType>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> crate::api::types::SwapType {
        (self.unchecked_into_f64() as i32).cst_decode()
    }
}
impl CstDecode<u32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<u64> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u64 {
        ::std::convert::TryInto::try_into(
            self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::BigInt>()
                .unwrap(),
        )
        .unwrap()
    }
}
impl CstDecode<u8> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<usize> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    // Codec=Cst (C-struct based), see doc to use other codecs
    fn cst_decode(self) -> usize {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_1_swap_claim(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_btc_ln_v_1_swap_claim_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_1_swap_new(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    id: String,
    kind: i32,
    network: i32,
    keys: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    preimage: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    redeem_script: String,
    invoice: String,
    script_address: String,
    out_amount: u64,
    electrum_url: String,
    boltz_url: String,
) {
    wire_btc_ln_v_1_swap_new_impl(
        port_,
        id,
        kind,
        network,
        keys,
        preimage,
        redeem_script,
        invoice,
        script_address,
        out_amount,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_1_swap_new_reverse(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    out_amount: u64,
    network: i32,
    electrum_url: String,
    boltz_url: String,
    pair_hash: String,
) {
    wire_btc_ln_v_1_swap_new_reverse_impl(
        port_,
        mnemonic,
        index,
        out_amount,
        network,
        electrum_url,
        boltz_url,
        pair_hash,
    )
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_1_swap_new_submarine(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    invoice: String,
    network: i32,
    electrum_url: String,
    boltz_url: String,
    pair_hash: String,
) {
    wire_btc_ln_v_1_swap_new_submarine_impl(
        port_,
        mnemonic,
        index,
        invoice,
        network,
        electrum_url,
        boltz_url,
        pair_hash,
    )
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_1_swap_refund(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_btc_ln_v_1_swap_refund_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_1_swap_tx_size(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_btc_ln_v_1_swap_tx_size_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_2_swap_claim(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_btc_ln_v_2_swap_claim_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_2_swap_new(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    id: String,
    kind: i32,
    network: i32,
    keys: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    preimage: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    swap_script: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    invoice: String,
    script_address: String,
    out_amount: u64,
    electrum_url: String,
    boltz_url: String,
) {
    wire_btc_ln_v_2_swap_new_impl(
        port_,
        id,
        kind,
        network,
        keys,
        preimage,
        swap_script,
        invoice,
        script_address,
        out_amount,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_2_swap_new_reverse(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    out_amount: u64,
    network: i32,
    electrum_url: String,
    boltz_url: String,
) {
    wire_btc_ln_v_2_swap_new_reverse_impl(
        port_,
        mnemonic,
        index,
        out_amount,
        network,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_2_swap_new_submarine(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    invoice: String,
    network: i32,
    electrum_url: String,
    boltz_url: String,
) {
    wire_btc_ln_v_2_swap_new_submarine_impl(
        port_,
        mnemonic,
        index,
        invoice,
        network,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_2_swap_refund(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_btc_ln_v_2_swap_refund_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_btc_ln_v_2_swap_tx_size(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_btc_ln_v_2_swap_tx_size_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_boltz_error_new(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    kind: String,
    message: String,
) {
    wire_boltz_error_new_impl(port_, kind, message)
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_1_swap_claim(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_lbtc_ln_v_1_swap_claim_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_1_swap_new(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    id: String,
    kind: i32,
    network: i32,
    keys: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    preimage: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    redeem_script: String,
    invoice: String,
    out_amount: u64,
    out_address: String,
    blinding_key: String,
    electrum_url: String,
    boltz_url: String,
) {
    wire_lbtc_ln_v_1_swap_new_impl(
        port_,
        id,
        kind,
        network,
        keys,
        preimage,
        redeem_script,
        invoice,
        out_amount,
        out_address,
        blinding_key,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_1_swap_new_reverse(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    out_amount: u64,
    network: i32,
    electrum_url: String,
    boltz_url: String,
    pair_hash: String,
) {
    wire_lbtc_ln_v_1_swap_new_reverse_impl(
        port_,
        mnemonic,
        index,
        out_amount,
        network,
        electrum_url,
        boltz_url,
        pair_hash,
    )
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_1_swap_new_submarine(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    invoice: String,
    network: i32,
    electrum_url: String,
    boltz_url: String,
    pair_hash: String,
) {
    wire_lbtc_ln_v_1_swap_new_submarine_impl(
        port_,
        mnemonic,
        index,
        invoice,
        network,
        electrum_url,
        boltz_url,
        pair_hash,
    )
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_1_swap_refund(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_lbtc_ln_v_1_swap_refund_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_1_swap_tx_size(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    swap: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_lbtc_ln_v_1_swap_tx_size_impl(port_, swap)
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_2_swap_claim(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_lbtc_ln_v_2_swap_claim_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_2_swap_new(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    id: String,
    kind: i32,
    network: i32,
    keys: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    preimage: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    swap_script: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    invoice: String,
    out_amount: u64,
    out_address: String,
    blinding_key: String,
    electrum_url: String,
    boltz_url: String,
) {
    wire_lbtc_ln_v_2_swap_new_impl(
        port_,
        id,
        kind,
        network,
        keys,
        preimage,
        swap_script,
        invoice,
        out_amount,
        out_address,
        blinding_key,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_2_swap_new_reverse(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    out_amount: u64,
    network: i32,
    electrum_url: String,
    boltz_url: String,
) {
    wire_lbtc_ln_v_2_swap_new_reverse_impl(
        port_,
        mnemonic,
        index,
        out_amount,
        network,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_2_swap_new_submarine(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    index: u64,
    invoice: String,
    network: i32,
    electrum_url: String,
    boltz_url: String,
) {
    wire_lbtc_ln_v_2_swap_new_submarine_impl(
        port_,
        mnemonic,
        index,
        invoice,
        network,
        electrum_url,
        boltz_url,
    )
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_2_swap_refund(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    out_address: String,
    abs_fee: u64,
) {
    wire_lbtc_ln_v_2_swap_refund_impl(port_, that, out_address, abs_fee)
}

#[wasm_bindgen]
pub fn wire_lbtc_ln_v_2_swap_tx_size(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    that: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
) {
    wire_lbtc_ln_v_2_swap_tx_size_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_all_fees_fetch(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    boltz_url: String,
) {
    wire_all_fees_fetch_impl(port_, boltz_url)
}

#[wasm_bindgen]
pub fn wire_decoded_invoice_from_string(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    s: String,
) {
    wire_decoded_invoice_from_string_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_key_pair_generate(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    mnemonic: String,
    network: i32,
    index: u64,
    swap_type: i32,
) {
    wire_key_pair_generate_impl(port_, mnemonic, network, index, swap_type)
}

#[wasm_bindgen]
pub fn wire_key_pair_new(
    secret_key: String,
    public_key: String,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_key_pair_new_impl(secret_key, public_key)
}

#[wasm_bindgen]
pub fn wire_pre_image_generate(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_pre_image_generate_impl(port_)
}

#[wasm_bindgen]
pub fn wire_pre_image_new(
    value: String,
    sha256: String,
    hash160: String,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_pre_image_new_impl(value, sha256, hash160)
}