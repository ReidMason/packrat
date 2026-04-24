use packrat_domain::Item;

/// What use cases need from the outside world to look up items. Infrastructure implements this.
pub trait ItemQueryPort: Send + Sync {
    fn fetch_example(&self) -> Option<Item>;
}
