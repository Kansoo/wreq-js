export const HTTP_TEST_BASE_URL = process.env.HTTP_TEST_BASE_URL ?? "https://httpbingo.org";

export const httpUrl = (path: string) => new URL(path, HTTP_TEST_BASE_URL).toString();

export function headerIndex(rawHeaders: string[], name: string) {
  return rawHeaders.findIndex((value, index) => index % 2 === 0 && value.toLowerCase() === name.toLowerCase());
}
