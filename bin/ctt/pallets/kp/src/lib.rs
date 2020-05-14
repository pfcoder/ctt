#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode},
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
};
use sp_std::prelude::*;

/// Knowledge power pallet  with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_system::{self as system, ensure_signed};

use sp_runtime::{print, MultiSignature, RuntimeDebug};

use sp_core::sr25519;
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, PartialEq, Clone, RuntimeDebug)]
pub enum KnowledgeType {
    ProductPublish = 0,
    ProductIdentify,
    ProductTry,
    Comment,
    Unknown,
}

impl Default for KnowledgeType {
    fn default() -> Self {
        KnowledgeType::ProductPublish
    }
}

impl From<u8> for KnowledgeType {
    fn from(orig: u8) -> Self {
        match orig {
            0x0 => return KnowledgeType::ProductPublish,
            0x1 => return KnowledgeType::ProductIdentify,
            0x2 => return KnowledgeType::ProductTry,
            0x3 => return KnowledgeType::Comment,
            _ => return KnowledgeType::Unknown,
        };
    }
}

type KnowledgeBaseDataOf<T> =
    KnowledgeBaseData<<T as system::Trait>::AccountId, <T as system::Trait>::Hash>;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct KnowledgeBaseData<AccountId, Hash> {
    app_id: Vec<u8>,
    content_hash: Hash,
    extra_compute_param: u8,
    knowledge_id: Vec<u8>,
    knowledge_type: KnowledgeType,
    memo: Hash,
    model_id: Vec<u8>,
    owner: AccountId,
    product_id: Vec<u8>,
    tx_id: Vec<u8>,
}

type KnowledgeCommentDataOf<T> =
    KnowledgeCommentData<<T as system::Trait>::AccountId, <T as system::Trait>::Hash>;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct KnowledgeCommentData<AccountId, Hash> {
    app_id: Vec<u8>,
    knowledge_id: Vec<u8>,
    comment_id: Vec<u8>,
    last_comment_id: Vec<u8>,
    comment_hash: Hash,
    comment_fee: u32,
    knowledge_profit: u32,
    owner: AccountId,
}

type KnowledgePowerDataOf<T> = KnowledgePowerData<<T as system::Trait>::AccountId>;

#[derive(Encode, Decode, Clone, Default, RuntimeDebug)]
pub struct KnowledgePowerData<AccountId> {
    app_id: Vec<u8>,
    knowledge_id: Vec<u8>,
    owner: AccountId,
    power: u32,
    // A: knowledge owner total profit
    owner_profit: u32,
    // B: comment total count
    comment_total_count: u32,
    // C: total user number of attending comment action
    comment_total_user: u32,
    // D: total cost of comments
    comment_total_cost: u32,
    // E: max cost of comment
    comment_max_cost: u32,
    // F: comments which repeated users count, for example: AABBBCD, 2 + 3
    comment_repeat_user_count: u32,
    // G: comment cost increase count
    comment_cost_increase_count: u32,
    // H: comment count of (user = knowledge owner)
    comment_self_count: u32,
}

/// our power compute algo is:
/// p = (comment_total_user * comment_total_cost) * (1 + comment_cost_increase_count / comment_total_count)
/// 	/ (owner_profit * (comment_self_count / comment_total_count + comment_repeat_user_count / comment_total_count) )
/// 	* comment_max_cost / comment_cost_increase_count
/// 	* (extra_compute_param / 100)
///
/// With simple symbol:
/// p = ((C * D) * (1 + G / B) / (A * (H / B + F / B))) * (E / G) * (ep / 100)
/// Simplified to:
/// p = ((C * D * E * (B + G)) / (A * G * (H + F)) * (ep / 100)
fn power_update<T: system::Trait>(power_data: &KnowledgePowerData<T::AccountId>, ep: u32) -> u32 {
    match power_data {
        KnowledgePowerData {
            app_id: _,
            knowledge_id: _,
            owner: _,
            power: _,
            owner_profit: a,
            comment_total_count: b,
            comment_total_user: c,
            comment_total_cost: d,
            comment_max_cost: e,
            comment_repeat_user_count: f,
            comment_cost_increase_count: g,
            comment_self_count: h,
        } => {
            if *a == 0 || *g == 0 {
                print("Power compute 0, because has 0 value in den !");
                return 0;
            }

            // TODO: overflow check
            // c * d * e * (b + g) / (a * g * (h + f)) * (ep / 100)
            let step1 = c * d * e * (b + g);
            let mut step2 = a * g;
            if h + f > 0 {
                step2 *= h + f;
            }

            let result: u32 = step1 * ep / step2 / 100;
            result
        }
    }
}

