const { firefox } = require('playwright');

(async () => {
  const browser = await firefox.launch({ headless: false });
  const context = await browser.newContext();
  const page = await context.newPage();

  await page.goto('https://www.instagram.com/direct/inbox/');
  console.log("Page loaded. Waiting for user login...");
  await page.waitForTimeout(15000); // Let user log in manually

  // Pipe browser console output to terminal
  page.on('console', msg => {
    const text = msg.text();
    if (text.includes("Mutation")) {
      console.log(`[browser]: ${text}`);
    }
  });

  // Inject the MutationObserver
  await page.evaluate(() => {
    const observer = new MutationObserver(() => {
      console.log("Mutation fired!"); // triggers Node fetch
    });

    observer.observe(document.body, { childList: true, subtree: true });
  });

  page.on('console', msg => {
    const text = msg.text();
    if (text.includes("Mutation fired!")) {
      fetch("http://localhost:3000/notify", { method: "POST" });
    }
  });


    console.log("MutationObserver running...");
  })();
