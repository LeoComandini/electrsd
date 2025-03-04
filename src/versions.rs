#[cfg(target_os = "macos")]
const OS: &str = "macos";

#[cfg(target_os = "linux")]
const OS: &str = "linux";

#[cfg(feature = "electrs_0_8_10")]
const VERSION: &str = "v0.8.10";

#[cfg(feature = "esplora_a33e97e1")]
const VERSION: &str = "esplora_a33e97e1a1fc63fa9c20a116bb92579bbf43b254";

#[cfg(not(any(feature = "electrs_0_8_10", feature = "esplora_a33e97e1",)))]
const VERSION: &str = "NA";

pub const HAS_FEATURE: bool = cfg!(any(
    feature = "electrs_0_8_10",
    feature = "esplora_a33e97e1",
));

pub fn electrs_name() -> String {
    format!("electrs_{}_{}", OS, VERSION)
}
