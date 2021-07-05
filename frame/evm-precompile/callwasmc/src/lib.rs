// Copyright (C) 2021 Cycan Technologies
//
// Licensed under the Business Source License included in the file License.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::marker::PhantomData;
use fp_evm::Precompile;
use evm::{ExitSucceed, ExitError, Context, executor::PrecompileOutput};
use pallet_evm::AddressMapping;
use frame_support::traits::Currency;
use pallet_contracts::chain_extension::UncheckedFrom;
use frame_support::sp_runtime::AccountId32;
use frame_system::Config as SysConfig;

pub struct Callwasmc<T: pallet_evm::Config> {
	_marker: PhantomData<T>,
}

impl<T> Precompile for Callwasmc<T> where
	T: pallet_evm::Config + pallet_vm_bridge::pallet::Config,
	T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]> + From<AccountId32>,
	<<T as pallet_contracts::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance: From<u128>,
	<T as SysConfig>::Origin: From<std::option::Option<<T as SysConfig>::AccountId>>,
{
	fn execute(
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
	) -> core::result::Result<PrecompileOutput, ExitError> {   //(ExitSucceed, Vec<u8>, u64)
	
		let origin = T::AddressMapping::into_account_id(context.caller);
		
		match pallet_vm_bridge::pallet::Module::<T>::call_wasm4evm(Some(origin).into(), input.iter().cloned().collect(), target_gas) {
			Ok(ret) => Ok(PrecompileOutput{exit_status:ExitSucceed::Stopped, cost:ret.1, output:ret.0, logs:Vec::new()}),
			Err(_) => Err(ExitError::Other("call wasmc execution failed".into())),		
		}		
	}
}
