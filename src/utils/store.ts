import { load } from "@tauri-apps/plugin-store";
import type { Server } from "../types";

const STORE_KEY = "server_list";
const STORE_PATH = "servers.json";

export const loadServers = async (): Promise<Server[]> => {
	try {
		const store = await load(STORE_PATH);
		const servers = await store.get<Server[]>(STORE_KEY);
		return servers || [];
	} catch (error) {
		console.error("Failed to load servers:", error);
		return [];
	}
};

export const saveServers = async (servers: Server[]): Promise<void> => {
	try {
		const store = await load(STORE_PATH);
		await store.set(STORE_KEY, servers);
	} catch (error) {
		console.error("Failed to save servers:", error);
	}
};
