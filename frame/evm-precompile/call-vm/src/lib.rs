// Copyright (C) 2021 Cycan Technologies
//
// Licensed under the Business Source License included in the file License.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use core::marker::PhantomData;
use fp_evm::Precompile;
use evm::{ExitSucceed, ExitError, Context, executor::PrecompileOutput};
use pallet_evm::AddressMapping;
//use frame_support::traits::Currency;
//use pallet_contracts::chain_extension::UncheckedFrom;
//use frame_support::sp_runtime::AccountId32;
use frame_system::Config as SysConfig;
use frame_system::pallet_prelude::*;

pub struct CallVm<T: pallet_evm::Config> 
{
	_marker: PhantomData<T>,
}

pub trait EvmChainExtension<C: SysConfig> {
	fn call_vm4evm(
			origin: OriginFor<C>,
			data: Vec<u8>,
			target_gas: Option<u64>
		) -> Result<(Vec<u8>, u64),sp_runtime::DispatchError>;
}

impl<T> Precompile for CallVm<T> where
	T: pallet_evm::Config + EvmChainExtension<T>,
//	T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]> + From<AccountId32>,
//	<<T as pallet_contracts::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance: From<u128>,
	<T as SysConfig>::Origin: From<std::option::Option<<T as SysConfig>::AccountId>>,
{
	fn execute(
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
	) -> core::result::Result<PrecompileOutput, ExitError> {   //(ExitSucceed, Vec<u8>, u64)
	
		let origin = T::AddressMapping::into_account_id(context.caller);
		
		match T::call_vm4evm(Some(origin).into(), input.iter().cloned().collect(), target_gas) {
			Ok(ret) => Ok(PrecompileOutput{exit_status:ExitSucceed::Stopped, cost:ret.1, output:ret.0, logs:Vec::new()}),
			Err(_) => Err(ExitError::Other("call wasmc execution failed".into())),		
		}		
	}
}
