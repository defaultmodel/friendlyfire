import {
	createContext,
	useState,
	useContext,
	useEffect,
	useMemo,
	type ReactNode,
} from "react";
import type { Server } from "../../types";
import { loadServers, saveServers } from "../../utils/store";

interface ServerContextType {
	servers: Server[];
	addServer: (server: Server) => void;
	removeServer: (index: number) => void;
	updateServer: (index: number, updatedServer: Server) => void;
	clearServers: () => void;
	loading: boolean; // Add loading state
}

const ServerContext = createContext<ServerContextType | undefined>(undefined);

export const ServerProvider = ({ children }: { children: ReactNode }) => {
	const [servers, setServers] = useState<Server[]>([]);
	const [loading, setLoading] = useState(true); // Track loading state

	// Load servers on mount
	useEffect(() => {
		const fetchServers = async () => {
			try {
				const loadedServers = await loadServers();
				setServers(loadedServers);
			} catch (error) {
				console.error("Failed to load servers:", error);
			} finally {
				setLoading(false); // Mark loading as complete
			}
		};
		fetchServers();
	}, []);

	// Save servers when they change (only if not empty)
	useEffect(() => {
		if (servers.length > 0 && !loading) {
			saveServers(servers);
		}
	}, [servers, loading]);

	const addServer = (newServer: Server) => {
		setServers((prevServers) => [...prevServers, newServer]);
	};

	const removeServer = (index: number) => {
		if (index < 0 || index >= servers.length) {
			console.error("Invalid index for removeServer");
			return;
		}
		setServers((prevServers) => prevServers.filter((_, i) => i !== index));
	};

	const updateServer = (index: number, updatedServer: Server) => {
		if (index < 0 || index >= servers.length) {
			console.error("Invalid index for updateServer");
			return;
		}
		setServers((prevServers) =>
			prevServers.map((server, i) => (i === index ? updatedServer : server)),
		);
	};

	const clearServers = () => {
		setServers([]);
	};

	// Memoize context value to avoid unnecessary re-renders
	// biome-ignore lint/correctness/useExhaustiveDependencies: <adding functions that modify servers is unneeded as it is already a dependency>
	const contextValue = useMemo(
		() => ({
			servers,
			addServer,
			removeServer,
			updateServer,
			clearServers,
			loading,
		}),
		[servers, loading],
	);

	return (
		<ServerContext.Provider value={contextValue}>
			{children}
		</ServerContext.Provider>
	);
};

export const useServerContext = () => {
	const context = useContext(ServerContext);
	if (!context) {
		throw new Error("useServerContext must be used within a ServerProvider");
	}
	return context;
};
