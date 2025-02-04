// App.tsx
import { moveWindow, Position } from "@tauri-apps/plugin-positioner";
import ImageUploader from "./components/ImageUploader";
import LoginForm from "./components/LoginForm";
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
