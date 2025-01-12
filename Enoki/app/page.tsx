"use client";

import Image from "next/image";
import { JSX } from "react";
import {
  DoesNetworkTableHandlerExist,
  NetworkTableHandlerId,
  StartNetworkTableHandler,
  TableEntry,
} from "@/utilities/NT4Handler";
import { invoke } from "@tauri-apps/api/tauri";
import { window } from "@tauri-apps/api";
import { TauriEvent } from "@tauri-apps/api/event";
import NetworkTable from "@/app/components/network_table";

export default function Home(): JSX.Element {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          Get started by opening&nbsp;
          <code className="font-mono font-bold">docs/developing.md</code>
        </p>
        <div className="fixed bottom-0 left-0 flex h-48 w-full items-end justify-center bg-gradient-to-t from-white via-white dark:from-black dark:via-black lg:static lg:h-auto lg:w-auto lg:bg-none">
          <a
            className="pointer-events-none flex place-items-center gap-2 p-8 lg:pointer-events-auto lg:p-0"
            href="https://vercel.com?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            By{" "}
            <Image
              src="/vercel.svg"
              alt="Vercel Logo"
              className="dark:invert"
              width={100}
              height={24}
              priority
            />
          </a>
        </div>
      </div>

      <div className="relative flex place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 before:lg:h-[360px]">
        <Image
          className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
          src="/next.svg"
          alt="Next.js Logo"
          width={180}
          height={37}
          priority
        />
      </div>

      <NetworkTable />

      <div className="mb-32 grid text-center lg:mb-0 lg:grid-cols-3 lg:text-left">
        <button
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          onClick={StartNTHandler}
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Connect{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Click here to connect to the network tables server on localhost:5810
          </p>
        </button>

        <button
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          onClick={StopNT4Handler}
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Disconnect{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Click here to disconnect from the network tables server on
            localhost:5810
          </p>
        </button>

        <button
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          onClick={SubscribeExample}
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Subscribe{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Click here to subscribe to a value on the network tables server on
            localhost:5810
          </p>
        </button>

        <button
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          onClick={PublishExample}
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Publish{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Click here to subscribe to a publish on the network tables server on
            localhost:5810
          </p>
        </button>

        <button
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          onClick={DoesHandlerExist}
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Is Connected?{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Click here to check if a handler exists for the network tables
            server on localhost:5810
          </p>
        </button>

        <button
          className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
          onClick={PollSubscriptions}
        >
          <h2 className={`mb-3 text-2xl font-semibold`}>
            Get Subbed{" "}
            <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
              -&gt;
            </span>
          </h2>
          <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
            Click here to poll all subbed data on the network tables server on
            localhost:5810
          </p>
        </button>
      </div>
    </main>
  );
}

//create a test table variable
let testTable: NetworkTableHandlerId;

function StartNTHandler(): void {
  console.log("Starting NetworkTables");
  testTable = StartNetworkTableHandler([74, 65, 89, 147], 5800, "Enoki-test");
}
function StopNT4Handler(): void {
  console.log("Stopping NetworkTables");
  testTable.stopNetworkTableHandler();
}

function DoesHandlerExist(): void {
  DoesNetworkTableHandlerExist(testTable).then((result: boolean) =>
    console.log(result)
  );
}

function SubscribeExample(): void {
  console.log("Subscribing to NetworkTables");
  testTable.subscribe("", 0.05, true, true);
}

function PublishExample(): void {
  console.log("Publishing to NetworkTables");
  testTable.setEntry("/test", 1);
}

async function PollSubscriptions(): Promise<void> {
  console.log("Polling Subscriptions");
  let entries: TableEntry[] = await testTable.getEntries();
  console.log(entries);
}
