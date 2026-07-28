use super::Float32;

impl std::fmt::Display for Float32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prec = f.precision().unwrap_or(crate::DEFAULT_DISPLAY_DECIMALS);
        write!(f, "{:.prec$}", self.0)
    }
}

impl std::ops::DerefMut for Float32 {
    #[inline]
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}
