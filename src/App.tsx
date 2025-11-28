import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import { DashboardPage } from './pages/DashboardPage';
import { AnalysisPage } from './pages/AnalysisPage';
import SessionHistoryPage from './pages/SessionHistoryPage';
import SessionDetailPage from './pages/SessionDetailPage';
import { AgentDashboard } from './pages/AgentDashboard';
import "./App.css";

function App() {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/analysis" element={<AnalysisPage />} />
          <Route path="/agents" element={<AgentDashboard />} />
          <Route path="/history" element={<SessionHistoryPage />} />
          <Route path="/history/:sessionId" element={<SessionDetailPage />} />
        </Routes>
      </Layout>
    </Router>
  );
}

export default App;
