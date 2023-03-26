#![deny(missing_docs)]

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Error, Result};
use chrono::{DateTime, Utc};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use reqwest::redirect::Policy;
use reqwest::{Client, ClientBuilder};
use serde_json::{json, Value, Value::Bool};

use crate::default::{school_end, school_start};
use crate::event::{Event, RawEvent};
use crate::menu::{Menu, Node};
use crate::pages::Pages;
use crate::schedule::ClassGroup;
use crate::utils::{get_form_id, get_schedule_form_id, get_view_state};

/// The main Aurion struct.
pub struct Aurion {
    pages: Pages,
    menu: Menu,
    view_state: Option<String>,
    form_id: Option<u8>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    client: Client,
}

impl Aurion {
    /// Create a new Aurion instance.
    pub fn new<S: Into<String>, U: Into<String>, G: Into<String>, T: Into<String>>(
        language_code: u32,
        schooling_id: S,
        user_planning_id: U,
        groups_planning_id: G,
        service_url: T,
    ) -> Self {
        Self {
            pages: Pages::new(service_url),
            menu: Menu::new(
                language_code,
                schooling_id,
                user_planning_id,
                groups_planning_id,
            ),
            view_state: None,
            form_id: None,
            start: school_start(),
            end: school_end(),
            client: ClientBuilder::new()
                .cookie_store(true)
                .redirect(Policy::none())
                .build()
                .unwrap(),
        }
    }

    /// Create the default payload for Aurion requests.
    fn default_parameters<M: Into<String>>(&self, menu_id: M) -> Value {
        // This payload form ids seems to be constant (805, 808, 820).
        json!({
            "form": "form",
            "form:sauvegarde": "",
            "form:largeurDivCenter": "",
            "form:j_idt820_focus": "",
            "form:j_idt820_input": "",
            "form:sidebar": "form:sidebar",
            "form:j_idt805:j_idt808_view": "basicDay",
            "javax.faces.ViewState": self.view_state,
            "form:sidebar_menuid": menu_id.into(),
        })
    }

