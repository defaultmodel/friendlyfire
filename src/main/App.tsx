import { useServerContext } from "./contexts/ServerContext";
import GetStartedPage from "./pages/GetStartedPage";

export default function App() {
	const { servers, loading } = useServerContext();

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
			{servers.map((server) => (
				<p key={server.serverName}>{server.serverName}</p>
			))}
		</main>
	);
}
