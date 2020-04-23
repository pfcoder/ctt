use frame_support::assert_ok;
use sp_core::H256;
use sp_io::hashing::blake2_256;

use crate::{mock::*, KnowledgeBaseData, KnowledgeExtraComputeParam, KnowledgeType};

#[test]
fn ctt_test_knowledge_create() {
    new_test_ext().execute_with(|| {
        let kid_hash = H256::from_slice(&blake2_256(String::from("GUID-01").as_bytes()));
        let test_hash = H256::from_slice(&blake2_256(String::from("da038934asd1").as_bytes()));
        assert_ok!(KpModule::create_knowledge(
            Origin::signed(1),
            0,
            kid_hash,
            test_hash,
            test_hash,
            None,
            test_hash,
            KnowledgeExtraComputeParam::ProductPublishRatio(10)
        ));
        // asserting that the stored value is equal to what we stored
        let expected_knowledge = KnowledgeBaseData {
            owner: 0,
            knowledge_type: KnowledgeType::ProductPublish,
            knowledge_id: test_hash,
            product_id: test_hash,
            content_hash: test_hash,
            tx_id: None,
            memo: test_hash,
            extra_compute_param: KnowledgeExtraComputeParam::ProductPublishRatio(10),
        };

        let read = KpModule::knowledge_basedata_by_idhash(kid_hash);
        println!("read result:{} {}", read.owner, read.content_hash);

        assert_eq!(read, expected_knowledge);
    });
}
