// LoginForm.tsx
import type React from "react";
import { useEffect, useState } from "react";
import { useSocket } from "../../context/SocketContext";
import {
	Box,
	TextField,
	Button,
	Typography,
	Container,
	Paper,
	Alert,
} from "@mui/material";

const LoginForm: React.FC = () => {
	const { socket, connectSocket, disconnectSocket } = useSocket();
	const [username, setUsername] = useState("");
	const [socketUrl, setSocketUrl] = useState("");
	const [apiKey, setApiKey] = useState("");
	const [isConnected, setIsConnected] = useState(socket?.connected);
	const [error, setError] = useState<string | null>(null);

	// Load saved inputs from local storage when the component mounts
	useEffect(() => {
		const savedUsername = localStorage.getItem("username");
		const savedApiKey = localStorage.getItem("apiKey");
		const savedSocketUrl = localStorage.getItem("socketUrl");

		if (savedUsername) setUsername(savedUsername);
		if (savedApiKey) setApiKey(savedApiKey);
		if (savedSocketUrl) setSocketUrl(savedSocketUrl);
	}, []);

	// Save inputs to local storage whenever they change
	useEffect(() => {
		localStorage.setItem("username", username);
	}, [username]);

	useEffect(() => {
		localStorage.setItem("apiKey", apiKey);
	}, [apiKey]);

	useEffect(() => {
		localStorage.setItem("socketUrl", socketUrl);
	}, [socketUrl]);

	// Set up socket listeners when the socket changes
	useEffect(() => {
		if (!socket) return;

		const handleConnectEvent = () => {
			setIsConnected(true);
			setError(null);
		};

		const handleConnectErrorEvent = (err: Error) => {
			disconnectSocket();
			setIsConnected(false);
			setError(`Failed to connect to the server: ${err.message}`);
		};

		socket.on("connect", handleConnectEvent);
		socket.on("connect_error", handleConnectErrorEvent);

		// Cleanup listeners on unmount or socket change
		return () => {
			socket.off("connect", handleConnectEvent);
			socket.off("connect_error", handleConnectErrorEvent);
		};
	}, [socket, disconnectSocket]);

	const handleConnect = () => {
		if (!username.trim()) {
			setError("Username cannot be empty");
			return;
		}

		if (!socketUrl.trim()) {
			setError("Socket URL cannot be empty");
			return;
		}

		connectSocket(socketUrl, apiKey, username);
	};

	const handleDisconnect = () => {
		disconnectSocket();
		setIsConnected(false);
	};

	return (
		<Container maxWidth="sm">
			<Paper elevation={3} sx={{ padding: 4, marginTop: 4 }}>
				<Typography variant="h4" component="h1" align="center" gutterBottom>
					Login
				</Typography>
				<Box
					component="form"
					sx={{ display: "flex", flexDirection: "column", gap: 3 }}
				>
					<TextField
						label="Username"
						variant="outlined"
						value={username}
						onChange={(e) => setUsername(e.target.value)}
						placeholder="Enter your username"
						fullWidth
						required
					/>
					<TextField
						label="API Key"
						variant="outlined"
						value={apiKey}
						onChange={(e) => setApiKey(e.target.value)}
						placeholder="Enter your API Key"
						fullWidth
					/>
					<TextField
						label="Socket URL"
						variant="outlined"
						value={socketUrl}
						onChange={(e) => setSocketUrl(e.target.value)}
						placeholder="Enter Socket URL"
						fullWidth
						required
					/>
					{isConnected ? (
						<Button
							variant="contained"
							color="error"
							onClick={handleDisconnect}
							fullWidth
						>
							Disconnect
						</Button>
					) : (
						<Button
							variant="contained"
							color="primary"
							onClick={handleConnect}
							disabled={!username || !socketUrl}
							fullWidth
						>
							Connect
						</Button>
					)}
					<Typography variant="body1" align="center">
						State: {isConnected ? "Connected" : "Disconnected"}
					</Typography>
					{error && (
						<Alert severity="error" sx={{ marginTop: 2 }}>
							{error}
						</Alert>
					)}
				</Box>
			</Paper>
		</Container>
	);
};

export default LoginForm;
