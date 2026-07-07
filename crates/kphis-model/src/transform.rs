use serde_derive::Deserialize;
use sqlx::FromRow;

// #[derive(Clone, Debug, Deserialize, FromRow)]
// pub struct VnAnMismatch {
//     /// `vn` in vn_an
//     pub vn: String,
//     /// current AN of `vn`
//     pub ovst_an: Option<String>,
//     /// current HN of `vn` (from ovst_an)
//     pub ovst_hn: Option<String>,
//     /// current ward of `vn` (from ovst_an)
//     pub ovst_ward: Option<String>,
//     /// `an` in vn_an
//     pub an: String,
//     /// current VN of `an`
//     pub ipt_vn: Option<String>,
//     /// current HN of `an`
//     pub ipt_hn: Option<String>,
//     /// current ward of `an`
//     pub ipt_ward: Option<String>,
// }

#[derive(Deserialize, FromRow)]
pub struct IptLog {
    pub ipt_log_id: u32,
    pub ipt_log_type: String, // I or D
    pub an: String,
    pub vn: String,
    pub hn: Option<String>,
    pub ward: Option<String>,
}

// impl PartialEq for IptLog {
//     fn eq(&self, other: &Self) -> bool {
//         // self.ipt_log_id == other.ipt_log_id &&
//         self.ipt_log_type == other.ipt_log_type
//             && self.an == other.an
//             && self.vn == other.vn
//             && self.hn == other.hn
//             && self.ward == other.ward
//     }
// }

#[derive(Debug, Deserialize, FromRow)]
pub struct AnInPreAdmitAndIpt {
    // -2-
    /// JOINED VN from `pm_an`, represent `pre_admit_master.vn`<br>
    /// Always `Some` when `pm_an` exixts
    pub pm_vn: Option<String>,
    // -1-
    /// Where AN matched `pre_admit_master.an`
    pub pm_an: Option<String>,
    // -3-
    /// ipt.an from JOINED `pm_vn`<br>
    /// Exists when `ipt.vn == pm_vn` exists
    pub pm_ipt_an: Option<String>,
    // (2)
    /// JOINED VN from `ipt_an`, `ipt` side<br>
    /// Always `Some` when `ipt_an` exixts
    pub ipt_vn: Option<String>,
    // (3)
    /// JOINED VN from `ipt_an`, `pre_admit_master` side<br>
    /// None when `pre_admit_master.vn == ipt.vn` not exists
    pub ipt_pm_vn: Option<String>,
    // (4)
    /// `pre_admit_master.an` from JOINED `ipt_pm_vn`<br>
    /// Represent Admit status of `ipt_pm_vn`
    pub ipt_pm_an: Option<String>,
    // (1)
    /// Where AN matched `ipt.an`
    pub ipt_an: Option<String>,
}

impl PartialEq for AnInPreAdmitAndIpt {
    fn eq(&self, other: &Self) -> bool {
        self.pm_vn == other.pm_vn
            && self.pm_an == other.pm_an
            && self.pm_ipt_an == other.pm_ipt_an
            && self.ipt_vn == other.ipt_vn
            && self.ipt_pm_vn == other.ipt_pm_vn
            && self.ipt_pm_an == other.ipt_pm_an
            && self.ipt_an == other.ipt_an
    }
}

#[derive(Debug, Deserialize, FromRow)]
pub struct VnInPreAdmitAndIpt {
    // -1-
    /// Where VN matched `pre_admit_master.vn`
    pub pm_vn: Option<String>,
    // -2-
    /// JOINED AN from `pm_vn`, represent Admit status of `pm_vn`
    pub pm_an: Option<String>,
    // -3-
    /// `ipt.vn` from JOINED `pm_an`<br>
    /// exists when `ipt.an == pm_an` exists
    pub pm_ipt_vn: Option<String>,
    // (1)
    /// Where VN matched `ipt.vn`
    pub ipt_vn: Option<String>,
    // (3)
    /// `pre_admit_master.vn` from JOINED `ipt_an`<br>
    /// represent `pre_admit_master.vn` that Admit status with `AN == ipt_an` exists
    pub ipt_pm_vn: Option<String>,
    // (2)
    /// JOINED AN from `ipt_vn`<br>
    /// Always `Some` when `ipt_vn` exixts
    pub ipt_an: Option<String>,
}

impl PartialEq for VnInPreAdmitAndIpt {
    fn eq(&self, other: &Self) -> bool {
        self.pm_vn == other.pm_vn && self.pm_an == other.pm_an && self.pm_ipt_vn == other.pm_ipt_vn && self.ipt_vn == other.ipt_vn && self.ipt_pm_vn == other.ipt_pm_vn && self.ipt_an == other.ipt_an
    }
}
