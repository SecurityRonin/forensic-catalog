#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MalwareClass {
    LdPreloadProcessHider,
    LdPreloadPamHooker,
    LdPreloadNetworkHider,
    LdPreloadFullRootkit,
    LkmRootkit,
    CryptoMiner,
    GenericLdPreload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    NoMatch,
    LowConfidence,
    ClassMatch,
    Probable,
    Confirmed,
}

#[derive(Debug)]
pub struct ProfileSignal {
    pub id: &'static str,
    pub weight: u32,
    pub required: bool,
}

#[derive(Debug)]
pub struct WeightedExclusion {
    pub id: &'static str,
    pub penalty: u32,
}

#[derive(Debug)]
pub struct MalwareProfile {
    pub id: &'static str,
    pub family: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub malware_class: MalwareClass,
    pub mitre_techniques: &'static [&'static str],
    pub signals: &'static [ProfileSignal],
    pub exclusions: &'static [WeightedExclusion],
    pub class_threshold: u32,
    pub probable_threshold: u32,
    pub confirmed_threshold: u32,
}

#[derive(Debug)]
pub struct FiredSignal {
    pub id: &'static str,
    pub weight: u32,
}

#[derive(Debug)]
pub struct ProfileMatch {
    pub profile: &'static MalwareProfile,
    pub score: u32,
    pub classification: Classification,
    pub fired: Vec<FiredSignal>,
    pub missed_required: Vec<&'static str>,
}
