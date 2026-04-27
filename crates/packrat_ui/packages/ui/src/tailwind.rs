use dioxus::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub enum Theme {
    #[default]
    Catppuccin,
    TokyoNight,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Catppuccin => "",
            Self::TokyoNight => "theme-tokyo-night",
        }
    }
}

#[component]
pub fn TailwindConfig(children: Element) -> Element {
    let theme = use_context_provider(|| Signal::new(Theme::default()));
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css"),
        }
        div {
            class: "{theme().as_str()} min-h-screen bg-app-base text-app-text transition-colors duration-500 ease-in-out",
            {children}
        }
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme = use_context::<Signal<Theme>>();
    let next = match theme() {
        Theme::Catppuccin => Theme::TokyoNight,
        Theme::TokyoNight => Theme::Catppuccin,
    };

    rsx! {
        button {
            class: "px-3 py-1 rounded-full bg-app-surface border border-app-accent text-app-accent hover:bg-app-accent hover:text-app-base transition-all",
            onclick: move |_| {
                theme.set(next);
            },
            "Switch to {next:?}"
        }
    }
}
