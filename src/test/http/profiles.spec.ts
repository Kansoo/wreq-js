import assert from "node:assert";
import { describe, test } from "node:test";
import type { BrowserProfile } from "../../wreq-js";
import { getProfiles, RequestError, fetch as wreqFetch } from "../../wreq-js";
import { httpUrl } from "../helpers/http";

describe("HTTP profiles", () => {
  test("returns available browser profiles", () => {
    const profiles = getProfiles();

    assert.ok(Array.isArray(profiles), "Profiles should be an array");
    assert.ok(profiles.length > 0, "Should have at least one profile");
    assert.ok(
      profiles.some((p) => p.includes("chrome")) ||
        profiles.some((p) => p.includes("firefox")) ||
        profiles.some((p) => p.includes("safari")),
      "Should include standard browser profiles",
    );
  });

  test("rejects invalid browser profiles", async () => {
    await assert.rejects(
      async () => {
        await wreqFetch(httpUrl("/get"), {
          browser: "nonexistent_browser" as BrowserProfile,
          timeout: 1000,
        });
      },
      (error: unknown) => error instanceof RequestError,
      "Should reject invalid browser profiles",
    );
  });
});
