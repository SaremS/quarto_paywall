use crate::models::PaywallArticle;
use crate::paywall::{
    PaywallItem, PaywallServer, RecursiveFileReader, SessionConditionalManipulation,
    SessionConditionalObject,
};

pub struct PaywallServerFactory<
    S: Clone,
    T: Clone + Send + Sync,
    U: SessionConditionalObject<T>,
    V: PaywallServer<T, U>,
> {
    filereader: Box<dyn RecursiveFileReader<S>>,
    manipulator: Box<dyn SessionConditionalManipulation<S, T, U>>,
    paywall_extractor: fn(S) -> Option<PaywallArticle>,
    _marker: std::marker::PhantomData<V>,
}

impl<S: Clone, T: Clone + Send + Sync, U: SessionConditionalObject<T>, V: PaywallServer<T, U>>
    PaywallServerFactory<S, T, U, V>
{
    pub fn make_server(&self, file_extensions: Vec<&str>) -> V {
        let paths_and_files = self.filereader.get_paths_and_files(file_extensions);

        let paths = paths_and_files.iter().map(|x| x.file_path.clone());
        let objects = paths_and_files.iter().map(|x| {
            PaywallItem::new(
                self.manipulator.manipulate_object(x.file_content.clone()),
                (self.paywall_extractor)(x.file_content.clone()),
            )
        });

        let items = paths.zip(objects).collect();

        return V::new_from_paywall_items(items);
    }
}
