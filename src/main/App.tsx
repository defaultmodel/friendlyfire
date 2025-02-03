// App.tsx
import ImageUploader from "./components/ImageUploader";
import LoginForm from "./components/LoginForm";
import UserList from "./components/UserList";
import { useSocket } from "./SocketContext";

export default function App() {
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
