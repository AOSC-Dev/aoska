

use serde::Serialize;
use tauri::command;

#[derive(Serialize)]
struct AppRecommand {
    name: String,
    intro: String,
}

#[derive(Serialize)]
struct AppInfo {
    name: String,
    intro: String,
    version: String,
    size: String,
}

#[command]
// 首页,获得推荐列表
// 无参数，返回值为推荐列表
fn fetch_recommand() -> Vec<AppRecommand> {
    vec![
        AppRecommand {
            name: "WPS 办公套件".to_string(),
            intro: "文档、幻灯片、表格三位一体".to_string(),
        },
        AppRecommand {
            name: "Google Chrome".to_string(),
            intro: "全世界用户最多的浏览器".to_string(),
        },
        AppRecommand {
            name: "微信".to_string(),
            intro: "聊天、小程序尽在指尖".to_string(),
        },
        AppRecommand {
            name: "Telegram Desktop".to_string(),
            intro: "简便快捷的即时通讯应用".to_string(),
        },
        AppRecommand {
            name: "QQ".to_string(),
            intro: "亲朋好友，常联系".to_string(),
        },
        AppRecommand {
            name: "Visual Studio Code".to_string(),
            intro: "程序编码、设计、调试集大成者".to_string(),
        },
        AppRecommand {
            name: "手机投屏".to_string(),
            intro: "手机应用，跃然桌上".to_string(),
        },
        AppRecommand {
            name: "火焰截图".to_string(),
            intro: "功能强大的截图工具".to_string(),
        },
    ]
}

#[command]
// 分类页面，根据类别返回程序信息
// 参数为程序类别，返回值为程序列表
fn fetch_by_category(category: String) -> Vec<AppInfo> {
    vec![
        AppInfo {
            name: "应用程序1".to_string(),
            intro: "这是应用程序1".to_string(),
            version: "11.4".to_string(),
            size: "22.3".to_string(),
        },
        AppInfo {
            name: "应用程序2".to_string(),
            intro: "这是应用程序2".to_string(),
            version: "11.4".to_string(),
            size: "22.3".to_string(),
        },
        AppInfo {
            name: "应用程序3".to_string(),
            intro: "这是应用程序3".to_string(),
            version: "11.4".to_string(),
            size: "22.3".to_string(),
        },
        AppInfo {
            name: "应用程序4".to_string(),
            intro: "这是应用程序4".to_string(),
            version: "11.4".to_string(),
            size: "22.3".to_string(),
        },
        AppInfo {
            name: "应用程序5".to_string(),
            intro: "这是应用程序5".to_string(),
            version: "11.4".to_string(),
            size: "22.3".to_string(),
        },
        AppInfo {
            name: "应用程序6".to_string(),
            intro: "这是应用程序6".to_string(),
            version: "11.4".to_string(),
            size: "22.3".to_string(),
        },
        AppInfo {
            name: "应用程序7".to_string(),
            intro: "这是应用程序7".to_string(),
            version: "11.4".to_string(),
            size: "22.3".to_string(),
        },
    ]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_recommand, fetch_by_category])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
