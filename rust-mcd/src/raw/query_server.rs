use crate::mcd_bindings::{mcd_server_info_st, mcd_server_st, DynamicMCDxDAS};

use super::McdReturnError;

/// Define available config parameters, see [DynamicMCDxDAS::mcd_qry_servers_f]
macro_rules! define_config {
    ($(($field_name: ident, $field_type:ty, $description: expr, $config_name: expr)),*) => {
        #[derive(Default)]
        pub struct ServerConfig {
            $(
                #[doc = $description]
                pub $field_name: Option<$field_type>
            ),*
        }

        impl ServerConfig {
            /// Returns a null terminated configuration string
            pub fn as_config_string(&self) -> String {
                let mut composed_string = String::new();

                $(
                    if let Some(value) = self.$field_name.as_ref() {
                        // FIXME: We might not need the double quotes for all field types
                        // FIXME: Only tested for string parameters
                        composed_string += &format!("{}=\"{}\"\n", $config_name, value);
                    }
                )*

                composed_string += "\0";

                composed_string
            }
        }
    };
}

// Partially list of available parameters, check [DynamicMCDxDAS::mcd_qry_servers_f] when a parameter should be added
define_config!(
    (acc_hw, String, "Restricts this server to connect to devices via a specific access hardware as determined by the string.", "McdAccHw")
);

impl DynamicMCDxDAS {
    /// See [DynamicMCDxDAS::mcd_qry_servers_f], with num_servers set to 0
    pub fn query_server_count(&self) -> Result<u32, McdReturnError> {
        log::trace!("Scanning for open servers");
        let host = b"localhost\0";
        let mut num_open_servers = 0u32;

        mcd_call!(unsafe {
            self.mcd_qry_servers_f(
                host.as_ptr() as *const i8,
                1,
                0,
                &mut num_open_servers,
                core::ptr::null_mut(),
            )
        })?;

        Ok(num_open_servers)
    }

    /// See [DynamicMCDxDAS::mcd_qry_servers_f]
    pub fn query_server_infos(&self) -> Result<Vec<mcd_server_info_st>, McdReturnError> {
        let mut open_server_count = self.query_server_count()?;

        let mut target = vec![mcd_server_info_st::default(); open_server_count as usize];

        let host = b"localhost\0";

        mcd_call!(unsafe {
            self.mcd_qry_servers_f(
                host.as_ptr() as *const i8,
                1,
                0,
                &mut open_server_count,
                target.as_mut_ptr(),
            )
        })?;

        Ok(target)
    }

    /// See [DynamicMCDxDAS::mcd_open_server_f]
    pub fn open_server(&self, config: ServerConfig) -> Result<*mut mcd_server_st, McdReturnError> {
        let config_string = config.as_config_string();
        let system_key: i8 = 0;
        let mut server_info = core::ptr::null_mut::<mcd_server_st>();
        mcd_call!(unsafe {
            self.mcd_open_server_f(
                &system_key as *const i8,
                config_string.as_bytes().as_ptr() as *const i8,
                &mut server_info as _,
            )
        })?;

        Ok(server_info)
    }
}
