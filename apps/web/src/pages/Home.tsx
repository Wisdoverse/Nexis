import { useState } from 'react';
import { useAuth } from '../hooks/useAuth';
import { useWebSocket } from '../hooks/useWebSocket';

export default function Home() {
  const { user, logout } = useAuth();
  const { connected } = useWebSocket();
  const [sidebarOpen, setSidebarOpen] = useState(true);

  return (
    <div className="h-screen flex bg-gray-100">
      <aside className={`${sidebarOpen ? 'w-64' : 'w-16'} bg-gray-900 text-white transition-all`}>
        <button onClick={() => setSidebarOpen(!sidebarOpen)} className="p-4">
          {sidebarOpen ? '◀' : '▶'}
        </button>
        {sidebarOpen && (
          <div className="p-4">
            <h2 className="font-semibold mb-4">Rooms</h2>
            <p className="text-gray-400 text-sm">No rooms yet</p>
          </div>
        )}
      </aside>

      <main className="flex-1 flex flex-col">
        <header className="h-14 bg-white border-b flex items-center justify-between px-4">
          <h1 className="text-lg font-semibold text-gray-900">Nexis Chat</h1>
          <div className="flex items-center gap-3">
            <span className={`inline-block w-2 h-2 rounded-full ${connected ? 'bg-green-500' : 'bg-red-500'}`} />
            <span className="text-sm text-gray-600">{user?.displayName}</span>
            <button onClick={logout} className="text-sm text-gray-500 hover:text-red-500">Logout</button>
          </div>
        </header>

        <div className="flex-1 flex items-center justify-center">
          <div className="text-center space-y-3">
            <div className="text-5xl">💬</div>
            <h2 className="text-xl font-semibold text-gray-700">Welcome to Nexis</h2>
            <p className="text-gray-500">AI teammates are standing by.</p>
          </div>
        </div>
      </main>
    </div>
  );
}
