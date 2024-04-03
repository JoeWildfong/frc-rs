#[cxx::bridge(namespace = "wpi")]
mod ffi {
    unsafe extern "C++" {
        include!("wpinet/DsClient.h");

        pub type DsClient;
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
