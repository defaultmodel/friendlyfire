// App.tsx
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
import ImageUploader from "./components/image/ImageUploader";
import LoginForm from "./components/auth/LoginForm";
import UserList from "./components/UserList";
import { useSocket } from "./SocketContext";

export default function App() {
	moveWindow(Position.TopLeft);

	const { isConnected } = useSocket();
	return (
		<main className="app">
			{isConnected ? (
				<>
					<ImageUploader />
					<UserList />
				</>
			) : (
				<LoginForm />
			)}
		</main>
	);
}
