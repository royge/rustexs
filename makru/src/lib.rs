#[macro_export]
macro_rules! my_vec {
    ($($x:expr), *) => {
        {
            let mut tmp = Vec::new();
            $(
                tmp.push($x);
            )*
            tmp
        }
    };
}
