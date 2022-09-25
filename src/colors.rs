macro_rules! make_colors {
    ($cat: ident; $($name: ident = $open: expr, $close: expr);+ $(;)?) => {
        pub enum $cat {
            $($name),+
        }

        impl $cat {
            pub fn get_parts(&self) -> (u8, u8) {
                match self {
                    $(
                        $cat :: $name => ($open, $close)
                    ),+
                }
            }

            pub fn get_open(&self) -> String {
                let (open, _) = self.get_parts();

                format!("\x1b[{}m", open)
            }

            pub fn get_close(&self) -> String {
                let (_, close) = self.get_parts();

                format!("\x1b[{}m", close)
            }

            pub fn apply_color<S: Into<String>>(&self, text: S) -> String {
                format!("{}{}{}", self.get_open(), text.into(), self.get_close())
            }

            /// Apply color - shothand
            pub fn a<S: Into<String>>(&self, text: S) -> String {
                format!("{}{}{}", self.get_open(), text.into(), self.get_close())
            }

        }

        impl ::std::fmt::Display for $cat {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.get_open().fmt(f)
            }
        }
    }
}

make_colors! {
    Color;
    Black = 30, 39;
    Red = 31, 39;
    Green = 32, 39;
    Yellow = 33, 39;
    Blue = 34, 39;
    Magenta = 35, 39;
    Cyan = 36, 39;
    White = 37, 39;

    BlackBright = 90, 39;
    RedBright = 91, 39;
    GreenBright = 92, 39;
    YellowBright = 93, 39;
    BlueBright = 94, 39;
    MagentaBright = 95, 39;
    CyanBright = 96, 39;
    WhiteBright = 97, 39;
}

make_colors! {
    Modifier;
    Reset = 0, 0;
    Bold = 1, 22;
    Dim = 2, 22;
    Italic = 3, 23;
    Underline = 4, 24;
    Overline = 53, 55;
    Inverse = 7, 27;
    Hidden = 8, 28;
    Striketrough = 9, 29;
}
