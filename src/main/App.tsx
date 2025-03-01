import { useServerContext } from "./contexts/ServerContext";
import { useSocket } from "./contexts/SocketContext";
import GetStartedPage from "./pages/GetStartedPage";
import ImageUploaderPage from "./pages/ImageUploaderPage";
import ServerSelectionPage from "./pages/ServerSelectionPage";

export default function App() {
	const { servers, loading } = useServerContext();
	const { isConnected } = useSocket();
	// Show loading state while servers are being loaded
	if (loading) {
		return <div>Loading...</div>;
	}

	// Show the Get Started page if there are no servers
	if (servers.length === 0) {
		return <GetStartedPage />;
	}

	return (
		<main className="app">
			{isConnected ? <ImageUploaderPage /> : <ServerSelectionPage />}
		</main>
	);
}
