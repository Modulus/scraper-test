pub trait StringDataCollector {
    fn collect(&self) -> impl std::future::Future<Output = Result<Vec<String>, Box<dyn std::error::Error>>> + Send;
}