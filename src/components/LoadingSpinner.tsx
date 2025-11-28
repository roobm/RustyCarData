// Loading spinner utility component

import React from 'react';
import { Loader2 } from 'lucide-react';

interface LoadingSpinnerProps {
  size?: 'small' | 'medium' | 'large';
  text?: string;
}

export const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
  size = 'medium',
  text,
}) => {
  const sizeMap = {
    small: 16,
    medium: 24,
    large: 48,
  };

  const iconSize = sizeMap[size];

  return (
    <div className="flex flex-col items-center justify-center gap-3" role="status" aria-live="polite">
      <Loader2 size={iconSize} className="animate-spin text-indigo-600" />
      {text && (
        <p className="text-sm text-gray-600 dark:text-gray-400">{text}</p>
      )}
      <span className="sr-only">Loading...</span>
    </div>
  );
};
