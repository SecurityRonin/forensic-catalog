use crate::threat_intel::profile::MalwareProfile;

mod azazel;
mod bdvl;
mod father;
mod jynx;
mod ld_preload_generic;
mod libprocesshider;
mod lkm_generic;
mod xmrig;

pub use azazel::AZAZEL;
pub use bdvl::BDVL;
pub use father::FATHER;
pub use jynx::JYNX;
pub use ld_preload_generic::LD_PRELOAD_GENERIC;
pub use libprocesshider::LIB_PROCESS_HIDER;
pub use lkm_generic::LKM_GENERIC;
pub use xmrig::XMRIG;

pub const ALL_PROFILES: &[&MalwareProfile] = &[
    &FATHER, &JYNX, &AZAZEL, &BDVL, &LIB_PROCESS_HIDER,
    &XMRIG, &LKM_GENERIC, &LD_PRELOAD_GENERIC,
];
