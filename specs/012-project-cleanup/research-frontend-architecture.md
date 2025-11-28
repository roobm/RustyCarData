# React TypeScript Frontend Architecture - Best Practices (2024-2025)

## Project Structure

### Recommended Folder Structure

All structure folders are typically inside the `src/` folder. Modern React TypeScript projects should use a modular, flexible structure:

```
src/
├── components/       # Reusable UI components (Button, Card, Modal, Input, etc.)
├── pages/           # Page/view components (DashboardPage, SettingsPage, etc.)
├── hooks/           # Custom React hooks accessible by many components
├── contexts/        # React context files for shared state
├── types/           # Centralized type & interface definitions
├── layouts/         # Layout components (Sidebar, Navbar, etc.)
├── config/          # Configuration files and environment variables
├── services/        # API services and external integrations
├── store/           # State management (if using Zustand, Redux, etc.)
└── utils/           # Utility functions and helpers
```

### Key Principles

1. **Modularity**: Organize by features or domains for larger projects
2. **Reusability**: Components, hooks, and utilities should be easily shared
3. **One Component Per File**: Each file should export only one React component
4. **Index Files**: Use `index.ts` files as single access points for directories

## TypeScript Best Practices

### Type Organization

- **Centralized Types**: Store all type definitions in `/types` folder
- **Avoid `.d.ts` files**: These put types in global scope; use regular `.ts` files instead
- **Type vs Interface**: Use `interface` for object shapes, `type` for unions/primitives
- **Strict Mode**: Enable `"strict": true` in `tsconfig.json` for compile-time safety

### Example Type Organization

```typescript
// types/session.ts
export interface Session {
  id: string;
  name: string;
  startedAt: Date;
}

export type SessionStatus = 'active' | 'completed' | 'error';
```

## Component Organization

### File Naming Conventions

- Use `.tsx` extension for components with JSX
- Use PascalCase for component files: `SessionNameDialog.tsx`
- Use camelCase for utilities: `formatDate.ts`
- One React component per file

### Component Structure

```typescript
// components/Button/Button.tsx
import { FC } from 'react';
import './Button.css';

interface ButtonProps {
  label: string;
  onClick: () => void;
  variant?: 'primary' | 'secondary';
}

export const Button: FC<ButtonProps> = ({ label, onClick, variant = 'primary' }) => {
  return (
    <button className={`btn btn-${variant}`} onClick={onClick}>
      {label}
    </button>
  );
};
```

## State Management

### Modern Approaches (2024-2025)

1. **Zustand** - Recommended for most projects
   - Simple, minimal boilerplate
   - Good TypeScript support
   - No Context provider wrapper needed

2. **Redux Toolkit** - For complex applications
   - More boilerplate but very structured
   - Excellent DevTools
   - Good for large teams

3. **React Context** - For simple shared state
   - Built-in, no dependencies
   - Can cause unnecessary re-renders if not careful
   - Good for theme, auth, i18n

### Store Organization

```typescript
// store/useSessionStore.ts
import { create } from 'zustand';

interface SessionStore {
  sessions: Session[];
  isLoading: boolean;
  fetchSessions: () => Promise<void>;
}

export const useSessionStore = create<SessionStore>((set) => ({
  sessions: [],
  isLoading: false,
  fetchSessions: async () => {
    set({ isLoading: true });
    const sessions = await api.getSessions();
    set({ sessions, isLoading: false });
  },
}));
```

## Custom Hooks Best Practices

### Guidelines

- Start hook names with `use`
- Extract reusable logic from components
- Keep hooks focused on single responsibility
- Return objects for multiple values (better than arrays)

### Example

```typescript
// hooks/useSession.ts
import { useState, useEffect } from 'react';

export const useSession = (sessionId: string) => {
  const [session, setSession] = useState<Session | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const fetchSession = async () => {
      try {
        const data = await api.getSession(sessionId);
        setSession(data);
      } catch (err) {
        setError(err as Error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchSession();
  }, [sessionId]);

  return { session, isLoading, error };
};
```

## Testing Setup (Vitest + React Testing Library)

### Installation

