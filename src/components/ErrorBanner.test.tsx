import { describe, it, expect, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import { userEvent } from '@testing-library/user-event'
import { ErrorBanner } from './ErrorBanner'

describe('ErrorBanner', () => {
  it('renders error message', () => {
    render(<ErrorBanner message="Something went wrong" />)

    expect(screen.getByText('Error')).toBeInTheDocument()
    expect(screen.getByText('Something went wrong')).toBeInTheDocument()
  })

  it('has proper ARIA attributes', () => {
    const { container } = render(<ErrorBanner message="Test error" />)

    const alertDiv = container.querySelector('[role="alert"]')
    expect(alertDiv).toBeInTheDocument()
    expect(alertDiv).toHaveAttribute('aria-live', 'assertive')
  })

  it('shows retry button when onRetry is provided', () => {
    const onRetry = vi.fn()
    render(<ErrorBanner message="Test error" onRetry={onRetry} />)

    const retryButton = screen.getByRole('button', { name: /retry action/i })
    expect(retryButton).toBeInTheDocument()
  })

  it('does not show retry button when onRetry is not provided', () => {
    render(<ErrorBanner message="Test error" />)

    const retryButton = screen.queryByRole('button', { name: /retry action/i })
    expect(retryButton).not.toBeInTheDocument()
  })

  it('calls onRetry when retry button is clicked', async () => {
    const user = userEvent.setup()
    const onRetry = vi.fn()
    render(<ErrorBanner message="Test error" onRetry={onRetry} />)

    const retryButton = screen.getByRole('button', { name: /retry action/i })
    await user.click(retryButton)

    expect(onRetry).toHaveBeenCalledOnce()
  })

  it('shows dismiss button when onDismiss is provided', () => {
    const onDismiss = vi.fn()
    render(<ErrorBanner message="Test error" onDismiss={onDismiss} />)

    const dismissButton = screen.getByRole('button', { name: /dismiss error/i })
    expect(dismissButton).toBeInTheDocument()
  })

  it('does not show dismiss button when onDismiss is not provided', () => {
    render(<ErrorBanner message="Test error" />)

    const dismissButton = screen.queryByRole('button', { name: /dismiss error/i })
    expect(dismissButton).not.toBeInTheDocument()
  })

  it('calls onDismiss when dismiss button is clicked', async () => {
    const user = userEvent.setup()
    const onDismiss = vi.fn()
    render(<ErrorBanner message="Test error" onDismiss={onDismiss} />)

    const dismissButton = screen.getByRole('button', { name: /dismiss error/i })
    await user.click(dismissButton)

    expect(onDismiss).toHaveBeenCalledOnce()
  })

  it('shows both retry and dismiss buttons when both callbacks provided', () => {
    const onRetry = vi.fn()
    const onDismiss = vi.fn()
    render(<ErrorBanner message="Test error" onRetry={onRetry} onDismiss={onDismiss} />)

    expect(screen.getByRole('button', { name: /retry action/i })).toBeInTheDocument()
    expect(screen.getByRole('button', { name: /dismiss error/i })).toBeInTheDocument()
  })

  it('displays alert icon', () => {
    const { container } = render(<ErrorBanner message="Test error" />)

    // The AlertCircle icon from lucide-react renders as an svg
    const icon = container.querySelector('svg')
    expect(icon).toBeInTheDocument()
    expect(icon).toHaveClass('text-red-600')
  })
})
