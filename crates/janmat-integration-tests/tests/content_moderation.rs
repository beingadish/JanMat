//! Integration: content safety pipeline + moderation jury flow.

use janmat_content::{ContentPipeline, SubmitRequest, Verdict, blocklist::BlocklistEngine};
use pallet_moderation::{JuryVerdict, ModerationPallet};

#[test]
fn blocked_phrase_rejected_end_to_end() {
    let pipeline = ContentPipeline::new(BlocklistEngine::with_phrases(vec![
        "banned-content".to_owned(),
    ]));
    let report = pipeline.check(&SubmitRequest {
        subtopic_id: "st/1",
        text: "This is banned-content and should not pass",
    });
    assert_eq!(report.verdict, Verdict::Blocked);
}

#[test]
fn clean_content_passes_pipeline() {
    let pipeline = ContentPipeline::new(BlocklistEngine::default());
    let report = pipeline.check(&SubmitRequest {
        subtopic_id: "st/1",
        text: "The NEP 2020 policy reforms are necessary for rural education.",
    });
    assert_eq!(report.verdict, Verdict::Clean);
}

#[test]
fn moderation_full_jury_flow_suppress() {
    let mut moderation = ModerationPallet::default();
    let pool: Vec<String> = (0..10).map(|i| format!("voter-{i:03}")).collect();

    moderation
        .flag_opinion(
            "o/test-001",
            "coordinating_harm",
            "mod-A",
            &pool,
            b"blockhash",
            500,
        )
        .unwrap();

    let jurors = moderation
        .audit_entries_for("o/test-001")
        .iter()
        .find(|e| e.action == pallet_moderation::ModerationAction::JuryConvened)
        .map(|e| {
            e.reason_code
                .trim_start_matches("jurors:")
                .split(',')
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap();

    // 4 jurors vote to suppress
    for j in jurors.iter().take(4) {
        moderation.jury_vote("o/test-001", j, true).unwrap();
    }
    // 3 jurors vote to keep
    for j in jurors.iter().skip(4) {
        if let Ok(Some(verdict)) = moderation.jury_vote("o/test-001", j, false) {
            moderation.finalise_verdict("o/test-001", verdict, 600);
            break;
        }
    }

    let log = moderation.audit_entries_for("o/test-001");
    assert!(
        log.iter()
            .any(|e| e.action == pallet_moderation::ModerationAction::Suppressed)
    );
}

#[test]
fn moderation_3_of_7_keeps_opinion() {
    let mut moderation = ModerationPallet::default();
    let pool: Vec<String> = (0..10).map(|i| format!("w-{i:03}")).collect();
    moderation
        .flag_opinion("o/keep-001", "review", "mod-B", &pool, b"hash2", 100)
        .unwrap();

    let jurors = {
        let entry = moderation
            .audit_entries_for("o/keep-001")
            .into_iter()
            .find(|e| e.action == pallet_moderation::ModerationAction::JuryConvened)
            .unwrap();
        entry
            .reason_code
            .trim_start_matches("jurors:")
            .split(',')
            .map(|s| s.to_owned())
            .collect::<Vec<_>>()
    };

    // Only 3 vote suppress (< 4 threshold)
    for j in jurors.iter().take(3) {
        moderation.jury_vote("o/keep-001", j, true).unwrap();
    }
    for j in jurors.iter().skip(3) {
        if let Ok(Some(verdict)) = moderation.jury_vote("o/keep-001", j, false) {
            assert_eq!(verdict, JuryVerdict::Keep);
            moderation.finalise_verdict("o/keep-001", verdict, 200);
            break;
        }
    }

    let log = moderation.audit_entries_for("o/keep-001");
    assert!(
        log.iter()
            .any(|e| e.action == pallet_moderation::ModerationAction::Kept)
    );
}
