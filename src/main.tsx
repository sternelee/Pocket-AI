import './styles.css';
import '@/i18n';

import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { AnimatePresence } from 'framer-motion';
import { ThemeProvider } from 'next-themes';
import React, { Suspense } from 'react';
import ReactDOM from 'react-dom/client';
import { ErrorBoundary } from 'react-error-boundary';
import {
  createBrowserRouter,
  Navigate,
  Outlet,
  RouterProvider,
  useLocation,
  useOutlet,
} from 'react-router-dom';

import CommonLayout from '@/layouts/CommonLayout';
import {
  ConversationsContextProvider,
  InitializationProvider,
  RQProvider,
} from '@/lib/providers';
import ConversationPage from '@/pages/Conversation';
import ConversationsPage from '@/pages/Conversations';
import ModelsPage from '@/pages/Models';
import NewConversationPage from '@/pages/NewConversation';
import PromptsPage from '@/pages/Prompts';
import SettingsPage from '@/pages/Settings';

import { GlobalFallback } from './components/GlobalFallback';
import { PageSkeleton } from './components/placeholders/WholePage';

const AnimatedOutlet = (): React.JSX.Element => {
  const location = useLocation();
  const element = useOutlet();

  return (
    <AnimatePresence mode="wait" initial={false}>
      {element && React.cloneElement(element, { key: location.pathname })}
    </AnimatePresence>
  );
};

const router = createBrowserRouter([
  {
    path: '/',
    element: (
      <CommonLayout>
        <AnimatedOutlet />
      </CommonLayout>
    ),
    children: [
      {
        index: true,
        element: <Navigate to="/conversations" replace />,
      },
      {
        path: 'models',
        element: <ModelsPage />,
      },
      {
        path: 'conversations',
        element: (
          <ConversationsContextProvider>
            <Outlet />
          </ConversationsContextProvider>
        ),
        children: [
          {
            index: true,
            element: <ConversationsPage />,
          },
          {
            path: 'new',
            element: <NewConversationPage />,
          },
          {
            path: ':conversationId',
            element: <ConversationPage />,
          },
        ],
      },
      {
        path: 'prompts',
        element: <PromptsPage />,
      },
      {
        path: 'settings',
        element: <SettingsPage />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ErrorBoundary FallbackComponent={GlobalFallback}>
      <RQProvider>
        <ThemeProvider defaultTheme="system" attribute="class">
          <Suspense fallback={<PageSkeleton />}>
            <InitializationProvider>
              <RouterProvider router={router} />
            </InitializationProvider>
          </Suspense>
        </ThemeProvider>
        <ReactQueryDevtools initialIsOpen={false} />
      </RQProvider>
    </ErrorBoundary>
  </React.StrictMode>
);
