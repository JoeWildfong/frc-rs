#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wpilibc/headers/frc/drive/DifferentialDrive.h");
        type DifferentialDrive;
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
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
