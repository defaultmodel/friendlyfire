import type React from "react";
import {
	createContext,
	useContext,
	useState,
	useEffect,
	type ReactNode,
} from "react";
import { io, type Socket } from "socket.io-client";
import { version } from "../../../package.json";
import { useSnackbar } from "./SnackbarContext";

const CLIENT_VERSION = version;

type SocketContextType = {
	socket: Socket | null;
	isConnected: boolean;
	socketUrl: string | null;
	connectSocket: (
		serverUrl: string,
		apiKey: string,
		username: string,
	) => Promise<void>;
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
	const { showSnackbar } = useSnackbar();
	const [socket, setSocket] = useState<Socket | null>(null);
	const [isConnected, setIsConnected] = useState(false);
	const [socketUrl, setSocketUrl] = useState<string | null>(null);

	const connectSocket = async (
		serverUrl: string,
		apiKey: string,
		username: string,
	) => {
		return new Promise<void>((resolve, reject) => {
			const newSocket = io(serverUrl);
			newSocket.auth = { key: apiKey, username, version: CLIENT_VERSION };

			newSocket.on("connect", () => {
				setIsConnected(true);
				newSocket.emit("ready");
				setSocketUrl(serverUrl);
				resolve();
			});

			newSocket.on("disconnect", () => {
				setIsConnected(false);
				setSocketUrl(null);
				resolve();
			});

			newSocket.on("connect_error", (error) => {
				newSocket.disconnect();
				setIsConnected(false);
				setSocketUrl(null);
				showSnackbar(`Connection Error: ${error.message}`, "error");
				reject(new Error(`Connection Error: ${error.message}`));
			});

			setSocket(newSocket);
		});
	};

	const disconnectSocket = () => {
		if (socket) {
			socket.disconnect();
			setSocket(null);
			setIsConnected(false);
			setSocketUrl(null);
		}
	};

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
