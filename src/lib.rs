pub mod lib {

    use std::{collections::HashSet};

    use caps::{CapSet, Capability};

    type ExResult<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

    pub fn remove_capabilities(needed_caps: &[Capability]) -> Result<(),i32>{
        let rm_cap = || -> ExResult<HashSet<Capability>> {
            let permited = caps::read(None, CapSet::Permitted)?;
            for cap in permited.iter() {
                if needed_caps.contains(cap) == false {
                    caps::drop(None, CapSet::Effective, *cap)?;
                    caps::drop(None, CapSet::Permitted, *cap)?;
                }
            }
            Ok(caps::read(None, CapSet::Effective)?)
        };

        let res = rm_cap();
        if let Err(_) = res {
            return Err(Code::CAPS_ERROR);
        }
        let effective: HashSet<Capability> = res.unwrap();

        if effective.len() != needed_caps.len() {
            return Err(Code::CAPS_ERROR);
        }

        let contain_only_authorized_caps = effective.iter().fold(true, |acc, cap| {
            return acc && needed_caps.contains(cap);
        });

        if contain_only_authorized_caps == false {
            return Err(Code::CAPS_ERROR);
        }
        Ok(())
    }

    pub struct Code {}

    #[cfg(debug_assertions)]
    impl Code {
        pub const SUCESS: i32 = 0;
        pub const INVALID_ARGUMENT: i32 = 1;
        pub const CAPS_ERROR: i32 = 2;
        pub const FAILED_GET_TIME: i32 = 3;
        pub const FAILED_SET_TIME: i32 = 4;
        pub const PARSE_ERROR: i32 = 5;
    }

    #[cfg(not(debug_assertions))]
    impl Code {
        pub const SUCESS: i32 = 0;
        pub const INVALID_ARGUMENT: i32 = 0;
        pub const CAPS_ERROR: i32 = 0;
        pub const FAILED_GET_TIME: i32 = 0;
        pub const FAILED_SET_TIME: i32 = 0;
        pub const PARSE_ERROR: i32 = 0;
    }
}
