use reqwest::Url;

/// A struct containing the URLs of the different pages of the service.
pub struct Pages {
    service_url: Url,
    login_url: Url,
    main_menu_url: Url,
    planning_choice_url: Url,
    planning_url: Url,
}

#[allow(dead_code)]
impl Pages {
    pub fn new<S: Into<String>>(service_url: S) -> Self {
        let service_url = service_url.into();
        let login_url = format!("{}/login", service_url.clone());
        let main_menu_url = format!("{}/faces/MainMenuPage.xhtml", service_url.clone());
        let planning_choice_url = format!("{}/faces/ChoixPlanning.xhtml", service_url.clone());
        let planning_url = format!("{}/faces/Planning.xhtml", service_url.clone());
        Self {
            service_url: Url::parse(&service_url).unwrap(),
            login_url: Url::parse(&login_url).unwrap(),
            main_menu_url: Url::parse(&main_menu_url).unwrap(),
            planning_choice_url: Url::parse(&planning_choice_url).unwrap(),
            planning_url: Url::parse(&planning_url).unwrap(),
        }
    }

    pub fn service_url(&self) -> Url {
        self.service_url.clone()
    }

    pub fn login_url(&self) -> Url {
        self.login_url.clone()
    }

    pub fn main_menu_url(&self) -> Url {
        self.main_menu_url.clone()
    }

    pub fn planning_choice_url(&self) -> Url {
        self.planning_choice_url.clone()
    }

    pub fn planning_url(&self) -> Url {
        self.planning_url.clone()
    }
}
