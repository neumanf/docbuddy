import { LogicalSize, appWindow } from "@tauri-apps/api/window";

export async function setWindowsSizeAsCompact() {
    await appWindow.setSize(new LogicalSize(450, 90));
}

export async function setWindowsSizeAsExpanded() {
    await appWindow.setSize(new LogicalSize(450, 560));
}
