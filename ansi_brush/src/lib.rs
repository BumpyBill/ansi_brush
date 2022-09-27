pub trait Style {
    fn black(&self) -> String;
    fn light_black(&self) -> String;
    fn red(&self) -> String;
    fn light_red(&self) -> String;
    fn green(&self) -> String;
    fn light_green(&self) -> String;
    fn yellow(&self) -> String;
    fn light_yellow(&self) -> String;
    fn blue(&self) -> String;
    fn light_blue(&self) -> String;
    fn magenta(&self) -> String;
    fn light_magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn light_cyan(&self) -> String;
    fn white(&self) -> String;
    fn light_white(&self) -> String;
    fn reset(&self) -> String;
}

impl Style for &str {
    fn black(&self) -> String {
        "\u{001b}[30m".to_owned() + &self.to_string()
    }

    fn light_black(&self) -> String {
        "\u{001b}[90m".to_owned() + &self.to_string()
    }

    fn red(&self) -> String {
        "\u{001b}[31m".to_owned() + &self.to_string()
    }

    fn light_red(&self) -> String {
        "\u{001b}[91m".to_owned() + &self.to_string()
    }

    fn green(&self) -> String {
        "\u{001b}[32m".to_owned() + &self.to_string()
    }

    fn light_green(&self) -> String {
        "\u{001b}[92m".to_owned() + &self.to_string()
    }

    fn yellow(&self) -> String {
        "\u{001b}[33m".to_owned() + &self.to_string()
    }

    fn light_yellow(&self) -> String {
        "\u{001b}[93m".to_owned() + &self.to_string()
    }

    fn blue(&self) -> String {
        "\u{001b}[34m".to_owned() + &self.to_string()
    }

    fn light_blue(&self) -> String {
        "\u{001b}[94m".to_owned() + &self.to_string()
    }

    fn magenta(&self) -> String {
        "\u{001b}[35m".to_owned() + &self.to_string()
    }

    fn light_magenta(&self) -> String {
        "\u{001b}[95m".to_owned() + &self.to_string()
    }

    fn cyan(&self) -> String {
        "\u{001b}[36m".to_owned() + &self.to_string()
    }

    fn light_cyan(&self) -> String {
        "\u{001b}[96m".to_owned() + &self.to_string()
    }

    fn white(&self) -> String {
        "\u{001b}[37m".to_owned() + &self.to_string()
    }

    fn light_white(&self) -> String {
        "\u{001b}[97m".to_owned() + &self.to_string()
    }

    fn reset(&self) -> String {
        "\u{001b}[0m".to_owned() + &self.to_string()
    }
}
