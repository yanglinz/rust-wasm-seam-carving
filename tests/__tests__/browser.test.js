require("expect-puppeteer");

test("puppeteer test", async () => {
  await page.goto("https://google.com");
  const evaluatedTitle = await page.evaluate(function () {
    return document.title;
  });
  expect(evaluatedTitle).toEqual("Google");
});
