macro_rules! colour {
    ( $default: ident, $light: ident, $default_bg:ident, $light_bg:ident, $num:expr ) => {
        fn $default(&self) -> String {
            format!("\u{001b}[{}m", 30 + $num) + &self.to_string()
        }

        fn $light(&self) -> String {
            format!("\u{001b}[{}m", 90 + $num) + &self.to_string()
        }

        fn $default_bg(&self) -> String {
            format!("\u{001b}[{}m", 40 + $num) + &self.to_string()
        }

        fn $light_bg(&self) -> String {
            format!("\u{001b}[{}m", 100 + $num) + &self.to_string()
        }
    };
}

macro_rules! colour_define {
    ( $default: ident, $light: ident, $default_bg:ident, $light_bg:ident) => {
        fn $default(&self) -> String;
        fn $light(&self) -> String;
        fn $default_bg(&self) -> String;
        fn $light_bg(&self) -> String;
    };
}

pub trait Style {
    colour_define!(black, light_black, bg_black, bg_light_black);
    colour_define!(red, light_red, bg_red, bg_light_red);
    colour_define!(green, light_green, bg_green, bg_light_green);
    colour_define!(yellow, light_yellow, bg_yellow, bg_light_yellow);
    colour_define!(blue, light_blue, bg_blue, bg_light_blue);
    colour_define!(magenta, light_magenta, bg_magenta, bg_light_magenta);
    colour_define!(cyan, light_cyan, bg_cyan, bg_light_cyan);
    colour_define!(white, light_white, bg_white, bg_light_white);

    fn reset(&self) -> String;
    fn conclude(&self) -> String;

    fn bold(&self) -> String;
    fn faint(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn slow_blink(&self) -> String;
    fn rapid_blink(&self) -> String;
    fn reverse(&self) -> String;
    fn strike(&self) -> String;
}

impl Style for &str {
    colour!(black, light_black, bg_black, bg_light_black, 0);
    colour!(red, light_red, bg_red, bg_light_red, 1);
    colour!(green, light_green, bg_green, bg_light_green, 2);
    colour!(yellow, light_yellow, bg_yellow, bg_light_yellow, 3);
    colour!(blue, light_blue, bg_blue, bg_light_blue, 4);
    colour!(magenta, light_magenta, bg_magenta, bg_light_magenta, 5);
    colour!(cyan, light_cyan, bg_cyan, bg_light_cyan, 6);
    colour!(white, light_white, bg_white, bg_light_white, 7);

    fn reset(&self) -> String {
        "\u{001b}[0m".to_owned() + &self.to_string()
    }
    fn conclude(&self) -> String {
        self.to_string() + &"\u{001b}[0m".to_owned()
    }

    fn bold(&self) -> String {
        "\u{001b}[1m".to_owned() + &self.to_string()
    }
    fn faint(&self) -> String {
        "\u{001b}[2m".to_owned() + &self.to_string()
    }
    fn italic(&self) -> String {
        "\u{001b}[3m".to_owned() + &self.to_string()
    }
    fn underline(&self) -> String {
        "\u{001b}[4m".to_owned() + &self.to_string()
    }
    fn slow_blink(&self) -> String {
        "\u{001b}[5m".to_owned() + &self.to_string()
    }
    fn rapid_blink(&self) -> String {
        "\u{001b}[6m".to_owned() + &self.to_string()
    }
    fn reverse(&self) -> String {
        "\u{001b}[7m".to_owned() + &self.to_string()
    }
    fn strike(&self) -> String {
        "\u{001b}[9m".to_owned() + &self.to_string()
    }
}

impl Style for String {
    colour!(black, light_black, bg_black, bg_light_black, 0);
    colour!(red, light_red, bg_red, bg_light_red, 1);
    colour!(green, light_green, bg_green, bg_light_green, 2);
    colour!(yellow, light_yellow, bg_yellow, bg_light_yellow, 3);
    colour!(blue, light_blue, bg_blue, bg_light_blue, 4);
    colour!(magenta, light_magenta, bg_magenta, bg_light_magenta, 5);
    colour!(cyan, light_cyan, bg_cyan, bg_light_cyan, 6);
    colour!(white, light_white, bg_white, bg_light_white, 7);

    fn reset(&self) -> String {
        "\u{001b}[0m".to_owned() + &self.to_string()
    }
    fn conclude(&self) -> String {
        self.to_string() + &"\u{001b}[0m".to_owned()
    }

    fn bold(&self) -> String {
        "\u{001b}[1m".to_owned() + &self.to_string()
    }
    fn faint(&self) -> String {
        "\u{001b}[2m".to_owned() + &self.to_string()
    }
    fn italic(&self) -> String {
        "\u{001b}[3m".to_owned() + &self.to_string()
    }
    fn underline(&self) -> String {
        "\u{001b}[4m".to_owned() + &self.to_string()
    }
    fn slow_blink(&self) -> String {
        "\u{001b}[5m".to_owned() + &self.to_string()
    }
    fn rapid_blink(&self) -> String {
        "\u{001b}[6m".to_owned() + &self.to_string()
    }
    fn reverse(&self) -> String {
        "\u{001b}[7m".to_owned() + &self.to_string()
    }
    fn strike(&self) -> String {
        "\u{001b}[9m".to_owned() + &self.to_string()
    }
}
