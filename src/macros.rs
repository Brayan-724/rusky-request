#[macro_export]
macro_rules! match_prompt_type_extra {
    (Bool) => {
        Option::Some(String::from("y/n"))
    };
    ($($_:ident)?) => {
        Option::None
    };
}

#[macro_export]
macro_rules! match_prompt_type_struct {
    ([Int] $($tail:tt)+) => {
        $crate::NumberPrompt $($tail)+
    };
    ([UInt] $($tail:tt)+) => {
        $crate::NumberPrompt $($tail)+
    };
    ([Float] $($tail:tt)+) => {
        $crate::NumberPrompt $($tail)+
    };
    ([UFloat] $($tail:tt)+) => {
        $crate::NumberPrompt $($tail)+
    };
    ([$($_:ident)?] $($tail:tt)+) => {
        $crate::Prompt $($tail)+
    };
}

#[macro_export]
macro_rules! create_prompt {
    ($prefix:tt $text:expr; $(($($extra:expr)+))? $([$($default:expr)+])? $({ $THEME: expr })? $($type:ident)?) => {
        $crate::match_prompt_type_struct!([$($type)?] {
            prefix: Into::into(stringify!($prefix)),
            text: Into::into($text),
            default: $crate::handle_optional!(if ($($($default)+)?) {
                Option::Some(Into::into($($($default)+)?))
            } else {
                Option::None
            }),
            extra: $crate::handle_optional!(if ($($($extra)+)?) {
                Option::Some(Into::into($($($extra)+)?))
            } else {
                $crate::match_prompt_type_extra!($($type)?)
            }),
            line: Option::None,
            prompt_type: $crate::handle_optional!(if ($($type)?) {
                $crate::PromptType:: $($type)?
            } else {
                $crate::PromptType::String
            }),
            theme: $crate::handle_optional!(if ($($THEME)?) {
                $($THEME)?
            } else {
                &$crate::DefaultTheme
            })
        })
    }
}

#[macro_export]
macro_rules! prompt_it {
    ($prompt:ident $(( $go_back:expr ))? ; $events:ident $stdout:ident) => (
        $prompt.prompt_handled(
            &mut $events,
            &mut $stdout,
            $crate::handle_optional!(if ($($go_back)?) {
                Option::Some(Into::into($($go_back)?))
            } else {
                Option::None
            })
        )
    )
}

#[macro_export]
macro_rules! handle_optional {
    (if ( $($_:tt)+ ) { $($T:tt)* } else { $($F:tt)* }) => ($($T)*);
    (if ( ) { $($T:tt)* } else { $($F:tt)+ }) => ($($F)*);
}

#[macro_export]
macro_rules! io_handl {
    ($expr:expr) => {
        match $expr {
            Ok(ok) => ok,
            Err(err) => return Err($crate::prompts::PromptError::IO(err)),
        }
    };
}

pub use create_prompt;
pub use handle_optional;
pub use io_handl;
pub use macro_export;
pub use match_prompt_type_extra;
pub use match_prompt_type_struct;
pub use prompt_it;
