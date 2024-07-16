import { Bug, MessageSquare, Package, Puzzle, Settings } from 'lucide-react';
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Link, useLocation, useNavigate } from 'react-router-dom';

import { cn } from '@/lib/utils';

import { Badge } from './ui/badge';
import { Button } from './ui/button';
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from './ui/context-menu';

type MenuProps = {
  expanded: boolean;
};

type MenuItemProps = {
  text: string;
  to: string;
  icon: React.ReactNode;
  expanded: boolean;
  active: boolean;
  className?: string;
};

function SideNavMenuItem({
  text,
  to,
  icon,
  expanded,
  active,
  className: extraClassName = '',
}: MenuItemProps) {
  return (
    <Button
      className={cn(
        'flex text-base font-bold rounded-2xl mb-4 h-12 cursor-pointer items-start justify-start shadow-none hover:bg-[--gray-a4] active:bg-[--gray-a5] text-foreground transition-all overflow-hidden',
        expanded ? 'w-72 ml-4' : 'w-12 ml-2',
        active ? 'bg-[--gray-a3]' : 'bg-transparent',
        extraClassName
      )}
      title={text}
      variant="ghost"
      asChild
    >
      <Link to={to}>
        <span className="my-auto">{icon}</span>
        <span className={cn('my-auto ml-4')}>{text}</span>
      </Link>
    </Button>
  );
}

export function SideNavMenu({ expanded = false }: MenuProps) {
  const navigate = useNavigate();
  const { pathname } = useLocation();
  const { t } = useTranslation('generic');

  return (
    <ul
      className={cn(
        'grow flex flex-col justify-start items-start transition-[margin] mt-32'
      )}
    >
      <SideNavMenuItem
        text={t('nav.conversations')}
        icon={<MessageSquare className="size-4 text-foreground" />}
        expanded={expanded}
        active={pathname === '/conversations'}
        to="/conversations"
      />
      <SideNavMenuItem
        text={t('nav.prompts')}
        icon={<Puzzle className="size-4 text-foreground" />}
        expanded={expanded}
        active={pathname === '/prompts'}
        to="/prompts"
      />
      <SideNavMenuItem
        text={t('nav.models')}
        icon={<Package className="size-4 text-foreground" />}
        expanded={expanded}
        active={pathname === '/models'}
        to="/models"
      />
      <SideNavMenuItem
        text={t('nav.settings')}
        icon={<Settings className="size-4 text-foreground" />}
        expanded={expanded}
        active={pathname === '/settings'}
        className="mt-auto"
        to="/settings"
      />
      <ContextMenu key="hidden-debug-menu">
        <ContextMenuTrigger>
          <Badge
            className={cn(
              'w-12 flex justify-center my-6 rounded-full bg-muted text-muted-foreground transition-all',
              expanded ? 'ml-4' : 'ml-2'
            )}
          >
            Free
          </Badge>
        </ContextMenuTrigger>
        <ContextMenuContent>
          <ContextMenuItem
            className="cursor-pointer gap-2"
            onClick={() => navigate('/debug')}
          >
            <Bug className="size-4" /> {t('generic:action:debug')}
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>
    </ul>
  );
}
