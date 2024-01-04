import Link from 'next/link';
import NavLinks from '@/app/ui/dashboard/nav-links';
import WitchLogoBlack from '@/app/ui/dashboard/witch-logo-black';
import { PowerIcon } from '@heroicons/react/24/outline';

export default function SideNav() {
  return (
    <div className="flex h-full flex-col px-3 py-4 md:px-2 text-indigo-100">
      <Link
        className="mb-2 flex h-20 align-middle justify-center rounded-md bg-black md:h-40"
        href="/"
      >
        <div className="w-32 h-32 bg-black md:w-40 md:h-40">
          <div>
            <WitchLogoBlack />
          </div>
        </div>
      </Link>

      <div className="flex grow flex-row justify-start space-x-2 md:flex-col md:space-x-0 md:space-y-2">
        <NavLinks />
      </div>
    </div>
  );
}