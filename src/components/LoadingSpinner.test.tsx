import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import { LoadingSpinner } from './LoadingSpinner'

describe('LoadingSpinner', () => {
  it('renders with default props', () => {
    render(<LoadingSpinner />)

    // Check for screen reader text
    expect(screen.getByText('Loading...')).toBeInTheDocument()
    // Check for status role
    const statusDiv = screen.getByRole('status')
    expect(statusDiv).toBeInTheDocument()
  })

  it('displays custom text when provided', () => {
    render(<LoadingSpinner text="Loading sessions..." />)

    expect(screen.getByText('Loading sessions...')).toBeInTheDocument()
  })

  it('does not display text when not provided', () => {
    const { container } = render(<LoadingSpinner />)

    // Only the screen reader text should be present
    const paragraphs = container.querySelectorAll('p')
    expect(paragraphs).toHaveLength(0)
  })

  it('renders with small size', () => {
    const { container } = render(<LoadingSpinner size="small" />)

    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
    expect(svg).toHaveAttribute('width', '16')
    expect(svg).toHaveAttribute('height', '16')
  })

  it('renders with medium size by default', () => {
    const { container } = render(<LoadingSpinner />)

    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
    expect(svg).toHaveAttribute('width', '24')
    expect(svg).toHaveAttribute('height', '24')
  })

  it('renders with large size', () => {
    const { container } = render(<LoadingSpinner size="large" />)

    const svg = container.querySelector('svg')
    expect(svg).toBeInTheDocument()
    expect(svg).toHaveAttribute('width', '48')
    expect(svg).toHaveAttribute('height', '48')
  })

  it('has proper ARIA attributes for accessibility', () => {
    render(<LoadingSpinner text="Please wait" />)

    const statusDiv = screen.getByRole('status')
    expect(statusDiv).toHaveAttribute('aria-live', 'polite')
  })

  it('includes animation classes', () => {
    const { container } = render(<LoadingSpinner />)

    const svg = container.querySelector('svg')
    expect(svg).toHaveClass('animate-spin')
    expect(svg).toHaveClass('text-indigo-600')
  })
})
