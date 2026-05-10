use std::collections::HashSet;
use super::profile::{Classification, MalwareProfile, ProfileMatch, FiredSignal};

#[derive(Debug)]
pub struct DetectedSignal {
    pub id: &'static str,
    pub confidence: f32,
    pub evidence: String,
}

pub fn score_against_profile(
    _signals: &[DetectedSignal],
    _profile: &'static MalwareProfile,
) -> ProfileMatch {
    todo!("implement score_against_profile")
}

pub fn score_all_profiles(_signals: &[DetectedSignal]) -> Vec<ProfileMatch> {
    todo!("implement score_all_profiles")
}

pub fn top_match(_signals: &[DetectedSignal]) -> Option<ProfileMatch> {
    todo!("implement top_match")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::threat_intel::{
        profile::Classification,
        signals::*,
    };

    fn sig(id: &'static str) -> DetectedSignal {
        DetectedSignal { id, confidence: 1.0, evidence: String::new() }
    }

    fn sigs(ids: &[&'static str]) -> Vec<DetectedSignal> {
        ids.iter().map(|&id| sig(id)).collect()
    }

    #[test]
    fn score_zero_signals_returns_no_match_for_all_profiles() {
        let matches = score_all_profiles(&[]);
        assert!(matches.is_empty(), "no signals → no profiles should match");
    }

    #[test]
    fn score_father_required_signals_returns_father_class_match() {
        use crate::threat_intel::profiles::FATHER;
        let signals = sigs(&[ELF_HOOKS_PROCESS_HIDING, ELF_HOOKS_PAM_CREDENTIAL]);
        let m = score_against_profile(&signals, &FATHER);
        assert!(m.score >= FATHER.class_threshold);
        assert!(m.classification >= Classification::ClassMatch);
    }

    #[test]
    fn score_father_full_signals_returns_father_confirmed() {
        use crate::threat_intel::profiles::FATHER;
        let signals = sigs(&[
            ELF_HOOKS_PROCESS_HIDING,
            ELF_HOOKS_PAM_CREDENTIAL,
            ARTIFACT_PAM_STAGING_STRUCTURAL,
            ARTIFACT_PAM_STAGING_FATHER,
            ELF_STRING_FATHER_FORMAT,
            ELF_STRING_STAGING_PATH,
            ELF_GLOBALLY_LOADED,
            ELF_NOT_IN_PKG_DB,
            TEMPORAL_LDPRELOAD_SSHD_RESTART,
        ]);
        let m = score_against_profile(&signals, &FATHER);
        assert_eq!(m.classification, Classification::Confirmed);
    }

    #[test]
    fn score_jynx_with_pam_signal_excluded_by_penalty() {
        use crate::threat_intel::profiles::JYNX;
        // Jynx with PAM signal: base score 60, penalty 40 → 20 < class_threshold 45
        let signals = sigs(&[
            ELF_HOOKS_PROCESS_HIDING,
            ELF_HOOKS_FILE_HIDING,
            ELF_HOOKS_PAM_CREDENTIAL,
        ]);
        let m = score_against_profile(&signals, &JYNX);
        assert!(m.classification < Classification::ClassMatch,
            "PAM signal should drop Jynx below class threshold, got {:?} (score {})",
            m.classification, m.score);
    }

    #[test]
    fn score_bdvl_required_signals_returns_bdvl_class_match() {
        use crate::threat_intel::profiles::BDVL;
        let signals = sigs(&[ELF_HOOKS_PROCESS_HIDING, ELF_HOOKS_NETWORK_HIDING]);
        let m = score_against_profile(&signals, &BDVL);
        assert!(m.score >= BDVL.class_threshold);
        assert!(m.classification >= Classification::ClassMatch);
    }

    #[test]
    fn score_xmrig_thread_signal_returns_xmrig_class_match() {
        use crate::threat_intel::profiles::XMRIG;
        let signals = sigs(&[PROCESS_THREAD_MINER_XMRIG]);
        let m = score_against_profile(&signals, &XMRIG);
        assert!(m.score >= XMRIG.class_threshold);
        assert!(m.classification >= Classification::ClassMatch);
    }

    #[test]
    fn score_all_profiles_sorted_descending_by_score() {
        let signals = sigs(&[
            ELF_HOOKS_PROCESS_HIDING,
            ELF_HOOKS_PAM_CREDENTIAL,
            ARTIFACT_PAM_STAGING_FATHER,
            ELF_STRING_FATHER_FORMAT,
        ]);
        let matches = score_all_profiles(&signals);
        assert!(!matches.is_empty());
        for w in matches.windows(2) {
            assert!(w[0].score >= w[1].score,
                "results not sorted: {} < {}", w[0].score, w[1].score);
        }
    }

    #[test]
    fn score_all_profiles_filters_zero_score() {
        let signals = sigs(&[ELF_HOOKS_PROCESS_HIDING, ELF_HOOKS_PAM_CREDENTIAL]);
        let matches = score_all_profiles(&signals);
        assert!(matches.iter().all(|m| m.score > 0));
    }

    #[test]
    fn top_match_returns_highest_classification() {
        let signals = sigs(&[
            ELF_HOOKS_PROCESS_HIDING,
            ELF_HOOKS_PAM_CREDENTIAL,
            ARTIFACT_PAM_STAGING_STRUCTURAL,
            ARTIFACT_PAM_STAGING_FATHER,
            ELF_STRING_FATHER_FORMAT,
        ]);
        let top = top_match(&signals);
        assert!(top.is_some());
        let top = top.unwrap();
        assert!(top.classification >= Classification::ClassMatch);
    }

    #[test]
    fn score_missing_required_signal_is_no_match() {
        use crate::threat_intel::profiles::FATHER;
        // Father requires both process hiding AND pam_credential
        let signals = sigs(&[ELF_HOOKS_PROCESS_HIDING]); // pam missing
        let m = score_against_profile(&signals, &FATHER);
        assert_eq!(m.classification, Classification::NoMatch);
        assert_eq!(m.score, 0);
        assert!(!m.missed_required.is_empty());
    }

    #[test]
    fn score_exclusion_reduces_score() {
        use crate::threat_intel::profiles::FATHER;
        // Father with network hiding signal: penalty 20 applied
        let without_exclusion = sigs(&[ELF_HOOKS_PROCESS_HIDING, ELF_HOOKS_PAM_CREDENTIAL]);
        let with_exclusion = sigs(&[ELF_HOOKS_PROCESS_HIDING, ELF_HOOKS_PAM_CREDENTIAL, ELF_HOOKS_NETWORK_HIDING]);
        let m_no_ex = score_against_profile(&without_exclusion, &FATHER);
        let m_ex    = score_against_profile(&with_exclusion, &FATHER);
        assert!(m_ex.score < m_no_ex.score, "exclusion should lower score");
    }

    #[test]
    fn score_exclusion_cannot_exceed_raw_score() {
        use crate::threat_intel::profiles::JYNX;
        // Jynx with only process hiding (30) + PAM penalty (40): saturating_sub → 0
        let signals = sigs(&[ELF_HOOKS_PROCESS_HIDING, ELF_HOOKS_FILE_HIDING, ELF_HOOKS_PAM_CREDENTIAL]);
        let m = score_against_profile(&signals, &JYNX);
        // score must be ≥ 0 (u32 can't go negative, but saturating_sub ensures no panic)
        let _ = m.score; // just assert it doesn't overflow/panic
    }
}
