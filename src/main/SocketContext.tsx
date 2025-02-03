// SocketContext.tsx
import type React from "react";
import {
	createContext,
	useState,
	useContext,
	type ReactNode,
	useEffect,
} from "react";
import { io, type Socket } from "socket.io-client";
import { version } from "../../package.json";

const CLIENT_VERSION = version;

type SocketContextType = {
	socket: Socket | null;
	isConnected: boolean;
	socketUrl: string | null;
	connectSocket: (serverUrl: string, apiKey: string, username: string) => void;
	disconnectSocket: () => void;
};

const SocketContext = createContext<SocketContextType | null>(null);

export const useSocket = () => {
	const context = useContext(SocketContext);
	if (!context) {
		throw new Error("useSocket must be used within a SocketProvider");
	}
	return context;
};

type SocketProviderProps = {
	children: ReactNode;
};

export const SocketProvider: React.FC<SocketProviderProps> = ({ children }) => {
	const [socket, setSocket] = useState<Socket | null>(null);
	const [isConnected, setIsConnected] = useState(false); // Track connection state
	const [socketUrl, setSocketUrl] = useState<string | null>(null); // Track the socket URL

	const connectSocket = (
		serverUrl: string,
		apiKey: string,
		username: string,
	) => {
		const newSocket = io(serverUrl);
		newSocket.auth = { key: apiKey, username, version: CLIENT_VERSION };

		// Set up event listeners for connection state changes
		newSocket.on("connect", () => {
			setIsConnected(true);
			newSocket.emit("ready"); // Tell the server we are ready to receive messages
			setSocketUrl(serverUrl); // Update the socket URL when connected
		});

		newSocket.on("disconnect", () => {
			setIsConnected(false);
			setSocketUrl(null); // Clear the socket URL when disconnected
		});

		newSocket.on("connect_error", () => {
			setIsConnected(false);
			setSocketUrl(null); // Clear the socket URL on connection error
		});

		setSocket(newSocket);
	};

	const disconnectSocket = () => {
		if (socket) {
			socket.disconnect();
			setSocket(null);
			setIsConnected(false);
			setSocketUrl(null); // Clear the socket URL on disconnect
		}
	};

	// Cleanup socket listeners on unmount
	useEffect(() => {
		return () => {
			if (socket) {
				socket.disconnect();
			}
		};
	}, [socket]);

	return (
		<SocketContext.Provider
			value={{
				socket,
				isConnected,
				socketUrl,
				connectSocket,
				disconnectSocket,
			}}
		>
			{children}
		</SocketContext.Provider>
	);
};
