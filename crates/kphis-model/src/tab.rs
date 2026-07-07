#[derive(Clone, Default, PartialEq)]
pub enum Tab {
    MedHx,
    MedReconcile,
    #[default]
    Order,
    VitalSign,
    Io,
    NurseNote,
    NursePlan,
    Lab,
    // LabCovid,
    XRay,
    // Ekg,
    // Scan,
    Emr,
    Document,
    // Operation,
    Doctor,
    Consult,
    ReferOut,
    // Cart,
    // Food,
}

impl Tab {
    pub fn from_string(tab: &str) -> Self {
        match tab {
            "med-hx" => Self::MedHx,
            "med-reconcile" => Self::MedReconcile,
            "order" => Self::Order,
            "vital-sign" => Self::VitalSign,
            "io" => Self::Io,
            "nurse-note" => Self::NurseNote,
            "nurse-plan" => Self::NursePlan,
            "lab" => Self::Lab,
            // lab-covid" => Self::LabCovid,
            "x-ray" => Self::XRay,
            // "ekg" => Self::Ekg,
            // "scan" => Self::Scan,
            "emr" => Self::Emr,
            "document" => Self::Document,
            // "operation" => Self::Operation,
            "doctor" => Self::Doctor,
            "consult" => Self::Consult,
            "refer-out" => Self::ReferOut,
            // "cart" => Self::Cart,
            // "food" => Self::Food,
            _ => Self::Order,
        }
    }

    pub fn str(&self) -> &'static str {
        match self {
            Self::MedHx => "med-hx",
            Self::MedReconcile => "med-reconcile",
            Self::Order => "order",
            Self::VitalSign => "vital-sign",
            Self::Io => "io",
            Self::NurseNote => "nurse-note",
            Self::NursePlan => "nurse-plan",
            Self::Lab => "lab",
            // Self::LabCovid => lab-covid",
            Self::XRay => "x-ray",
            // Self::Ekg => "ekg",
            // Self::Scan => "scan",
            Self::Emr => "emr",
            Self::Document => "document",
            // Self::Operation => "operation",
            Self::Doctor => "doctor",
            Self::Consult => "consult",
            Self::ReferOut => "refer-out",
            // Self::Cart => "cart",
            // Self::Food => "food",
        }
    }
}
