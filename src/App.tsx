import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import { DashboardPage } from './pages/DashboardPage';
import { UnifiedAnalysisPage } from './pages/UnifiedAnalysisPage';
import SessionHistoryPage from './pages/SessionHistoryPage';
import SessionDetailPage from './pages/SessionDetailPage';
import "./App.css";

function App() {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/analysis" element={<UnifiedAnalysisPage />} />
          <Route path="/history" element={<SessionHistoryPage />} />
          <Route path="/history/:sessionId" element={<SessionDetailPage />} />
        </Routes>
      </Layout>
    </Router>
  );
}

export default App;
