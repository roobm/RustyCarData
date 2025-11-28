import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, screen } from '@testing-library/react'
import { userEvent } from '@testing-library/user-event'
import { TelemetryChart } from './TelemetryChart'
import { useStore } from '../store/useStore'
import { api } from '../services/api'

// Mock the store
vi.mock('../store/useStore')
// Mock the API service
vi.mock('../services/api')
// Mock window.alert
vi.stubGlobal('alert', vi.fn())

describe('TelemetryChart', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('renders the chart title', () => {
    vi.mocked(useStore).mockReturnValue({
      history: [],
      addDataPoint: vi.fn(),
      clearHistory: vi.fn(),
      isConnected: false,
      connect: vi.fn(),
      disconnect: vi.fn(),
      sessionId: null,
      setSessionId: vi.fn(),
    })

    render(<TelemetryChart />)
    expect(screen.getByText('Live Telemetry')).toBeInTheDocument()
  })

  it('disables export button when history is empty', () => {
    vi.mocked(useStore).mockReturnValue({
      history: [],
      addDataPoint: vi.fn(),
      clearHistory: vi.fn(),
      isConnected: false,
      connect: vi.fn(),
      disconnect: vi.fn(),
      sessionId: null,
      setSessionId: vi.fn(),
    })

    render(<TelemetryChart />)
    const exportButton = screen.getByRole('button', { name: /export csv/i })
    expect(exportButton).toBeDisabled()
  })

  it('enables export button when history has data', () => {
    vi.mocked(useStore).mockReturnValue({
      history: [
        { timestamp: new Date('2025-01-01T12:00:00').getTime(), rpm: 2000, speed: 60, temp: 85 },
      ],
      addDataPoint: vi.fn(),
      clearHistory: vi.fn(),
      isConnected: false,
      connect: vi.fn(),
      disconnect: vi.fn(),
      sessionId: null,
      setSessionId: vi.fn(),
    })

    render(<TelemetryChart />)
    const exportButton = screen.getByRole('button', { name: /export csv/i })
    expect(exportButton).not.toBeDisabled()
  })

  it('exports CSV with correct data when button is clicked', async () => {
    const user = userEvent.setup()
    const mockSaveExportCsv = vi.fn().mockResolvedValue('/path/to/file.csv')
    vi.mocked(api).saveExportCsv = mockSaveExportCsv

    const testData = [
      { timestamp: new Date('2025-01-01T12:00:00').getTime(), rpm: 2000, speed: 60, temp: 85 },
      { timestamp: new Date('2025-01-01T12:00:01').getTime(), rpm: 2500, speed: 65, temp: 86 },
    ]

    vi.mocked(useStore).mockReturnValue({
      history: testData,
      addDataPoint: vi.fn(),
      clearHistory: vi.fn(),
      isConnected: false,
      connect: vi.fn(),
      disconnect: vi.fn(),
      sessionId: null,
      setSessionId: vi.fn(),
    })

    render(<TelemetryChart />)
    const exportButton = screen.getByRole('button', { name: /export csv/i })

    await user.click(exportButton)

    expect(mockSaveExportCsv).toHaveBeenCalledOnce()
    const [csvContent, filename] = mockSaveExportCsv.mock.calls[0]

    // Verify CSV headers
    expect(csvContent).toContain('Timestamp,RPM,Speed,CoolantTemp')
    // Verify CSV data rows (ISO timestamps will be in UTC format)
    expect(csvContent).toContain('2025-01-01')
    expect(csvContent).toContain('2000.00,60.00,85.00')
    expect(csvContent).toContain('2500.00,65.00,86.00')
    // Verify filename pattern
    expect(filename).toMatch(/^telemetry_export_\d{4}-\d{2}-\d{2}T\d{2}-\d{2}-\d{2}/)
  })

  it('shows alert on successful export', async () => {
    const user = userEvent.setup()
    const mockAlert = vi.fn()
    vi.stubGlobal('alert', mockAlert)

    vi.mocked(api).saveExportCsv = vi.fn().mockResolvedValue('/path/to/export.csv')

    vi.mocked(useStore).mockReturnValue({
      history: [
        { timestamp: new Date('2025-01-01T12:00:00').getTime(), rpm: 2000, speed: 60, temp: 85 },
      ],
      addDataPoint: vi.fn(),
      clearHistory: vi.fn(),
      isConnected: false,
      connect: vi.fn(),
      disconnect: vi.fn(),
      sessionId: null,
      setSessionId: vi.fn(),
    })

    render(<TelemetryChart />)
    const exportButton = screen.getByRole('button', { name: /export csv/i })

    await user.click(exportButton)

    expect(mockAlert).toHaveBeenCalledWith('Export saved to: /path/to/export.csv')
  })

  it('shows alert on export failure', async () => {
    const user = userEvent.setup()
    const mockAlert = vi.fn()
    vi.stubGlobal('alert', mockAlert)

    vi.mocked(api).saveExportCsv = vi.fn().mockRejectedValue(new Error('Save failed'))

    vi.mocked(useStore).mockReturnValue({
      history: [
        { timestamp: new Date('2025-01-01T12:00:00').getTime(), rpm: 2000, speed: 60, temp: 85 },
      ],
      addDataPoint: vi.fn(),
      clearHistory: vi.fn(),
      isConnected: false,
      connect: vi.fn(),
      disconnect: vi.fn(),
      sessionId: null,
      setSessionId: vi.fn(),
    })

    render(<TelemetryChart />)
    const exportButton = screen.getByRole('button', { name: /export csv/i })

    await user.click(exportButton)

    expect(mockAlert).toHaveBeenCalledWith('Failed to export CSV. Check console for details.')
  })
})
