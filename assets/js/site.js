// Small enhancements. The site is fully readable without this file.
(() => {
  "use strict";

  const root = document.documentElement;
  const reduceMotion = matchMedia("(prefers-reduced-motion: reduce)").matches;

  // Light and dark theme. The first choice comes from the system setting
  // (applied by an inline script before first paint); a click overrides it.
  const toggle = document.querySelector("[data-theme-toggle]");
  if (toggle) {
    const sync = () =>
      toggle.setAttribute("aria-pressed", String(root.dataset.theme === "light"));
    sync();
    toggle.addEventListener("click", () => {
      root.dataset.theme = root.dataset.theme === "light" ? "dark" : "light";
      try {
        localStorage.setItem("theme", root.dataset.theme);
      } catch {
        // Private browsing: the choice just lasts for this page.
      }
      sync();
    });
  }

  // The navigation bar gets a background once the page has scrolled.
  const nav = document.querySelector("[data-nav]");
  if (nav) {
    const onScroll = () => nav.toggleAttribute("data-scrolled", scrollY > 8);
    onScroll();
    addEventListener("scroll", onScroll, { passive: true });
  }

  // Content rises into place the first time it scrolls into view.
  const reveals = document.querySelectorAll("[data-reveal]");
  if (reduceMotion || !("IntersectionObserver" in window)) {
    reveals.forEach((el) => el.classList.add("is-in"));
  } else {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            entry.target.classList.add("is-in");
            observer.unobserve(entry.target);
          }
        }
      },
      { rootMargin: "0px 0px -6% 0px", threshold: 0.06 },
    );
    reveals.forEach((el) => observer.observe(el));
  }

  // The glow behind the introduction follows the pointer.
  const glow = document.querySelector("[data-glow]");
  if (glow && !reduceMotion && matchMedia("(pointer: fine)").matches) {
    let frame = 0;
    glow.addEventListener("pointermove", (event) => {
      if (frame) return;
      frame = requestAnimationFrame(() => {
        frame = 0;
        const box = glow.getBoundingClientRect();
        glow.style.setProperty("--mx", `${event.clientX - box.left}px`);
        glow.style.setProperty("--my", `${event.clientY - box.top}px`);
      });
    });
  }

  // Gallery screenshots open full-size in a dialog. Without it, the links
  // simply open the image.
  const dialog = document.querySelector("[data-lightbox-dialog]");
  if (dialog && typeof dialog.showModal === "function") {
    const image = dialog.querySelector("img");
    const caption = dialog.querySelector(".lightbox__caption");
    document.querySelectorAll("a[data-lightbox]").forEach((link) => {
      link.addEventListener("click", (event) => {
        event.preventDefault();
        image.src = link.href;
        image.alt = link.querySelector("img")?.alt ?? "";
        caption.textContent = link.dataset.caption ?? "";
        dialog.showModal();
      });
    });
    dialog.addEventListener("click", (event) => {
      if (event.target === dialog) dialog.close();
    });
  }
})();
