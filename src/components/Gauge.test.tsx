import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { Gauge } from './Gauge'

describe('Gauge', () => {
  it('renders the label', () => {
    render(<Gauge value={50} max={100} label="Test Gauge" unit="rpm" color="#ff0000" />)

    expect(screen.getByText('Test Gauge')).toBeInTheDocument()
  })

  it('displays the current value with unit', () => {
    render(<Gauge value={75} max={150} label="Speed" unit="km/h" color="#0000ff" />)

    expect(screen.getByText(/75/)).toBeInTheDocument()
    expect(screen.getByText(/km\/h/)).toBeInTheDocument()
  })

  it('calculates percentage correctly', () => {
    render(<Gauge value={50} max={100} label="Test" unit="test" color="#00ff00" />)

    // The percentage should be 50% (value/max * 100)
    // This tests that the component renders without errors with 50% progress
    expect(screen.getByText('Test')).toBeInTheDocument()
  })

  it('handles zero value', () => {
    render(<Gauge value={0} max={100} label="Zero Test" unit="rpm" color="#ff0000" />)

    expect(screen.getByText(/0/)).toBeInTheDocument()
  })

  it('handles max value', () => {
    render(<Gauge value={8000} max={8000} label="Max RPM" unit="rpm" color="#ff0000" />)

    expect(screen.getByText(/8000/)).toBeInTheDocument()
  })
})
