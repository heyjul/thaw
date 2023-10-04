use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Button"</h1>
            <Demo>
                <Space>
                    <Button type_=ButtonType::PRIMARY>
                        "PRIMARY"
                    </Button>
                    <Button type_=ButtonType::SOLID>
                        "SOLID"
                    </Button>
                    <Button type_=ButtonType::TEXT>
                        "TEXT"
                    </Button>
                    <Button type_=ButtonType::LINK>
                        "LINK"
                    </Button>
                </Space>
                <DemoCode slot html=highlight_str!(r#"
                    <Button type_=ButtonType::PRIMARY>
                        "PRIMARY"
                    </Button>
                    <Button type_=ButtonType::SOLID>
                        "SOLID"
                    </Button>
                    <Button type_=ButtonType::TEXT>
                        "TEXT"
                    </Button>
                    <Button type_=ButtonType::LINK>
                        "LINK"
                    </Button>
                "#, "rust")>
                    ""
                </DemoCode>
            </Demo>
            <h3>"color"</h3>
            <Demo>
                <Space>
                    <Button color=ButtonColor::PRIMARY>
                        "PRIMARY Color"
                    </Button>
                    <Button color=ButtonColor::SUCCESS>
                        "SUCCESS Color"
                    </Button>
                    <Button color=ButtonColor::WARNING>
                        "WARNING Color"
                    </Button>
                    <Button color=ButtonColor::ERROR>
                        "ERROR Color"
                    </Button>
                </Space>
                <DemoCode slot html=highlight_str!(r#"
                    <Button color=ButtonColor::PRIMARY>
                        "PRIMARY Color"
                    </Button>
                    <Button color=ButtonColor::SUCCESS>
                        "SUCCESS Color"
                    </Button>
                    <Button color=ButtonColor::WARNING>
                        "WARNING Color"
                    </Button>
                    <Button color=ButtonColor::ERROR>
                        "ERROR Color"
                    </Button>
                "#, "rust")>
                    ""
                </DemoCode>
            </Demo>
            <h3>"icon"</h3>
            <Demo>
                <Space>
                    <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined>
                        "ERROR Color Icon"
                    </Button>
                    <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined round=true>
                    </Button>
                </Space>
                <DemoCode slot html=highlight_str!(r#"
                    <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined>
                        "ERROR Color Icon"
                    </Button>
                    <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined round=true>
                    </Button>
                "#, "rust")>
                    ""
                </DemoCode>
            </Demo>
            <h3>"style"</h3>
            <Demo>
                <Space>
                    <Button style="background: blue;">"style blue"</Button>
                    <Button style="width: 40px; height: 20px">"size"</Button>
                </Space>
                <DemoCode slot html=highlight_str!(r#"
                    <Button style="background: blue;">"style blue"</Button>
                    <Button style="width: 40px; height: 20px">"size"</Button>
                "#, "rust")>
                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}
