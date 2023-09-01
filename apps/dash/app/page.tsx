'use client';

import NavBar from '../components/nav-bar/nav-bar';

const Index = () => {
  return (
    <div>
      <NavBar />
      <section className="bg-gray-50 dark:bg-gray-900">
        <div className="px-4 mx-auto max-w-screen-xl flex flex-col item justify-center h-screen">
          <div className="">
            <div className="max-w-2xl mx-auto lg:max-w-xl p-6 space-y-8 sm:p-8 bg-white rounded-lg shadow-xl dark:bg-gray-800">
              <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
                Sign in
              </h2>
              <form className="mt-8 space-y-6" action="#">
                <div>
                  <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                    Your email
                  </label>
                  <input
                    type="email"
                    name="email"
                    id="email"
                    className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="name@company.com"
                    required
                  />
                </div>
                <div>
                  <label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                    Your password
                  </label>
                  <input
                    type="password"
                    name="password"
                    id="password"
                    placeholder="••••••••"
                    className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    required
                  />
                </div>

                <button
                  type="submit"
                  className="w-full px-5 py-3 text-base font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 sm:w-auto dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >
                  Login to your account
                </button>
              </form>
            </div>
          </div>
        </div>
        <footer className="bg-white shadow dark:bg-gray-900 mx-4">
          <div className="w-full max-w-screen-2xl mx-auto py-4 md:py-8">
            <div className="sm:flex sm:items-center sm:justify-between">
              <a href="//" className="flex items-center mb-4 sm:mb-0">
                <span className="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">
                  d.hanz.lol
                </span>
              </a>
            </div>
            <hr className="my-6 border-gray-200 sm:mx-auto dark:border-gray-700 lg:my-8" />
            <span className="block text-sm text-gray-500 sm:text-center dark:text-gray-400">
              <a
                href="https://github.com/fivehanz/"
                className="hover:underline"
              >
                Hanz
              </a>
              . © {new Date().getFullYear()} All Rights Reserved.
            </span>
          </div>
        </footer>
      </section>
    </div>
  );
};

export default Index;
