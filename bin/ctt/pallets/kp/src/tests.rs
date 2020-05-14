use frame_support::assert_ok;
use sp_core::H256;
use sp_io::hashing::blake2_256;

use sp_core::{sr25519, Pair};

use crate::mock::*;

#[test]
fn ctt_test_knowledge_create() {
    new_test_ext().execute_with(|| {
        let test_hash = H256::from_slice(&blake2_256(String::from("da038934asd1").as_bytes()));
        let test_signer_pair =
            sr25519::Pair::from_string(&format!("//{}", "Alice"), None).expect("valid seed");

        let app_id = String::from("A01").into_bytes();
        let knowledge_id = String::from("K01").into_bytes();
        let knowledge_type: u8 = 0;
        let extra_compute_param: u8 = 10;

        let mut buf = vec![];
        buf.append(&mut (app_id.clone()));
        buf.append(&mut (knowledge_id.clone()));
        buf.append(&mut vec![knowledge_type, extra_compute_param]);

        let test_signature = test_signer_pair.sign(&buf);

        assert_ok!(KpModule::create_knowledge(
            Origin::signed(1),
            app_id.clone(),
            0,
            knowledge_id.clone(),
            String::from("M01").into_bytes(),
            String::from("P01").into_bytes(),
            test_hash,
            String::from("").into_bytes(),
            test_hash,
            extra_compute_param,
            test_signer_pair.public().into(),
            test_signature,
        ));
        // asserting that the stored value is equal to what we stored

        let read = KpModule::knowledge_basedata_by_idhash((app_id, knowledge_id));
        println!("read result:{} {}", read.owner, read.content_hash);

        assert_eq!(read.extra_compute_param, extra_compute_param);
    });
}
