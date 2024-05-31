use ens160::AirqualityIndex;

pub fn airquality_to_string(quality: AirqualityIndex) -> &'static str {
    match quality {
        AirqualityIndex::Excellent => "Excellent",
        AirqualityIndex::Good => "Good",
        AirqualityIndex::Moderate => "Moderate",
        AirqualityIndex::Poor => "Poor",
        AirqualityIndex::Unhealthy => "Unhealthy",
    }
}

pub fn validity_to_string(validity: ens160::Validity) -> &'static str {
    match validity {
        ens160::Validity::NormalOperation => "Operating",
        ens160::Validity::WarmupPhase => "Warming up",
        ens160::Validity::InitStartupPhase => "Calibrating",
        ens160::Validity::InvalidOutput => "Invalid out",
    }
}
