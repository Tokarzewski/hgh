/* Shared site behaviour: PL/EN language toggle, scroll reveal, active nav.
   i18n is attribute-based: any element with data-en / data-pl gets its text
   swapped. Placeholder text uses data-en-ph / data-pl-ph. */

(function () {
  const KEY = 'sg-lang';
  const langs = ['en', 'pl'];

  function applyLang(lang) {
    if (!langs.includes(lang)) lang = 'en';
    document.documentElement.lang = lang;
    try { localStorage.setItem(KEY, lang); } catch (e) {}

    document.querySelectorAll('[data-en]').forEach((el) => {
      const v = el.getAttribute('data-' + lang);
      if (v != null) el.textContent = v;
    });
    document.querySelectorAll('[data-en-ph]').forEach((el) => {
      const v = el.getAttribute('data-' + lang + '-ph');
      if (v != null) el.setAttribute('placeholder', v);
    });
    document.querySelectorAll('.lang-current').forEach((el) => {
      el.textContent = lang.toUpperCase();
    });
  }

  function initLang() {
    let saved = 'en';
    try { saved = localStorage.getItem(KEY) || 'en'; } catch (e) {}
    applyLang(saved);
    document.querySelectorAll('[data-lang-toggle]').forEach((btn) => {
      btn.addEventListener('click', () => {
        const cur = document.documentElement.lang || 'en';
        applyLang(cur === 'en' ? 'pl' : 'en');
      });
    });
  }

  function initReveal() {
    const els = document.querySelectorAll('.reveal');
    if (!('IntersectionObserver' in window) || !els.length) {
      els.forEach((el) => el.classList.add('in'));
      return;
    }
    const io = new IntersectionObserver((entries) => {
      entries.forEach((e) => {
        if (e.isIntersecting) { e.target.classList.add('in'); io.unobserve(e.target); }
      });
    }, { threshold: 0.12 });
    els.forEach((el) => io.observe(el));
  }

  function initActiveNav() {
    const here = location.pathname.replace(/index\.html$/, '').replace(/\/$/, '') || '/';
    document.querySelectorAll('.topbar nav a').forEach((a) => {
      const path = new URL(a.href, location.origin).pathname
        .replace(/index\.html$/, '').replace(/\/$/, '') || '/';
      if (path === here) a.setAttribute('aria-current', 'page');
    });
  }

  document.addEventListener('DOMContentLoaded', () => {
    initLang();
    initReveal();
    initActiveNav();
  });
})();
