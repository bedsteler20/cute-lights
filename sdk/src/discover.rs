use crate::{
    config::CuteLightsConfig,
    integrations::{govee::GoveeIntegration, hue::HueIntegration, kasa::KasaIntegration, Integration, Light},
    utils::future::FutureBatch,
};

struct Discoverer {
    config: &'static CuteLightsConfig,
    batch: FutureBatch<Vec<Box<dyn Light>>>,
}

impl Discoverer {
    fn new(config: &'static CuteLightsConfig) -> Self {
        Self {
            config,
            batch: FutureBatch::new(),
        }
    }

    fn register<I: Integration + Send + Sync + 'static>(&mut self) {
        let config = self.config;
        if I::preflight(&self.config) {
            self.batch.push(async move {
                if let Ok(lights) = I::discover(&config).await {
                    lights
                } else {
                    eprintln!("Failed to discover lights for {}", I::name());
                    Vec::new()
                }
            });
        }
    }

    async fn run(self) -> Vec<Box<dyn Light>> {
        self.batch.run().await.into_iter().flatten().collect()
    }
}

pub async fn discover_lights() -> Vec<Box<dyn Light>> {
    let config = Box::leak(Box::new(CuteLightsConfig::load_default()));
    let mut discoverer = Discoverer::new(config);

    discoverer.register::<KasaIntegration>();
    discoverer.register::<HueIntegration>();
    discoverer.register::<GoveeIntegration>();

    discoverer.run().await
}
