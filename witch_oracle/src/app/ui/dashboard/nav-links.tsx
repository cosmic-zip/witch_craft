'use client';

import {
  UserGroupIcon,
  HomeIcon,
  DocumentDuplicateIcon,
  UserIcon,
} from '@heroicons/react/24/outline';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import clsx from 'clsx';

const links = [
  { name: 'Home', href: '/dashboard' },
  { name: 'all', href: '/dashboard/tabs/all' },
  { name: 'antivirus', href: '/dashboard/tabs/antivirus' },
  { name: 'attack', href: '/dashboard/tabs/attack' },
  { name: 'bcurl', href: '/dashboard/tabs/bcurl' },
  { name: 'botnet', href: '/dashboard/tabs/botnet' },
  { name: 'lookup', href: '/dashboard/tabs/lookup' },
  { name: 'osint', href: '/dashboard/tabs/osint' },
  { name: 'pods', href: '/dashboard/tabs/pods' },
  { name: 'rootkit', href: '/dashboard/tabs/rootkit' },
  { name: 'scanner', href: '/dashboard/tabs/scanner' },
  { name: 'utils', href: '/dashboard/tabs/utils' },
];

export default function NavLinks() {
  const pathname = usePathname();

  return (
    <>
      {links.map((link) => {
        // const LinkIcon = link.icon;
        return (
          <Link
            key={link.name}
            href={link.href}
            className={clsx(
              'flex h-[40px] grow items-center justify-center rounded-md bg-black text-sm font-medium hover:bg-[#d4d409] hover:text-gray-900 hover:border-black md:flex-none md:justify-start md:px-3 uppercase border-2 border-white',
              {
                'border-[#d4d409] bg-white text-black': pathname === link.href,
              },
            )}
          >
            {/* <LinkIcon className="w-6" /> */}
            <p className="hidden md:block">{link.name}</p>
          </Link>
        );
      })}
    </>
  );
  


}