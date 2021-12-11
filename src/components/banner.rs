use crate::components::common::{Popup, PopupTrigger};
use crate::workers::theme_agent::ThemeAgent;
use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct BannerProps {
    pub bg_color: String,
    pub illustration: String,
    #[prop_or_default]
    pub illustration_style: Option<&'static str>,
}

pub struct Banner {
    style: Style,
    props: BannerProps,
    theme_agent: Box<dyn yew::Bridge<ThemeAgent>>,
}

pub enum BannerMessage {
    ChangeTheme,
}

struct ContactInfo {
    name: &'static str,
    url: &'static str,
}

const CONTACTS: [ContactInfo; 3] = [
    ContactInfo {
        name: "github",
        url: "https://github.com/mistricky",
    },
    ContactInfo {
        name: "twitter",
        url: "https://twitter.com/_mistricky",
    },
    ContactInfo {
        name: "mail",
        url: "Mailto:mist.zzh@gmail.com",
    },
];

impl Component for Banner {
    type Message = BannerMessage;
    type Properties = BannerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Banner",
            r#"
            width: 100%;
            height: 244px;

            .wrapper {
                position: relative;
            }

            .greeter {
                padding-top: 31px;
                display: flex;
            }

            .greeter-text {
                margin-right: 13px;
                font-size: 30px;
                font-weight: 500;
                color: var(--banner-text-color);
                line-height: 42px;
            }

            .desc {
                margin-top: 14px;
                font-size: 14px;
                font-weight: 300;
                color: var(--banner-text-color);
                line-height: 20px;
                width: 729px;
            }

            .contact-title {
                font-size: 18px;
                font-weight: 500;
                color: var(--banner-text-color);
                line-height: 25px;
                padding-top: 14px;
            }

            .contacts {
                margin-top: 14px;
                display: flex;
                align-items: center;
            }

            .contact-icon {
                margin-right: 10px;
            }

            .illustration {
                position: absolute;
                right: 0;
                top: 61px;
                right: -50px;
            }

            .wechat-qr-code {
                width: 200px;
                height: 200px;
            }

            .discord-popup-body {
                width: 106px;
            }

            @media (max-width: 600px) {
                height: auto;
                padding-bottom: 10px;

                .illustration {
                    display: none;
                }
                
                .desc {
                    width: 100%;
                }
            }
        "#,
        )
        .unwrap();
        let theme_agent = ThemeAgent::bridge(link.callback(|_| BannerMessage::ChangeTheme));

        Self {
            style,
            props,
            theme_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BannerMessage::ChangeTheme => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let parsed_illustration_style = match self.props.illustration_style.clone() {
            Some(style) => style,
            None => "",
        };

        html! {
            <div class=self.style.to_string() style=format!("background: {};", self.props.bg_color)>
                <div class="container wrapper">
                    <div class="greeter">
                        <div class="greeter-text">
                            {"I'm Mist"}
                        </div>
                        <img src="/images/avatar.svg" />
                    </div>
                    <div class="desc">
                        {"<Web /> & Block chain & Native developer, Passionate C++, Python, TypeScript, Golang. And I'm one of Rustaceans, like math also Haskell. Remote worker."}
                    </div>
                    <div class="contact-wrapper">
                        <div class="contact-title">
                            {"Contact with MI"}
                        </div>
                        <div class="contacts">
                            {for CONTACTS.iter().map(|contact| {
                                html!{<a href={contact.url}><img class="contact-icon" src={format!("/images/{}_icon.svg", contact.name)} /></a>}
                            })}
                            <Popup
                                offset=(20, 0)
                                has_default_padding=true
                                bind=html!{<img class="contact-icon" src="/images/discord_icon.svg" />}
                                trigger={PopupTrigger::Click}
                            >
                                <div class="discord-popup-body">{"<Mist />#5667"}</div>
                            </Popup>
                            <Popup
                                has_default_padding=false
                                offset=(20, 0)
                                bind=html!{<img class="contact-icon" src="/images/wechat_icon.svg" />}
                                trigger={PopupTrigger::Click}
                            >
                                <img class="wechat-qr-code" src="/images/wechat_qr_code.svg" alt="wechat QR code" />
                            </Popup>
                        </div>
                    </div>
                    <img style=parsed_illustration_style class="illustration" src={self.props.illustration.clone()} />
                </div>
            </div>
        }
    }
}