/// The pallet's configuration trait.
pub trait Trait: frame_system::Trait {
    // Add other types and constants required to configure this pallet.

    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    trait Store for Module<T: Trait> as Kp {
        // Trusted application server account
        AuthServers get(fn auth_servers) config() : Vec<T::AccountId>;

        // (AppId, KnowledgeId) -> KnowledgeBaseData
        KnowledgeBaseDataByIdHash get(fn knowledge_basedata_by_idhash):
            map hasher(twox_64_concat) (Vec<u8>, Vec<u8>) => KnowledgeBaseDataOf<T>;

        // (AppId, CommentId) -> KnowledgeCommentData
        KnowledgeCommentDataByIdHash get(fn knowledge_commentdata_by_idhash):
            map hasher(twox_64_concat) (Vec<u8>, Vec<u8>) => KnowledgeCommentDataOf<T>;

        // (AppId, KnowledgeId) -> KnowledgePowerData
        KnowledgePowerDataByIdHash get(fn knowledge_powerdata_by_idhash):
            map hasher(twox_64_concat) (Vec<u8>, Vec<u8>) => KnowledgePowerDataOf<T>;

        // (AccountId, AppId, KnowledgeId) -> u32
        KnowledgeCommentUserCountHash get(fn knowledge_comment_user_count_hash):
            map hasher(twox_64_concat) (<T as system::Trait>::AccountId, Vec<u8>, Vec<u8>) => u32;

        // global total knowledge power
        TotalPower get(fn total_power): u32;

        // miner power table
        MinerPowerByAccount get(fn miner_power_by_account):
            map hasher(blake2_128_concat) <T as system::Trait>::AccountId => u32;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Just a dummy event.
        /// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        /// To emit this event, we call the deposit function, from our runtime functions
        // SomethingStored(u32, AccountId),
        KnowledgeCreated(AccountId),
        CommentCreated(AccountId),
    }
);

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Some action needs to check specified account has enough balance to pay for gas fee.
        BalanceNotEnough
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create_knowledge(origin,
            app_id: Vec<u8>,
            knowledge_type: u8,
            knowledge_id: Vec<u8>,
            model_id: Vec<u8>,
            product_id: Vec<u8>,
            content_hash: T::Hash,
            tx_id: Vec<u8>,
            memo: T::Hash,
            extra_compute_param: u8,
            auth_server: AccountId,
            auth_sign: sr25519::Signature) -> dispatch::DispatchResult {

            // Check it was signed and get the signer. See also: ensure_root and ensure_none
            let who = ensure_signed(origin)?;

            // Validation checks:
            // check if knowledge_id is existed already.
            ensure!(!<KnowledgeBaseDataByIdHash<T>>::contains_key((app_id.clone(), knowledge_id.clone())), "Knowledge base data already existed.");

            // construct verification u8 array:
            let mut buf = vec![];
            buf.append(&mut(app_id.clone()));
            buf.append(&mut(knowledge_id.clone()));
            buf.append(&mut vec![knowledge_type, extra_compute_param]);

            // auth sign check with auth_server & auth_sign
            ensure!(Self::auth_server_verify(auth_server, auth_sign, &buf), "auth server signature verification fail");

            let k = KnowledgeBaseData {
                owner: who.clone(),
                knowledge_type: knowledge_type.into(),
                app_id: app_id.clone(),
                knowledge_id: knowledge_id.clone(),
                model_id,
                product_id,
                content_hash,
                tx_id,
                extra_compute_param,
                memo
            };

            // init this knowledge power map
            let p = KnowledgePowerData {
                app_id: app_id.clone(),
                knowledge_id: knowledge_id.clone(),
                owner: who.clone(),
                ..Default::default()
            };

            let key = (app_id, knowledge_id);

            <KnowledgeBaseDataByIdHash<T>>::insert(key.clone(), k);
            <KnowledgePowerDataByIdHash<T>>::insert(key, p);

            Self::deposit_event(RawEvent::KnowledgeCreated(who));

            Ok(())
        }

        #[weight = 0]
        pub fn create_comment(origin,
            app_id: Vec<u8>,
            comment_id: Vec<u8>,
            knowledge_id: Vec<u8>,
            last_comment_id: Vec<u8>,
            comment_hash: T::Hash,
            cost: u32,
            knowledge_owner_profit: u32,
            auth_server: AccountId,
            auth_sign: sr25519::Signature) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            let knowledge_key = (app_id.clone(), knowledge_id.clone());

            // make sure matched knowledge exist
            ensure!(<KnowledgePowerDataByIdHash<T>>::contains_key(knowledge_key.clone()), "Knowledge power data not found.");

            let comment_key = (app_id.clone(), comment_id.clone());
            // make sure same comment id not exist
            ensure!(!<KnowledgeCommentDataByIdHash<T>>::contains_key(comment_key.clone()), "Knowledge comment already exsited.");

            let mut buf = vec![];
            buf.append(&mut(app_id.clone()));
            buf.append(&mut(knowledge_id.clone()));
            buf.append(&mut(comment_id.clone()));
            buf.append(&mut(knowledge_owner_profit.to_be_bytes().to_vec()));
            // TODO: more fields to verify
            ensure!(Self::auth_server_verify(auth_server, auth_sign, &buf), "auth server signature verification fail");

            let user_comment_count_key = (who.clone(), app_id.clone(), knowledge_id.clone());

            // store comment
            <KnowledgeCommentDataByIdHash<T>>::insert((app_id.clone(), comment_id.clone()), KnowledgeCommentData {
                app_id: app_id.clone(),
                knowledge_id: knowledge_id.clone(),
                comment_id: comment_id.clone(),
                last_comment_id,
                comment_hash,
                comment_fee: cost,
                knowledge_profit: knowledge_owner_profit,
                owner: who.clone(),
            });

            <KnowledgeCommentUserCountHash<T>>::mutate(user_comment_count_key.clone(), |cc| {
                *cc += 1;
            });
            // read it out
            let repeat_count = Self::knowledge_comment_user_count_hash(user_comment_count_key);
            print(repeat_count);

            let k = Self::knowledge_basedata_by_idhash(knowledge_key.clone());

            // update kp
            let mut before_update_power: u32 = 0;
            let mut new_power: u32 = 0;

            <KnowledgePowerDataByIdHash<T>>::mutate(knowledge_key.clone(), |kp| {
                kp.owner_profit += knowledge_owner_profit;
                kp.comment_total_count += 1;
                kp.comment_total_cost += cost;

                if cost > kp.comment_max_cost {
                    kp.comment_max_cost = cost;
                    kp.comment_cost_increase_count += 1;
                }

                if who == kp.owner {
                    kp.comment_self_count += 1;
                }

                if repeat_count > 1 {
                    kp.comment_repeat_user_count += 1;
                } else {
                    kp.comment_total_user += 1;
                }

                before_update_power = kp.power;
                kp.power = power_update::<T>(&kp, k.extra_compute_param as u32);
                new_power = kp.power;
                print(before_update_power);
                print(kp.power);
            });

            // update miner power, update diff only
            if new_power > before_update_power {
                print("power need update");
                <MinerPowerByAccount<T>>::mutate(k.owner, |mp| {
                    *mp += new_power - before_update_power;
                });
            }

            Self::deposit_event(RawEvent::CommentCreated(who));
            Ok(())
        }
      }
}

impl<T: Trait> Module<T> {
    pub fn is_auth_server(who: &T::AccountId) -> bool {
        <AuthServers<T>>::get().contains(who)
    }

    pub fn auth_server_verify(server: AccountId, sign: sr25519::Signature, msg: &[u8]) -> bool {
        let ms: MultiSignature = sign.into();
        ms.verify(msg, &server)
    }
}
