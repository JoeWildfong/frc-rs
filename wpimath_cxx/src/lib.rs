#[cxx::bridge(namespace = "units")]
mod ffi {
    struct volts {
        // cxx doesn't support std::ffi::c_double, this is almost always the same
        value: f64,
    }

    unsafe extern "C++" {
        include!("units/voltage.h");

        pub type volts;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
