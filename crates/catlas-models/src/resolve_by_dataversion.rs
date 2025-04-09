pub(crate) trait ResolveByVersion<T, E>
where
    E: serde::de::Error,
{
    fn resolve_by_version(self, data_version: u32) -> Result<T, E>;
}

impl<I, O, E> ResolveByVersion<Vec<O>, E> for Vec<I>
where
    I: ResolveByVersion<O, E>,
    E: serde::de::Error,
{
    fn resolve_by_version(self, data_version: u32) -> Result<Vec<O>, E> {
        self.into_iter()
            .map(|element| element.resolve_by_version(data_version))
            .collect()
    }
}
