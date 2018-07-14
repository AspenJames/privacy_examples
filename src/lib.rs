mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function(); // public
    outermost::middle_secret_function(); // private
    outermost::inside::inner_function(); // public
    outermost::inside::secret_function(); // private
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
