use std::f32;
use std::boxed::FnBox;
use super::resolve::Env;
use super::context::MessageContext;

#[derive(Clone, Debug, PartialEq)]
pub enum FluentValue {
    String(String),
    Number(f32),
}

// XXX Replace this with a proper plural rule
fn get_plural_rule<'a>(ctx: &MessageContext) -> Box<FnBox(f32) -> &'static str> {
    let locale = &ctx.locales[0];

    if locale == "pl" {
        return Box::new(|num| if num == 1.0 { "one" } else { "other" });
    }
    return Box::new(|num| if num == 1.0 { "one" } else { "other" });
}

impl FluentValue {
    pub fn format(&self) -> String {
        match *self {
            FluentValue::String(ref s) => s.clone(),
            FluentValue::Number(ref n) => format!("{}", n),
        }
    }

    pub fn matches(&self, _env: &Env, other: &FluentValue) -> bool {
        match (self, other) {
            (&FluentValue::String(ref a), &FluentValue::String(ref b)) => a == b,
            (&FluentValue::Number(ref a), &FluentValue::Number(ref b)) => {
                (a - b).abs() < f32::EPSILON
            }
            (&FluentValue::String(ref a), &FluentValue::Number(ref b)) => {
                let pr = get_plural_rule(_env.ctx);
                let category = pr(*b);
                a == category
            }
            (&FluentValue::Number(..), &FluentValue::String(..)) => false,
        }
    }
}

impl From<String> for FluentValue {
    fn from(s: String) -> Self {
        FluentValue::String(s)
    }
}

impl<'a> From<&'a str> for FluentValue {
    fn from(s: &'a str) -> Self {
        FluentValue::String(String::from(s))
    }
}

impl From<f32> for FluentValue {
    fn from(n: f32) -> Self {
        FluentValue::Number(n)
    }
}

impl From<i8> for FluentValue {
    fn from(n: i8) -> Self {
        FluentValue::Number(f32::from(n))
    }
}
