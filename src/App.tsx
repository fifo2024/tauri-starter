import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
// import { TrayIcon } from "@tauri-apps/api/tray";
// import { Menu } from "@tauri-apps/api/window";
// import {
//     onOpenUrl,
//     getCurrent as getCurrentDeepLinkUrls,
// } from "@tauri-apps/plugin-deep-link";
import { DockMenu, AnimatedTabs } from "@/components";
import { AlbumIcon, HomeIcon, MonitorIcon } from "lucide-react";
import "./App.css";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

const fetchVersion = async () => {
    const update = await check();

    if (update) {
        console.log(
            `found update ${update.version} from ${update.date} with notes ${update.body}`
        );
        let downloaded = 0;
        let contentLength: any = 0;
        // 也可以分开调用 update.download() 和 update.install()
        await update.downloadAndInstall((event) => {
            switch (event.event) {
                case "Started":
                    contentLength = event.data.contentLength;
                    console.log(
                        `started downloading ${event.data.contentLength} bytes`
                    );
                    break;
                case "Progress":
                    downloaded += event.data.chunkLength;
                    console.log(
                        `downloaded ${downloaded} from ${contentLength}`
                    );
                    break;
                case "Finished":
                    console.log("download finished");
                    break;
            }
        });

        console.log("update installed");
        // 此处 relaunch 前最好询问用户
        await relaunch();
    }
};

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, setName] = useState("");
    const items = [
        { id: "first-id", icon: <AlbumIcon size={32} /> },
        { id: "second-id", icon: <HomeIcon size={32} /> },
        { id: "third-id", icon: <MonitorIcon size={32} /> },
    ];
    const tabs = [
        {
            title: "Product",
        },
        {
            title: "Services",
        },
        {
            title: "About",
        },
    ];
    const [trayIconValue, setTrayIconValue] = useState("dark");

    // const setupMenu = async () => {
    //     const menu = await Menu.new({
    //         items: [
    //             {
    //                 id: "quit",
    //                 text: "Quit",
    //             },
    //         ],
    //     });

    //     const options = {
    //         menu,
    //         menuOnLeftClick: true,
    //     };

    //     const tray = TrayIcon.new(options);
    // };

    // Utility function to implement a sleep function in TypeScript
    function sleep(seconds: number): Promise<void> {
        return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
    }

    // Setup function
    async function setup() {
        // Fake perform some really heavy setup task
        console.log("Performing really heavy frontend setup task...");
        await sleep(3);
        console.log("Frontend setup task complete!");
        // Set the frontend task as being completed
        invoke("set_complete", { task: "frontend" });
    }

    useEffect(() => {
        fetchVersion();
        // setupMenu();
        setup();
    }, []);

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        console.log(greetMsg);
        setGreetMsg(await invoke("greet", { name }));
    }

    const onSwitchTrayIcon = async () => {
        try {
            // invoke  switch_tray_icon 事件名 isDarkMode 参数名
            await invoke("switch_tray_icon", {
                isDarkMode: trayIconValue === "dark",
            });
        } catch (err) {
            console.error("Failed to switch tray icon:", err);
        }
        setTrayIconValue(trayIconValue === "dark" ? "light" : "dark");
    };

    return (
        <div className="relative flex h-[100vh] w-full flex-col items-center justify-center overflow-hidden rounded-lg bg-background px-20 py-16">
            <DockMenu items={items} />
            <AnimatedTabs tabs={tabs} tabClassName="" />
            <div>
                <button onClick={onSwitchTrayIcon}>切换图标0.1.6</button>
            </div>
            <div>
                <input
                    type="text"
                    style={{ background: "#eee" }}
                    onChange={(e) => setName(e.target.value)}
                />
                <button onClick={greet}>submit</button>
            </div>
            <div>{greetMsg}</div>
        </div>
    );
}

export default App;