    /// Login to Aurion with the given credentials.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aurion_rs::Aurion;
    /// # async fn run() -> Result<(), reqwest::Error> {
    /// #     let mut aurion = Aurion::new(
    /// #         275805,
    /// #         "submenu_291906",
    /// #         "1_3",
    /// #         "submenu_299102",
    /// #         "https://web.isen-ouest.fr/webAurion/",
    /// #     );
    /// aurion.login("username", "password").await;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn login<U: Into<String>, P: Into<String>>(
        &mut self,
        username: U,
        password: P,
    ) -> Result<()> {
        // Create the payload for the authentication request
        let payload = json!({
            "username": username.into(),
            "password": password.into(),
        });

        // Send the request
        trace!("Beginning login request.");
        let response = self
            .client
            .post(self.pages.login_url())
            .form(&payload)
            .send()
            .await?;
        trace!("Login request sent.");

        // Check if the credentials are correct with the automated redirection
        // by Aurion
        trace!("Checking login response.");
        if !response.headers().contains_key("location") {
            let message = format!("Failed to login: username or password might be wrong.");
            error!("{}", message);
            return Err(Error::msg(message));
        }

        // Send a dummy request to fetch the view state and form id values from
        // Aurion's main logged page
        trace!("Fetching view state and form id values.");
        let dummy_response = self.client.get(self.pages.service_url()).send().await?;
        let dummy_text = dummy_response.text().await?;
        trace!("View state and form id values fetched.");

        // Set the view state and form id values if found
        self.view_state = get_view_state(&dummy_text);
        self.form_id = get_form_id(&dummy_text);

        Ok(())
    }

    /// Get the menu child nodes of the given menu id.
    ///
    /// Aurion's menu is a tree structure. Each node has a unique id and can have
    /// multiple child nodes. This function returns the child nodes of the given
    /// menu id. Also, each node need to be loaded before being able to get its
    /// child nodes.
    pub async fn get_menu_child_nodes<T: Into<String>>(
        &mut self,
        menu_id: T,
    ) -> Result<Vec<Rc<RefCell<Node>>>> {
        let menu_id = menu_id.into();
        let menu_node = self.menu.get_menu_node(menu_id.clone());

        if menu_node.is_none() {
            let message = format!(
                "Failed to get menu child nodes: menu node with id {} not found.",
                menu_id.clone()
            );
            error!("{}", message);
            return Err(Error::msg(message));
        }

        let menu_node = menu_node.unwrap();
        let mut node = menu_node.borrow_mut();

        // Create the payload for the request
        let j_idt = format!("form:j_idt{}", self.form_id.clone().unwrap_or_default());
        let payload = json!({
            "javax.faces.partial.ajax": Bool(true),
            "javax.faces.source": j_idt.clone(),
            "javax.faces.partial.execute": j_idt.clone(),
            "javax.faces.partial.render": "form:sidebar",
            j_idt.clone(): j_idt.clone(),
            "form": "form",
            "form:largeurDivCenter": "",
            "form:sauvegarde": "",
            "form:j_idt805:j_idt808_view": "basicDay",
            "form:j_idt820_focus": "",
            "form:j_idt820_input": "",
            "javax.faces.ViewState": self.view_state.clone().unwrap_or_default(),
            "webscolaapp.Sidebar.ID_SUBMENU": menu_id.clone(),
        });

        // Send the request
        trace!("Beginning menu child nodes request.");
        let response = self
            .client
            .post(self.pages.main_menu_url())
            .form(&payload)
            .send()
            .await?;
        trace!("Menu child nodes request sent.");

        // Get the raw html data from the response
        let text = response.text().await.unwrap();
        let splitter = "<update id=\"form:sidebar\"><![CDATA[";
        let splitted = text.split(splitter).collect::<Vec<&str>>();

        if splitted.len() < 2 {
            let message = format!("Failed to get menu child nodes: invalid response");
            error!("{}", message);
            return Err(Error::msg(message));
        }

        let raw_data = splitted[1].split("]]></update>").collect::<Vec<&str>>()[0];

        // Parse the raw data to dyer::Response to support XPath
        let body = dyer::Body::from(raw_data);
        let mut response = dyer::Response::new(body);

        // Get the child nodes of menu_id's menu
        let result = response.xpath(&format!(
            "//li[contains(@class, \"{}\")]/ul/li",
            menu_id.clone()
        ));

        // Parse the child nodes and add them to the menu tree
        for child_node in &result {
            let is_parent = child_node
                .clone()
                .get_attribute("class")
                .unwrap()
                .contains("ui-menu-parent");

            // Name is contained in the <span> whose class is "ui-menuitem-text"
            let name = child_node
                .clone()
                .findnodes("a/span[@class=\"ui-menuitem-text\"]/text()")
                .unwrap()[0]
                .get_content();
            let name = name.replace("Plannings", "");
            let name = name.replace("Planning", "");
            let name = name.trim().to_string();

            // A node can either be a parent that holds unloaded submenus (children)
            // or a leaf. The parsing of the id for the two cases is
            // unfortunately different.
            let id;
            if is_parent {
                // the id is contained in the class of the <li>
                id = format!(
                    "submenu_{}",
                    child_node
                        .clone()
                        .get_attribute("class")
                        .unwrap()
                        .split_once(" submenu_")
                        .unwrap()
                        .1
                        .split_once(" ")
                        .unwrap()
                        .0
                );
            } else {
                // The id here is contained in the "onclick" attribute of the <a>
                let _id = child_node.clone().findnodes("a").unwrap()[0]
                    .get_attribute("onclick")
                    .unwrap();
                let _id = _id
                    .split_once("form:sidebar_menuid':'")
                    .unwrap()
                    .1
                    .split_once("'")
                    .unwrap()
                    .0;
                id = _id.to_string();
            }

            let parent = Rc::clone(&menu_node);
            let child = Rc::new(RefCell::new(Node::new(id.clone(), name, Some(parent))));

            node.add_child(Rc::clone(&child));
            self.menu.add_node(Rc::clone(&child));
        }

        Ok(node.get_children().to_vec())
    }

    /// Load the menu nodes specified in menu_nodes into the menu tree.
    ///
    /// Aurion's menu is lazy-loaded, meaning that the menu tree is not
    /// loaded all at once. Instead, the menu tree is loaded on demand
    /// when the user clicks on a menu node. This function allows to
    /// load the menu nodes specified in menu_nodes into the menu tree.
    ///
    /// # Arguments
    ///
    /// * `menu_nodes` - The menu nodes to load into the menu tree.
    ///
    /// # Errors
    ///
    /// This function returns an error if the menu nodes could not be
    /// loaded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aurion_rs::Aurion;
    /// # async fn run() -> Result<(), reqwest::Error> {
    /// #     let mut aurion = Aurion::new(
    /// #         275805,
    /// #         "submenu_291906",
    /// #         "1_3",
    /// #         "submenu_299102",
    /// #         "https://web.isen-ouest.fr/webAurion/",
    /// #     );
    /// #     aurion.login("username", "password").await;
    /// aurion.load_menu_nodes(vec!["submenu_1", "submenu_2"]).await;
    /// #     Ok(())
    /// # }
    pub async fn load_menu_nodes<T: Into<String>, V: Into<Vec<T>>>(
        &mut self,
        menu_nodes: V,
    ) -> Result<()> {
        for menu_node in menu_nodes.into() {
            let menu_node = menu_node.into();

            // Check if node is loaded
            if self.menu.is_node_loaded(menu_node.clone()) {
                debug!("Node {} is already loaded", menu_node.clone());
                continue;
            }

            self.get_menu_child_nodes(menu_node.clone()).await?;
        }

        Ok(())
    }

    /// Get the class groups designated by class_group_id.
    /// A class can have multiple groups, for example, a class can have a
    /// group for the morning and a group for the afternoon. This function
    /// returns the groups designated by class_group_id.
    pub async fn get_class_groups<T: Into<String>>(
        &self,
        class_group_id: T,
    ) -> Result<Vec<ClassGroup>> {
        let class_group_id = class_group_id.into();

        // We need to check if the node is loaded. If it is not, we need
        // to load it first because of Aurion's lazy-loading menu tree.

        // Try to get the node from the menu tree
        let node = self.menu.get_menu_node(class_group_id.clone());

        // Check if the node was found
        if node.is_none() {
            let message = format!("Node {} not found", class_group_id.clone());
            error!("{}", message);
            return Err(Error::msg(message));
        }

        // Get the node
        let node = node.unwrap();

        // Check if the node is a leaf node
        if !node.borrow().is_leaf() {
            let message = format!("Node {} is not a leaf node", class_group_id.clone());
            error!("{}", message);
            return Err(Error::msg(message));
        }

        // Check if the node is loaded
        if !node.borrow().is_loaded() {
            let message = format!("Node {} is not loaded", class_group_id.clone());
            error!("{}", message);
            return Err(Error::msg(message));
        }

        // Get the class groups

        // Send the request to load the page for getting the class groups
        let payload = self.default_parameters(class_group_id);
        trace!("Sending first request to get class groups");
        let response = self
            .client
            .post(self.pages.main_menu_url())
            .form(&payload)
            .send()
            .await?;
        trace!("Response received from get class groups request");

        // Parse the response
        let headers = response.headers();

        // Check if the response was successful
        if !headers.contains_key("location") {
            let message = format!("Response to get class groups was not successful");
            error!("{}", message);
            return Err(Error::msg(message));
        }

        // Send the request to get the class groups
        trace!("Sending request to get class groups");
        let response = self
            .client
            .get(self.pages.planning_choice_url())
            .send()
            .await?;
        trace!("Response received from get class groups request");

        // Parse the response data to dyer::Response to support XPath
        let body = dyer::Body::from(response.text().await.unwrap());
        let mut response = dyer::Response::new(body);

        // Get the class groups
        let class_groups = response.xpath("//div[@id=\"form:dataTableFavori\"]//tbody/tr");

        // Check if the class groups were found
        if class_groups.is_empty() {
            let message = format!("Class groups not found");
            error!("{}", message);
            return Err(Error::msg(message));
        }

        // Parse the class groups
        let mut groups = Vec::new();
        for class_group in class_groups {
            let id = class_group
                .get_attribute("data-rk")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let name = class_group
                .get_last_element_child()
                .unwrap()
                .get_last_element_child()
                .unwrap()
                .get_content();
            groups.push(ClassGroup::new(id, name));
        }

        Ok(groups)
    }

    /// Get the lazy-loaded schedule previously initialized by either calling
    /// `get_user_schedule` or `get_group_schedule`.
    /// The schedule is returned as a vector of `Value`s.
    async fn get_schedule(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<Vec<Event>> {
        // Send the request to get the schedule form id
        trace!("Sending request to get schedule form id");
        let response = self.client.get(self.pages.planning_url()).send().await?;
        trace!("Request to get schedule form id sent");

        // Parse the response
        let text = response.text().await.unwrap();
        let schedule_form_id = get_schedule_form_id(text.clone());
        let view_state = get_view_state(text.clone());

        // Check if the form id was found
        if schedule_form_id.is_none() {
            let message = format!("Schedule form id not found");
            return Err(Error::msg(message));
        }

        // Parse the form id
        let schedule_form_id = schedule_form_id.unwrap();

        // Parse start and end dates
        let start = start.unwrap_or_else(|| self.start);
        let end = end.unwrap_or_else(|| self.end);

        // Send the request to get the schedule
        let j_idt = format!("form:j_idt{}", schedule_form_id);
        let payload = json!({
            "javax.faces.partial.ajax": Bool(true),
            "javax.faces.source": j_idt.clone(),
            "javax.faces.partial.execute": j_idt.clone(),
            "javax.faces.partial.render": j_idt.clone(),
            j_idt.clone(): j_idt.clone(),
            format!("{}_start", j_idt.clone()): start.timestamp_millis(),
            format!("{}_end", j_idt.clone()): end.timestamp_millis(),
            "form": "form",
            "javax.faces.ViewState": view_state,
        });

        trace!("Sending request to get schedule");
        let response = self
            .client
            .post(self.pages.planning_url())
            .form(&payload)
            .send()
            .await?;
        trace!("Request to get schedule sent");

        // Parse the response
        let text = response.text().await.unwrap();
        let splitter = "<![CDATA[{\"events\" : ";
        let splitted = text.split_once(splitter);

        // Check if the response was valid
        if splitted.is_none() {
            let message = format!("Response to get schedule was not valid");
            return Err(Error::msg(message));
        }

        let data = splitted.unwrap().1.split_once("}]]></update>").unwrap().0;

        // Parse the schedule
        let mut schedule: Vec<Event> = Vec::new();
        let raw_schedule: Vec<RawEvent> = serde_json::from_str(data)?;
        for raw_event in raw_schedule {
            let event = Event::from_raw_event(raw_event)?;
            schedule.push(event);
        }

        Ok(schedule)
    }

    // pub async fn get_group_schedule<T: Into<String>>(
    //     &mut self,
    //     group_id: T,
    //     start: Option<DateTime<Utc>>,
    //     end: Option<DateTime<Utc>>,
    // )

    /// Get the user's schedule.
    /// The schedule is returned as a vector of `Value`s.
    pub async fn get_user_schedule(
        &mut self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<Vec<Event>> {
        // Load the schooling menu node if it is not loaded
        let schooling_id = self.menu.schooling_id().to_string();
        if !self.menu.is_node_loaded(schooling_id.clone()) {
            debug!("Loading schooling menu node: {}", schooling_id.clone());
            self.load_menu_nodes([schooling_id.clone()]).await.unwrap();
        }

        // Send the request to prepare to get the user's schedule
        trace!("Preparing to get user schedule");
        let payload = self.default_parameters(self.menu.user_planning_id());
        let response = self
            .client
            .post(self.pages.main_menu_url())
            .form(&payload)
            .send()
            .await?;
        trace!("Prepared to get user schedule");

        // Parse the response
        let headers = response.headers().clone();

        // Check if the response is valid
        if !headers.clone().contains_key("location") {
            let message = format!("Response to prepare to get user schedule is not valid");
            error!("{}", message);
            return Err(Error::msg(message));
        }

        // Send the request to get the user's schedule
        let schedule = self.get_schedule(start, end).await.unwrap();

        Ok(schedule)
    }
}
