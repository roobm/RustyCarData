import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { LayoutDashboard, BrainCircuit, History, Settings } from 'lucide-react';

interface LayoutProps {
  children: React.ReactNode;
}

export const Layout: React.FC<LayoutProps> = ({ children }) => {
  const location = useLocation();

  const isActive = (path: string) => location.pathname === path;

  return (
    <div className="flex h-screen bg-gray-900 text-white">
      {/* Sidebar */}
      <aside className="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
        <div className="p-6 border-b border-gray-700">
          <h1 className="text-2xl font-bold text-blue-400 flex items-center gap-2">
            RustyAssCarData
          </h1>
        </div>

        <nav className="flex-1 p-4 space-y-2">
          <Link
            to="/"
            className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
              isActive('/') 
                ? 'bg-blue-600 text-white' 
                : 'text-gray-400 hover:bg-gray-700 hover:text-white'
            }`}
          >
            <LayoutDashboard size={20} />
            <span>Live Dashboard</span>
          </Link>

          <Link
            to="/analysis"
            className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
              isActive('/analysis')
                ? 'bg-purple-600 text-white'
                : 'text-gray-400 hover:bg-gray-700 hover:text-white'
            }`}
          >
            <BrainCircuit size={20} />
            <span>AI Analysis</span>
          </Link>

          <Link
            to="/history"
            className={`flex items-center gap-3 px-4 py-3 rounded-lg transition-colors ${
              isActive('/history') || location.pathname.startsWith('/history/')
                ? 'bg-green-600 text-white'
                : 'text-gray-400 hover:bg-gray-700 hover:text-white'
            }`}
          >
            <History size={20} />
            <span>Session History</span>
          </Link>
        </nav>

        <div className="p-4 border-t border-gray-700">
          <button className="flex items-center gap-3 px-4 py-3 rounded-lg text-gray-400 hover:bg-gray-700 hover:text-white w-full transition-colors">
            <Settings size={20} />
            <span>Settings</span>
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 overflow-auto">
        {children}
      </main>
    </div>
  );
};
