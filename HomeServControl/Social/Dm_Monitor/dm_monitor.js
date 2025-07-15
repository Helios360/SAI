const { firefox } = require('playwright');

(async () => {
  const browser = await firefox.launch({
    headless: true, // important for xvfb
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  const page = await browser.newPage();

  // Print mutations to stdout
  await page.exposeFunction('onMutation', (mutation) => {
    console.log('Mutation detected:', JSON.stringify(mutation));
  });

  // Replace with your target URL
  const url = process.argv[2] || 'https://instagram.com';

  await page.goto(url, { waitUntil: 'load' });

  // Inject the MutationObserver into the page context
  await page.evaluate(() => {
    const observer = new MutationObserver((mutationsList) => {
      for (const mutation of mutationsList) {
        window.onMutation({
          type: mutation.type,
          target: mutation.target.outerHTML,
          addedNodes: Array.from(mutation.addedNodes).map(node => node.outerHTML || node.textContent),
          removedNodes: Array.from(mutation.removedNodes).map(node => node.outerHTML || node.textContent),
          attributeName: mutation.attributeName,
        });
      }
    });

    observer.observe(document.body, {
      attributes: true,
      childList: true,
      subtree: true
    });
  });

  // Keep the process alive
  console.log(`Observing mutations on ${url}... Press Ctrl+C to exit.`);
})();
