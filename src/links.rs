use ini::Ini;
use std::cmp::Eq;
use std::cmp::Ordering;
use std::collections::HashMap;
use url::Url;

pub struct Link {
    name: String,
    australia: Option<String>,
    brazil: Option<String>,
    canada: Option<String>,
    china: Option<String>,
    france: Option<String>,
    germany: Option<String>,
    india: Option<String>,
    italy: Option<String>,
    japan: Option<String>,
    mexico: Option<String>,
    spain: Option<String>,
    uk: Option<String>,
    us: Option<String>,
}

impl Eq for Link {}
impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Ord for Link {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Link) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn ini_to_links(ini: &Ini) -> Result<Vec<Link>, String> {
    fn first_invalid_url_of_props(props: &HashMap<String, String>) -> Option<String> {
        let maybe_invalid_url_and_error = props.values()
            .map(|url| (url, Url::parse(url)))
            .filter(|(_, r)| r.is_err())
            .next();

        return match maybe_invalid_url_and_error {
            None => None,
            Some(res) => {
                match res.1 {
                    Ok(_) => None,
                    Err(_) => Some(res.0.to_owned())
                }
            }
        }
    }

    let mut links: Vec<Link> = Vec::new();

    for (sec, props) in ini {
        match sec {
            None => return Err(format!("A section has no name.")),
            Some(section_name) => {
                fn get_key_of_props(props: &HashMap<String, String>, key: &str) -> Option<String> {
                    return props.get(key).map(|s| s.to_string())
                }

                if props.is_empty() {
                    return Err(format!("Section {} has no links.", section_name))
                }

                let maybe_invalid_url = first_invalid_url_of_props(props);
                match maybe_invalid_url {
                    None => {}
                    Some(url) => {
                        return Err(format!("Invalid link: \"{}\".", url))
                    }
                }

                let link = Link {
                    name: section_name.to_string(),
                    australia: get_key_of_props(props, "au"),
                    brazil: get_key_of_props(props, "br"),
                    canada: get_key_of_props(props, "ca"),
                    china: get_key_of_props(props, "cn"),
                    france: get_key_of_props(props, "fr"),
                    germany: get_key_of_props(props, "de"),
                    india: get_key_of_props(props, "in"),
                    italy: get_key_of_props(props, "it"),
                    japan: get_key_of_props(props, "jp"),
                    mexico: get_key_of_props(props, "mx"),
                    spain: get_key_of_props(props, "es"),
                    uk: get_key_of_props(props, "uk"),
                    us: get_key_of_props(props, "us"),
                };

                links.push(link);
            },
        }
    }

    if ini.sections().len() == 0 {
        return Err("No link found.".to_owned());
    }

    links.sort();
    return Ok(links)
}

#[cfg(test)]
mod tests {
    use crate::links::ini_to_links;

    #[test]
    fn test_ini_to_links_success_case() {
        let mut ini = ini::Ini::new();
        ini.with_section(Some("product1".to_owned()))
            .set("au", "https://amazon.au/whatever")
            .set("br", "https://amazon.com.br/whatever")
            .set("ca", "https://amazon.ca/whatever")
            .set("cn", "https://amazon.cn/whatever")
            .set("fr", "https://amazon.fr/whatever")
            .set("de", "https://amazon.de/whatever")
            .set("in", "https://amazon.in/whatever")
            .set("it", "https://amazon.it/whatever")
            .set("jp", "https://amazon.jp/whatever")
            .set("mx", "https://amazon.mx/whatever")
            .set("es", "https://amazon.es/whatever")
            .set("uk", "https://amazon.uk/whatever")
            .set("us", "https://amazon.us/whatever");
        ini.with_section(Some("product2".to_owned()))
            .set("au", "https://amazon.au/whatever");

        let links = ini_to_links(&ini)
            .expect("This shouldn't happen.");

        assert_eq!(links[0].name, "product1");
        assert_eq!(links[0].australia, Some("https://amazon.au/whatever".to_owned()));
        assert_eq!(links[0].brazil, Some("https://amazon.com.br/whatever".to_owned()));
        assert_eq!(links[0].canada, Some("https://amazon.ca/whatever".to_owned()));
        assert_eq!(links[0].china, Some("https://amazon.cn/whatever".to_owned()));
        assert_eq!(links[0].france, Some("https://amazon.fr/whatever".to_owned()));
        assert_eq!(links[0].germany, Some("https://amazon.de/whatever".to_owned()));
        assert_eq!(links[0].india, Some("https://amazon.in/whatever".to_owned()));
        assert_eq!(links[0].italy, Some("https://amazon.it/whatever".to_owned()));
        assert_eq!(links[0].japan, Some("https://amazon.jp/whatever".to_owned()));
        assert_eq!(links[0].mexico, Some("https://amazon.mx/whatever".to_owned()));
        assert_eq!(links[0].spain, Some("https://amazon.es/whatever".to_owned()));
        assert_eq!(links[0].uk, Some("https://amazon.uk/whatever".to_owned()));
        assert_eq!(links[0].us, Some("https://amazon.us/whatever".to_owned()));

        assert_eq!(links[1].name, "product2");
        assert_eq!(links[1].australia, Some("https://amazon.au/whatever".to_owned()));
        assert_eq!(links[1].brazil, None);
        assert_eq!(links[1].canada, None);
        assert_eq!(links[1].china, None);
        assert_eq!(links[1].france, None);
        assert_eq!(links[1].germany, None);
        assert_eq!(links[1].india, None);
        assert_eq!(links[1].italy, None);
        assert_eq!(links[1].japan, None);
        assert_eq!(links[1].mexico, None);
        assert_eq!(links[1].spain, None);
        assert_eq!(links[1].uk, None);
        assert_eq!(links[1].us, None);
    }

    #[test]
    fn test_ini_to_links_section_with_no_links() {
        let mut ini = ini::Ini::new();
        ini.with_section(Some("product1".to_owned()));

        let links = ini_to_links(&ini);

        assert_eq!(links.is_err(), true);
        assert_eq!(links.err(), Some("No link found.".to_owned()));
    }

    #[test]
    fn test_ini_to_links3() {
        let mut ini = ini::Ini::new();
        ini.with_section(Some("product1".to_owned()));
        ini.with_section(Some("product2".to_owned()))
            .set("test", "https://test");

        let links = ini_to_links(&ini);

        assert_eq!(links.is_err(), false);
    }

    #[test]
    fn test_ini_to_links_section_with_no_name() {
        let mut ini = ini::Ini::new();
        let section_name: Option<String> = None; ini.with_section(section_name)
            .set("test", "https://test");

        let links = ini_to_links(&ini);

        assert_eq!(links.is_err(), true);
    }

    #[test]
    fn test_ini_to_links4_invalid_url() {
        let mut ini = ini::Ini::new();
        let invalid_link = "lkjsadklasj";
        ini.with_section(Some("product1".to_owned()))
            .set("fr", invalid_link);

        let links = ini_to_links(&ini);

        assert_eq!(links.is_err(), true);
        assert_eq!(links.err(), Some(format!("Invalid link: \"{}\".", invalid_link.to_owned())));
    }
}
