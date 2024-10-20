"use client";
import { useState } from "react";
import Image from "next/image";

export default function Header() {

  return (
    
    <nav className="flex items-center justify-between flex-wrap w-full bg-[#0a0a0a] p-6 fixed">
          <div className="flex items-center flex-shrink-0 text-white mr-6">
            <a href="/">
            <span className="font-semibold text-xl tracking-tight">
              Pint Auction
            </span>
            </a>
          </div>
          <div className="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
            <div className="ml-4 text-sm lg:flex-grow">
              <a
                href="/auction"
                className="block mx-6 lg:inline-block lg:mt-0 text-slate-300 hover:text-slate-400"
                >
                View Auctions
              </a>
              <a
                href="/host"
                className="mx-6 block mt-4 lg:inline-block lg:mt-0 text-slate-300 hover:text-slate-400"
                >
                Host Auction
              </a>
            </div>
            <div>
              <a
                href="/bid"
                className="inline-block text-sm px-4 py-2 leading-none border rounded text-white border-white hover:border-transparent hover:text-black hover:bg-slate-200 mt-4 lg:mt-0"
                >
                How To Bid ?
              </a>
            </div>
          </div>
        </nav>
  );
}
