use crate::mcd_bindings::{mcd_core_con_info_st, DynamicMCDxDAS};

use super::McdReturnError;

impl DynamicMCDxDAS {
    /// See [DynamicMCDxDAS::mcd_qry_systems_f], with num_systems set to 0
    pub fn query_system_count(&self) -> Result<u32, McdReturnError> {
        let mut num_systems = 0;

        mcd_call!(unsafe { self.mcd_qry_systems_f(0, &mut num_systems, core::ptr::null_mut()) })?;

        Ok(num_systems)
    }

    /// See [DynamicMCDxDAS::mcd_qry_systems_f]
    pub fn query_systems(&self) -> Result<Vec<mcd_core_con_info_st>, McdReturnError> {
        let mut num_systems = self.query_system_count()?;

        let mut result = vec![mcd_core_con_info_st::default(); num_systems as usize];

        mcd_call!(unsafe { self.mcd_qry_systems_f(0, &mut num_systems, result.as_mut_ptr()) })?;

        Ok(result)
    }
}
