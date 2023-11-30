use crate::models::PaywallArticle;
use crate::paywall::{
    PaywallItem, PaywallServer, RecursiveFileReader, SessionConditionalManipulation,
    SessionConditionalObject,
};

pub fn paywall_server_factory<
    S: Clone,
    T: Clone + Send + Sync,
    U: SessionConditionalObject<T>,
    V: PaywallServer<T, U>,
>(
    filereader: impl RecursiveFileReader<S>,
    manipulator: impl SessionConditionalManipulation<S, T, U>,
    paywall_extraction: fn(S, &str) -> Option<PaywallArticle>,
) -> V {
    let paths_and_files = filereader.get_paths_and_files();

    let paths = paths_and_files.iter().map(|x| x.file_path.clone());
    let objects = paths_and_files.iter().map(|x| {
        PaywallItem::new(
            manipulator.manipulate_object(x.file_content.clone()),
            (paywall_extraction)(x.file_content.clone(), &x.file_path),
        )
    });

    let items = paths.zip(objects).collect();

    return V::new_from_paywall_items(items);
}

