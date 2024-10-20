import Image from "next/image";
import Header from "../components/header";
import Card from "../components/card";

export default function Bid() {
  return (
    <>
      <Header></Header>
      <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
        <main className="flex flex-col gap-8 row-start-2 items-center">
          <ol className=" space-y-8 list-inside list-decimal text-sm text-center sm:text-left font-[family-name:var(--font-geist-mono)]">
            <li className="mb-2">
              You need to have the essential tookit installed, follow the{" "}
              <a href="https://essential-contributions.github.io/essential-integration/getting-started/installation/index.html">
                <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
                  Installation Guide
                </code>{" "}
              </a>
              .{" "}
            </li>
            <li className="mb-2">
              Create your wallet by running{" "}
              <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
                essential-wallet generate &lt;NAME&gt;
              </code>{" "}
              , you will be prompted for a password.<br></br> You can view your
              public key using{" "}
              <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
                essential-wallet print-pub-key --hashed &lt;NAME&gt;
              </code>{" "}
            </li>
            <li className="mb-2">
              Download the {" "}
              <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
                pint-auction
              </code>{" "} zip (containing executable and contract abis) and unzip.
            </li>
            <li className="mb-2">
              After viewing detials about the item. use {" "}
              <code className="bg-black/[.05] dark:bg-white/[.06] px-1 py-0.5 rounded font-semibold">
                ./pint-auction place-bid &lt;ACCOUNT_NAME&gt; &lt;ITEM_ID&gt; &lt;AMOUNT&gt; <br></br> "https://bigbangblock.builders" "https://bigbangblock.builders" "./pint/token/"
              </code>
            </li>
            <li className="mb-2">
              If the constraints match i.e you have sufficient balance, the ownership of the item_id will be changed to <br></br>your address (the earliest bidder wins.).
            </li>
          </ol>
        </main>
        <footer className="row-start-3 flex gap-6 flex-wrap items-center justify-center">
          <a
            className="flex items-center gap-2 hover:underline hover:underline-offset-4"
            href="https://youtu.be/eB7SaMKbE9A"
            target="_blank"
            rel="noopener noreferrer"
          >
            <Image
              aria-hidden
              src="https://nextjs.org/icons/window.svg"
              alt="Window icon"
              width={16}
              height={16}
            />
            Demo
          </a>
          <a
            className="flex items-center gap-2 hover:underline hover:underline-offset-4"
            href="https://github.com/0xn4utilus/PintAuction"
            target="_blank"
            rel="noopener noreferrer"
          >
            <Image
              aria-hidden
              src="https://nextjs.org/icons/globe.svg"
              alt="Globe icon"
              width={16}
              height={16}
            />
            Go to Github Repo →
          </a>
        </footer>
      </div>
    </>
  );
}
