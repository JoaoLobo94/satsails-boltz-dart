#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.6.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::types::AllFees;
use crate::types::BoltzError;
use crate::types::BtcLnSwap;
use crate::types::Chain;
use crate::types::KeyPair;
use crate::types::LbtcLnSwap;
use crate::types::Limits;
use crate::types::PreImage;
use crate::types::ReverseSwapFees;
use crate::types::SubmarineSwapFees;
use crate::types::SwapType;

// Section: wire functions

fn wire_swap_fees__static_method__Api_impl(
    port_: MessagePort,
    boltz_url: impl Wire2Api<String> + UnwindSafe,
    output_amount: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, AllFees, _>(
        WrapInfo {
            debug_name: "swap_fees__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_boltz_url = boltz_url.wire2api();
            let api_output_amount = output_amount.wire2api();
            move |task_callback| Api::swap_fees(api_boltz_url, api_output_amount)
        },
    )
}
fn wire_new_btc_ln_submarine__static_method__Api_impl(
    port_: MessagePort,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
    index: impl Wire2Api<u64> + UnwindSafe,
    invoice: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Chain> + UnwindSafe,
    electrum_url: impl Wire2Api<String> + UnwindSafe,
    boltz_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, BtcLnSwap, _>(
        WrapInfo {
            debug_name: "new_btc_ln_submarine__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_mnemonic = mnemonic.wire2api();
            let api_index = index.wire2api();
            let api_invoice = invoice.wire2api();
            let api_network = network.wire2api();
            let api_electrum_url = electrum_url.wire2api();
            let api_boltz_url = boltz_url.wire2api();
            move |task_callback| {
                Api::new_btc_ln_submarine(
                    api_mnemonic,
                    api_index,
                    api_invoice,
                    api_network,
                    api_electrum_url,
                    api_boltz_url,
                )
            }
        },
    )
}
fn wire_new_btc_ln_reverse__static_method__Api_impl(
    port_: MessagePort,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
    index: impl Wire2Api<u64> + UnwindSafe,
    out_amount: impl Wire2Api<u64> + UnwindSafe,
    network: impl Wire2Api<Chain> + UnwindSafe,
    electrum_url: impl Wire2Api<String> + UnwindSafe,
    boltz_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, BtcLnSwap, _>(
        WrapInfo {
            debug_name: "new_btc_ln_reverse__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_mnemonic = mnemonic.wire2api();
            let api_index = index.wire2api();
            let api_out_amount = out_amount.wire2api();
            let api_network = network.wire2api();
            let api_electrum_url = electrum_url.wire2api();
            let api_boltz_url = boltz_url.wire2api();
            move |task_callback| {
                Api::new_btc_ln_reverse(
                    api_mnemonic,
                    api_index,
                    api_out_amount,
                    api_network,
                    api_electrum_url,
                    api_boltz_url,
                )
            }
        },
    )
}
fn wire_btc_ln_tx_size__static_method__Api_impl(
    port_: MessagePort,
    swap: impl Wire2Api<BtcLnSwap> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, usize, _>(
        WrapInfo {
            debug_name: "btc_ln_tx_size__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_swap = swap.wire2api();
            move |task_callback| Api::btc_ln_tx_size(api_swap)
        },
    )
}
fn wire_btc_ln_reverse_claim__static_method__Api_impl(
    port_: MessagePort,
    swap: impl Wire2Api<BtcLnSwap> + UnwindSafe,
    out_address: impl Wire2Api<String> + UnwindSafe,
    abs_fee: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "btc_ln_reverse_claim__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_swap = swap.wire2api();
            let api_out_address = out_address.wire2api();
            let api_abs_fee = abs_fee.wire2api();
            move |task_callback| Api::btc_ln_reverse_claim(api_swap, api_out_address, api_abs_fee)
        },
    )
}
fn wire_new_lbtc_ln_submarine__static_method__Api_impl(
    port_: MessagePort,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
    index: impl Wire2Api<u64> + UnwindSafe,
    invoice: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Chain> + UnwindSafe,
    electrum_url: impl Wire2Api<String> + UnwindSafe,
    boltz_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, LbtcLnSwap, _>(
        WrapInfo {
            debug_name: "new_lbtc_ln_submarine__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_mnemonic = mnemonic.wire2api();
            let api_index = index.wire2api();
            let api_invoice = invoice.wire2api();
            let api_network = network.wire2api();
            let api_electrum_url = electrum_url.wire2api();
            let api_boltz_url = boltz_url.wire2api();
            move |task_callback| {
                Api::new_lbtc_ln_submarine(
                    api_mnemonic,
                    api_index,
                    api_invoice,
                    api_network,
                    api_electrum_url,
                    api_boltz_url,
                )
            }
        },
    )
}
fn wire_new_lbtc_ln_reverse__static_method__Api_impl(
    port_: MessagePort,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
    index: impl Wire2Api<u64> + UnwindSafe,
    out_amount: impl Wire2Api<u64> + UnwindSafe,
    network: impl Wire2Api<Chain> + UnwindSafe,
    electrum_url: impl Wire2Api<String> + UnwindSafe,
    boltz_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, LbtcLnSwap, _>(
        WrapInfo {
            debug_name: "new_lbtc_ln_reverse__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_mnemonic = mnemonic.wire2api();
            let api_index = index.wire2api();
            let api_out_amount = out_amount.wire2api();
            let api_network = network.wire2api();
            let api_electrum_url = electrum_url.wire2api();
            let api_boltz_url = boltz_url.wire2api();
            move |task_callback| {
                Api::new_lbtc_ln_reverse(
                    api_mnemonic,
                    api_index,
                    api_out_amount,
                    api_network,
                    api_electrum_url,
                    api_boltz_url,
                )
            }
        },
    )
}
fn wire_lbtc_ln_tx_size__static_method__Api_impl(
    port_: MessagePort,
    swap: impl Wire2Api<LbtcLnSwap> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, usize, _>(
        WrapInfo {
            debug_name: "lbtc_ln_tx_size__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_swap = swap.wire2api();
            move |task_callback| Api::lbtc_ln_tx_size(api_swap)
        },
    )
}
fn wire_lbtc_ln_reverse_claim__static_method__Api_impl(
    port_: MessagePort,
    swap: impl Wire2Api<LbtcLnSwap> + UnwindSafe,
    out_address: impl Wire2Api<String> + UnwindSafe,
    abs_fee: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "lbtc_ln_reverse_claim__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_swap = swap.wire2api();
            let api_out_address = out_address.wire2api();
            let api_abs_fee = abs_fee.wire2api();
            move |task_callback| Api::lbtc_ln_reverse_claim(api_swap, api_out_address, api_abs_fee)
        },
    )
}
fn wire_swap_status__static_method__Api_impl(
    port_: MessagePort,
    boltz_url: impl Wire2Api<String> + UnwindSafe,
    id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "swap_status__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_boltz_url = boltz_url.wire2api();
            let api_id = id.wire2api();
            move |task_callback| Api::swap_status(api_boltz_url, api_id)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<Chain> for i32 {
    fn wire2api(self) -> Chain {
        match self {
            0 => Chain::Testnet,
            1 => Chain::LiquidTestnet,
            _ => unreachable!("Invalid variant for Chain: {}", self),
        }
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<SwapType> for i32 {
    fn wire2api(self) -> SwapType {
        match self {
            0 => SwapType::Submarine,
            1 => SwapType::Reverse,
            _ => unreachable!("Invalid variant for SwapType: {}", self),
        }
    }
}
impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for AllFees {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.btc_limits.into_into_dart().into_dart(),
            self.lbtc_limits.into_into_dart().into_dart(),
            self.btc_submarine.into_into_dart().into_dart(),
            self.btc_reverse.into_into_dart().into_dart(),
            self.lbtc_submarine.into_into_dart().into_dart(),
            self.lbtc_reverse.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for AllFees {}
impl rust2dart::IntoIntoDart<AllFees> for AllFees {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for BoltzError {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.kind.into_into_dart().into_dart(),
            self.message.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BoltzError {}
impl rust2dart::IntoIntoDart<BoltzError> for BoltzError {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for BtcLnSwap {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_into_dart().into_dart(),
            self.kind.into_into_dart().into_dart(),
            self.network.into_into_dart().into_dart(),
            self.keys.into_into_dart().into_dart(),
            self.preimage.into_into_dart().into_dart(),
            self.redeem_script.into_into_dart().into_dart(),
            self.invoice.into_into_dart().into_dart(),
            self.script_address.into_into_dart().into_dart(),
            self.out_amount.into_into_dart().into_dart(),
            self.electrum_url.into_into_dart().into_dart(),
            self.boltz_url.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for BtcLnSwap {}
impl rust2dart::IntoIntoDart<BtcLnSwap> for BtcLnSwap {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Chain {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Testnet => 0,
            Self::LiquidTestnet => 1,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Chain {}
impl rust2dart::IntoIntoDart<Chain> for Chain {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for KeyPair {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.secret_key.into_into_dart().into_dart(),
            self.public_key.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for KeyPair {}
impl rust2dart::IntoIntoDart<KeyPair> for KeyPair {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for LbtcLnSwap {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_into_dart().into_dart(),
            self.kind.into_into_dart().into_dart(),
            self.network.into_into_dart().into_dart(),
            self.keys.into_into_dart().into_dart(),
            self.preimage.into_into_dart().into_dart(),
            self.redeem_script.into_into_dart().into_dart(),
            self.invoice.into_into_dart().into_dart(),
            self.out_amount.into_into_dart().into_dart(),
            self.script_address.into_into_dart().into_dart(),
            self.blinding_key.into_into_dart().into_dart(),
            self.electrum_url.into_into_dart().into_dart(),
            self.boltz_url.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LbtcLnSwap {}
impl rust2dart::IntoIntoDart<LbtcLnSwap> for LbtcLnSwap {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Limits {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.minimal.into_into_dart().into_dart(),
            self.maximal.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Limits {}
impl rust2dart::IntoIntoDart<Limits> for Limits {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for PreImage {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.value.into_into_dart().into_dart(),
            self.sha256.into_into_dart().into_dart(),
            self.hash160.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PreImage {}
impl rust2dart::IntoIntoDart<PreImage> for PreImage {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for ReverseSwapFees {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.boltz_fees.into_into_dart().into_dart(),
            self.lockup_fees.into_into_dart().into_dart(),
            self.claim_fees_estimate.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ReverseSwapFees {}
impl rust2dart::IntoIntoDart<ReverseSwapFees> for ReverseSwapFees {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for SubmarineSwapFees {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.boltz_fees.into_into_dart().into_dart(),
            self.claim_fees.into_into_dart().into_dart(),
            self.lockup_fees_estimate.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SubmarineSwapFees {}
impl rust2dart::IntoIntoDart<SubmarineSwapFees> for SubmarineSwapFees {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for SwapType {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Submarine => 0,
            Self::Reverse => 1,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SwapType {}
impl rust2dart::IntoIntoDart<SwapType> for SwapType {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
mod io {
    use super::*;
    // Section: wire functions

    #[no_mangle]
    pub extern "C" fn wire_swap_fees__static_method__Api(
        port_: i64,
        boltz_url: *mut wire_uint_8_list,
        output_amount: u64,
    ) {
        wire_swap_fees__static_method__Api_impl(port_, boltz_url, output_amount)
    }

    #[no_mangle]
    pub extern "C" fn wire_new_btc_ln_submarine__static_method__Api(
        port_: i64,
        mnemonic: *mut wire_uint_8_list,
        index: u64,
        invoice: *mut wire_uint_8_list,
        network: i32,
        electrum_url: *mut wire_uint_8_list,
        boltz_url: *mut wire_uint_8_list,
    ) {
        wire_new_btc_ln_submarine__static_method__Api_impl(
            port_,
            mnemonic,
            index,
            invoice,
            network,
            electrum_url,
            boltz_url,
        )
    }

    #[no_mangle]
    pub extern "C" fn wire_new_btc_ln_reverse__static_method__Api(
        port_: i64,
        mnemonic: *mut wire_uint_8_list,
        index: u64,
        out_amount: u64,
        network: i32,
        electrum_url: *mut wire_uint_8_list,
        boltz_url: *mut wire_uint_8_list,
    ) {
        wire_new_btc_ln_reverse__static_method__Api_impl(
            port_,
            mnemonic,
            index,
            out_amount,
            network,
            electrum_url,
            boltz_url,
        )
    }

    #[no_mangle]
    pub extern "C" fn wire_btc_ln_tx_size__static_method__Api(
        port_: i64,
        swap: *mut wire_BtcLnSwap,
    ) {
        wire_btc_ln_tx_size__static_method__Api_impl(port_, swap)
    }

    #[no_mangle]
    pub extern "C" fn wire_btc_ln_reverse_claim__static_method__Api(
        port_: i64,
        swap: *mut wire_BtcLnSwap,
        out_address: *mut wire_uint_8_list,
        abs_fee: u64,
    ) {
        wire_btc_ln_reverse_claim__static_method__Api_impl(port_, swap, out_address, abs_fee)
    }

    #[no_mangle]
    pub extern "C" fn wire_new_lbtc_ln_submarine__static_method__Api(
        port_: i64,
        mnemonic: *mut wire_uint_8_list,
        index: u64,
        invoice: *mut wire_uint_8_list,
        network: i32,
        electrum_url: *mut wire_uint_8_list,
        boltz_url: *mut wire_uint_8_list,
    ) {
        wire_new_lbtc_ln_submarine__static_method__Api_impl(
            port_,
            mnemonic,
            index,
            invoice,
            network,
            electrum_url,
            boltz_url,
        )
    }

    #[no_mangle]
    pub extern "C" fn wire_new_lbtc_ln_reverse__static_method__Api(
        port_: i64,
        mnemonic: *mut wire_uint_8_list,
        index: u64,
        out_amount: u64,
        network: i32,
        electrum_url: *mut wire_uint_8_list,
        boltz_url: *mut wire_uint_8_list,
    ) {
        wire_new_lbtc_ln_reverse__static_method__Api_impl(
            port_,
            mnemonic,
            index,
            out_amount,
            network,
            electrum_url,
            boltz_url,
        )
    }

    #[no_mangle]
    pub extern "C" fn wire_lbtc_ln_tx_size__static_method__Api(
        port_: i64,
        swap: *mut wire_LbtcLnSwap,
    ) {
        wire_lbtc_ln_tx_size__static_method__Api_impl(port_, swap)
    }

    #[no_mangle]
    pub extern "C" fn wire_lbtc_ln_reverse_claim__static_method__Api(
        port_: i64,
        swap: *mut wire_LbtcLnSwap,
        out_address: *mut wire_uint_8_list,
        abs_fee: u64,
    ) {
        wire_lbtc_ln_reverse_claim__static_method__Api_impl(port_, swap, out_address, abs_fee)
    }

    #[no_mangle]
    pub extern "C" fn wire_swap_status__static_method__Api(
        port_: i64,
        boltz_url: *mut wire_uint_8_list,
        id: *mut wire_uint_8_list,
    ) {
        wire_swap_status__static_method__Api_impl(port_, boltz_url, id)
    }

    // Section: allocate functions

    #[no_mangle]
    pub extern "C" fn new_box_autoadd_btc_ln_swap_0() -> *mut wire_BtcLnSwap {
        support::new_leak_box_ptr(wire_BtcLnSwap::new_with_null_ptr())
    }

    #[no_mangle]
    pub extern "C" fn new_box_autoadd_lbtc_ln_swap_0() -> *mut wire_LbtcLnSwap {
        support::new_leak_box_ptr(wire_LbtcLnSwap::new_with_null_ptr())
    }

    #[no_mangle]
    pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
        let ans = wire_uint_8_list {
            ptr: support::new_leak_vec_ptr(Default::default(), len),
            len,
        };
        support::new_leak_box_ptr(ans)
    }

    // Section: related functions

    // Section: impl Wire2Api

    impl Wire2Api<String> for *mut wire_uint_8_list {
        fn wire2api(self) -> String {
            let vec: Vec<u8> = self.wire2api();
            String::from_utf8_lossy(&vec).into_owned()
        }
    }
    impl Wire2Api<BtcLnSwap> for *mut wire_BtcLnSwap {
        fn wire2api(self) -> BtcLnSwap {
            let wrap = unsafe { support::box_from_leak_ptr(self) };
            Wire2Api::<BtcLnSwap>::wire2api(*wrap).into()
        }
    }
    impl Wire2Api<LbtcLnSwap> for *mut wire_LbtcLnSwap {
        fn wire2api(self) -> LbtcLnSwap {
            let wrap = unsafe { support::box_from_leak_ptr(self) };
            Wire2Api::<LbtcLnSwap>::wire2api(*wrap).into()
        }
    }
    impl Wire2Api<BtcLnSwap> for wire_BtcLnSwap {
        fn wire2api(self) -> BtcLnSwap {
            BtcLnSwap {
                id: self.id.wire2api(),
                kind: self.kind.wire2api(),
                network: self.network.wire2api(),
                keys: self.keys.wire2api(),
                preimage: self.preimage.wire2api(),
                redeem_script: self.redeem_script.wire2api(),
                invoice: self.invoice.wire2api(),
                script_address: self.script_address.wire2api(),
                out_amount: self.out_amount.wire2api(),
                electrum_url: self.electrum_url.wire2api(),
                boltz_url: self.boltz_url.wire2api(),
            }
        }
    }

    impl Wire2Api<KeyPair> for wire_KeyPair {
        fn wire2api(self) -> KeyPair {
            KeyPair {
                secret_key: self.secret_key.wire2api(),
                public_key: self.public_key.wire2api(),
            }
        }
    }
    impl Wire2Api<LbtcLnSwap> for wire_LbtcLnSwap {
        fn wire2api(self) -> LbtcLnSwap {
            LbtcLnSwap {
                id: self.id.wire2api(),
                kind: self.kind.wire2api(),
                network: self.network.wire2api(),
                keys: self.keys.wire2api(),
                preimage: self.preimage.wire2api(),
                redeem_script: self.redeem_script.wire2api(),
                invoice: self.invoice.wire2api(),
                out_amount: self.out_amount.wire2api(),
                script_address: self.script_address.wire2api(),
                blinding_key: self.blinding_key.wire2api(),
                electrum_url: self.electrum_url.wire2api(),
                boltz_url: self.boltz_url.wire2api(),
            }
        }
    }
    impl Wire2Api<PreImage> for wire_PreImage {
        fn wire2api(self) -> PreImage {
            PreImage {
                value: self.value.wire2api(),
                sha256: self.sha256.wire2api(),
                hash160: self.hash160.wire2api(),
            }
        }
    }

    impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
        fn wire2api(self) -> Vec<u8> {
            unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }
        }
    }
    // Section: wire structs

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_BtcLnSwap {
        id: *mut wire_uint_8_list,
        kind: i32,
        network: i32,
        keys: wire_KeyPair,
        preimage: wire_PreImage,
        redeem_script: *mut wire_uint_8_list,
        invoice: *mut wire_uint_8_list,
        script_address: *mut wire_uint_8_list,
        out_amount: u64,
        electrum_url: *mut wire_uint_8_list,
        boltz_url: *mut wire_uint_8_list,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_KeyPair {
        secret_key: *mut wire_uint_8_list,
        public_key: *mut wire_uint_8_list,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_LbtcLnSwap {
        id: *mut wire_uint_8_list,
        kind: i32,
        network: i32,
        keys: wire_KeyPair,
        preimage: wire_PreImage,
        redeem_script: *mut wire_uint_8_list,
        invoice: *mut wire_uint_8_list,
        out_amount: u64,
        script_address: *mut wire_uint_8_list,
        blinding_key: *mut wire_uint_8_list,
        electrum_url: *mut wire_uint_8_list,
        boltz_url: *mut wire_uint_8_list,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_PreImage {
        value: *mut wire_uint_8_list,
        sha256: *mut wire_uint_8_list,
        hash160: *mut wire_uint_8_list,
    }

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_uint_8_list {
        ptr: *mut u8,
        len: i32,
    }

    // Section: impl NewWithNullPtr

    pub trait NewWithNullPtr {
        fn new_with_null_ptr() -> Self;
    }

    impl<T> NewWithNullPtr for *mut T {
        fn new_with_null_ptr() -> Self {
            std::ptr::null_mut()
        }
    }

    impl NewWithNullPtr for wire_BtcLnSwap {
        fn new_with_null_ptr() -> Self {
            Self {
                id: core::ptr::null_mut(),
                kind: Default::default(),
                network: Default::default(),
                keys: Default::default(),
                preimage: Default::default(),
                redeem_script: core::ptr::null_mut(),
                invoice: core::ptr::null_mut(),
                script_address: core::ptr::null_mut(),
                out_amount: Default::default(),
                electrum_url: core::ptr::null_mut(),
                boltz_url: core::ptr::null_mut(),
            }
        }
    }

    impl Default for wire_BtcLnSwap {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    impl NewWithNullPtr for wire_KeyPair {
        fn new_with_null_ptr() -> Self {
            Self {
                secret_key: core::ptr::null_mut(),
                public_key: core::ptr::null_mut(),
            }
        }
    }

    impl Default for wire_KeyPair {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    impl NewWithNullPtr for wire_LbtcLnSwap {
        fn new_with_null_ptr() -> Self {
            Self {
                id: core::ptr::null_mut(),
                kind: Default::default(),
                network: Default::default(),
                keys: Default::default(),
                preimage: Default::default(),
                redeem_script: core::ptr::null_mut(),
                invoice: core::ptr::null_mut(),
                out_amount: Default::default(),
                script_address: core::ptr::null_mut(),
                blinding_key: core::ptr::null_mut(),
                electrum_url: core::ptr::null_mut(),
                boltz_url: core::ptr::null_mut(),
            }
        }
    }

    impl Default for wire_LbtcLnSwap {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    impl NewWithNullPtr for wire_PreImage {
        fn new_with_null_ptr() -> Self {
            Self {
                value: core::ptr::null_mut(),
                sha256: core::ptr::null_mut(),
                hash160: core::ptr::null_mut(),
            }
        }
    }

    impl Default for wire_PreImage {
        fn default() -> Self {
            Self::new_with_null_ptr()
        }
    }

    // Section: sync execution mode utility

    #[no_mangle]
    pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
        unsafe {
            let _ = support::box_from_leak_ptr(ptr);
        };
    }
}
#[cfg(not(target_family = "wasm"))]
pub use self::io::*;
