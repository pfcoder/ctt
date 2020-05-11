use frame_support::assert_ok;
use sp_core::H256;
use sp_io::hashing::blake2_256;

use sp_core::sr25519::Signature;

use sp_runtime::MultiSignature;

use crate::{mock::*, KnowledgeBaseData, KnowledgeType};

/*#[test]
fn ctt_test_knowledge_create() {
    new_test_ext().execute_with(|| {
        let kid_hash = H256::from_slice(&blake2_256(String::from("K01").as_bytes()));
        let test_hash = H256::from_slice(&blake2_256(String::from("da038934asd1").as_bytes()));
        assert_ok!(KpModule::create_knowledge(
            Origin::signed(1),
            0,
            String::from("K01").into_bytes(),
            String::from("M01").into_bytes(),
            String::from("P01").into_bytes(),
            test_hash,
            String::from("").into_bytes(),
            test_hash,
            0,
            1,
            Signature::default(),
        ));
        // asserting that the stored value is equal to what we stored
        let expected_knowledge = KnowledgeBaseData {
            owner: 0,
            knowledge_type: KnowledgeType::ProductPublish,
            knowledge_id: String::from("K01").into_bytes(),
            model_id: String::from("M01").into_bytes(),
            product_id: String::from("P01").into_bytes(),
            content_hash: test_hash,
            tx_id: String::from("").into_bytes(),
            memo: test_hash,
            extra_compute_param: 0,
        };

        let read = KpModule::knowledge_basedata_by_idhash(kid_hash);
        println!("read result:{} {}", read.owner, read.content_hash);

        assert_eq!(read, expected_knowledge);
    });
}*/
