import { spawn } from "node:child_process";
import { existsSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import process from "node:process";
import type { LocalTestServer } from "./helpers/local-test-server";
import { startLocalTestServer } from "./helpers/local-test-server";

const testDir = __dirname;
const httpTestDir = resolve(testDir, "http");
const httpTestFiles = existsSync(httpTestDir)
  ? readdirSync(httpTestDir)
      .filter((filename) => filename.endsWith(".spec.js"))
      .map((filename) => resolve(httpTestDir, filename))
      .sort()
  : [];

async function main() {
  const extraArgs = process.argv.slice(2);
  const defaultTestFiles = [...httpTestFiles, resolve(testDir, "websocket.spec.js")];

  const env = { ...process.env };

  let localServer: LocalTestServer | undefined;
  if (!env.HTTP_TEST_BASE_URL || !env.WS_TEST_URL) {
    localServer = await startLocalTestServer();
    if (!env.HTTP_TEST_BASE_URL) {
      env.HTTP_TEST_BASE_URL = localServer.httpBaseUrl;
    }
    if (!env.WS_TEST_URL) {
      env.WS_TEST_URL = localServer.wsUrl;
    }
  }

  const nodeArgs = ["--test", ...defaultTestFiles, ...extraArgs];
  const testProcess = spawn(process.execPath, nodeArgs, {
    stdio: "inherit",
    env,
  });

  const cleanup = async () => {
    if (!localServer) {
      return;
    }
    const server = localServer;
    localServer = undefined;
    try {
      await server.close();
    } catch (error) {
      console.error("Failed to stop local test server:", error);
    }
  };

  testProcess.once("exit", async (code, signal) => {
    await cleanup();
    if (signal) {
      process.kill(process.pid, signal);
      return;
    }
    process.exit(code ?? 1);
  });

  testProcess.once("error", async (error) => {
    console.error("Failed to run tests:", error);
    await cleanup();
    process.exit(1);
  });
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
