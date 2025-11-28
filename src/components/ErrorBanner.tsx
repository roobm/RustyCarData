// Error banner utility component

import React from 'react';
import { AlertCircle, RefreshCw, X } from 'lucide-react';

interface ErrorBannerProps {
  message: string;
  onRetry?: () => void;
  onDismiss?: () => void;
}

export const ErrorBanner: React.FC<ErrorBannerProps> = ({
  message,
  onRetry,
  onDismiss,
}) => {
  return (
    <div
      className="flex items-start gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg"
      role="alert"
      aria-live="assertive"
    >
      <AlertCircle className="text-red-600 dark:text-red-400 mt-0.5" size={20} aria-hidden="true" />
      <div className="flex-1">
        <h3 className="font-semibold text-red-900 dark:text-red-100 mb-1">
          Error
        </h3>
        <p className="text-sm text-red-700 dark:text-red-300">{message}</p>
      </div>
      <div className="flex gap-2">
        {onRetry && (
          <button
            onClick={onRetry}
            className="flex items-center gap-2 px-3 py-2 bg-red-100 dark:bg-red-800 hover:bg-red-200 dark:hover:bg-red-700 text-red-900 dark:text-red-100 rounded-md transition-colors"
            aria-label="Retry action"
          >
            <RefreshCw size={16} aria-hidden="true" />
            Retry
          </button>
        )}
        {onDismiss && (
          <button
            onClick={onDismiss}
            className="text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-200 p-2"
            aria-label="Dismiss error"
          >
            <X size={20} aria-hidden="true" />
          </button>
        )}
      </div>
    </div>
  );
};
