import { Navbar } from 'flowbite-react';

/* eslint-disable-next-line */
export interface NavBarProps {}

export function NavBar(props: NavBarProps) {
  return (
    <Navbar>
      <Navbar.Brand href="/#">
        <span className="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
          d.hanz.lol
        </span>
      </Navbar.Brand>
    </Navbar>
  );
}

export default NavBar;
