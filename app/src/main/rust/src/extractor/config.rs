pub struct Config<'a> {
    pub user_agent: &'a str,
    pub proxy: Option<&'a str>,
    pub cookie: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn new(proxy: Option<&'a str>, cookie: Option<&'a str>) -> Self {
        Config {
            proxy,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36",
            cookie,
        }
    }
}
