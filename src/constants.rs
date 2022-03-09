use cfg_if;

cfg_if::cfg_if! {
    if #[cfg(feature = "web")] {
        pub static CLIMA_WEB: &str = "0clima.md";
    }
}