```bash
npm install -D vitest jsdom @testing-library/react @testing-library/dom @testing-library/jest-dom @testing-library/user-event @types/react @types/react-dom
```

### Configuration

**vite.config.ts:**
```typescript
/// <reference types="vitest" />
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './vitest.setup.ts',
  }
})
```

**vitest.setup.ts:**
```typescript
import { expect, afterEach } from 'vitest'
import { cleanup } from '@testing-library/react'
import * as matchers from '@testing-library/jest-dom/matchers'

expect.extend(matchers)

afterEach(() => {
  cleanup()
})
```

**tsconfig.json:**
```json
{
  "compilerOptions": {
    "types": ["vitest/globals", "@testing-library/jest-dom"]
  }
}
```

### Test Organization

- Co-locate tests with components: `Button.test.tsx` next to `Button.tsx`
- Or use `/tests` directory for integration tests
- Use descriptive test names: `it('should disable button when loading')`

### Testing Best Practices

**What to Test:**
- User interactions (clicks, inputs)
- Conditional rendering
- Data fetching and loading states
- Error states

**What NOT to Test:**
- Implementation details
- Third-party library internals
- Styling (unless critical to functionality)

**Example Test:**
```typescript
// components/Button.test.tsx
import { describe, it, expect, vi } from 'vitest'
import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { Button } from './Button'

describe('Button', () => {
  it('calls onClick when clicked', async () => {
    const handleClick = vi.fn()
    render(<Button label="Click me" onClick={handleClick} />)

    await userEvent.click(screen.getByRole('button'))

    expect(handleClick).toHaveBeenCalledOnce()
  })
})
```

## Performance Best Practices

### Modern React 18+ Patterns

1. **Avoid Premature Optimization**: Profile first, optimize what's actually slow
2. **Use `useMemo` sparingly**: Only for expensive calculations
3. **Use `useCallback` for child props**: Prevent unnecessary child re-renders
4. **Code Splitting**: Use `React.lazy()` for route-based splitting

### Example

```typescript
// Lazy load heavy pages
const AnalysisPage = lazy(() => import('./pages/AnalysisPage'));

function App() {
  return (
    <Suspense fallback={<Loading />}>
      <Routes>
        <Route path="/analysis" element={<AnalysisPage />} />
      </Routes>
    </Suspense>
  );
}
```

## Application to Current Project

### Current Structure Assessment

**Good:**
- ✅ Separate `/components`, `/pages`, `/services` directories
- ✅ TypeScript throughout
- ✅ Zustand for state management

**Improvements Needed:**
- ❌ Missing `/hooks` directory (hooks scattered in components)
- ❌ Missing `/types` directory (types defined in various files)
- ❌ No test setup yet
- ❌ Some unused components (AgentDashboard not routed)

### Recommended Changes

1. **Create `/types` directory**: Move all interfaces to centralized location
2. **Create `/hooks` directory**: Extract custom hooks
3. **Set up Vitest**: Add testing infrastructure
4. **Remove unused code**: Delete AgentDashboard and related files
5. **Add tests**: Start with critical components (Gauge, TelemetryChart)

## References

- [Effective React TypeScript Project Structure (Medium)](https://medium.com/@tusharupadhyay691/effective-react-typescript-project-structure-best-practices-for-scalability-and-maintainability-bcbcf0e09bd5)
- [Best Practices for Structuring React TypeScript (Medium)](https://thiraphat-ps-dev.medium.com/best-practices-for-structuring-a-react-typescript-project-f5ee7f9a264e)
- [Recommended Folder Structure for React 2025 (DEV)](https://dev.to/pramod_boda/recommended-folder-structure-for-react-2025-48mc)
- [React Folder Structure in 5 Steps (Robin Wieruch)](https://www.robinwieruch.de/react-folder-structure/)
- [Configuring Vite with TypeScript, Vitest, and React Testing Library (John Smilga)](https://johnsmilga.com/articles/2024/10/15)
- [React Testing Setup: Vitest + TypeScript (DEV)](https://dev.to/kevinccbsg/react-testing-setup-vitest-typescript-react-testing-library-42c8)
