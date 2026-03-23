// ===== Language Toggle =====
const translations = { en: {}, es: {} };
let currentLang = 'en';

function setLanguage(lang) {
  currentLang = lang;
  document.documentElement.setAttribute('data-lang', lang);

  document.querySelectorAll('[data-en]').forEach(el => {
    const text = el.getAttribute(`data-${lang}`);
    if (text) el.textContent = text;
  });

  document.querySelectorAll('.lang-option').forEach(opt => {
    opt.classList.toggle('active', opt.dataset.lang === lang);
  });

  startTyping();
}

document.querySelectorAll('.lang-toggle').forEach(btn => {
  btn.addEventListener('click', () => {
    setLanguage(currentLang === 'en' ? 'es' : 'en');
  });
});

// ===== Typing Effect =====
const phrases = {
  en: [
    'Go microservices in production',
    'Rust game engines & systems',
    'AI-augmented development',
    'Self-taught since age 12',
    'Retro MMORPG enthusiast',
  ],
  es: [
    'Microservicios Go en produccion',
    'Motores de juegos & sistemas en Rust',
    'Desarrollo asistido por IA',
    'Autodidacta desde los 12 anos',
    'Entusiasta de MMORPGs retro',
  ],
};

let typingTimeout;
function startTyping() {
  const el = document.getElementById('typedText');
  if (!el) return;

  clearTimeout(typingTimeout);
  el.textContent = '';

  const currentPhrases = phrases[currentLang];
  let phraseIndex = 0;
  let charIndex = 0;
  let isDeleting = false;

  function type() {
    const phrase = currentPhrases[phraseIndex];

    if (!isDeleting) {
      el.textContent = phrase.substring(0, charIndex + 1);
      charIndex++;
      if (charIndex === phrase.length) {
        isDeleting = true;
        typingTimeout = setTimeout(type, 2000);
        return;
      }
      typingTimeout = setTimeout(type, 50);
    } else {
      el.textContent = phrase.substring(0, charIndex - 1);
      charIndex--;
      if (charIndex === 0) {
        isDeleting = false;
        phraseIndex = (phraseIndex + 1) % currentPhrases.length;
        typingTimeout = setTimeout(type, 400);
        return;
      }
      typingTimeout = setTimeout(type, 30);
    }
  }

  typingTimeout = setTimeout(type, 600);
}

// ===== Navbar scroll effect =====
const nav = document.getElementById('nav');
window.addEventListener('scroll', () => {
  nav.classList.toggle('scrolled', window.scrollY > 50);
});

// ===== Mobile menu =====
const hamburger = document.getElementById('hamburger');
const mobileMenu = document.getElementById('mobileMenu');

hamburger.addEventListener('click', () => {
  hamburger.classList.toggle('active');
  mobileMenu.classList.toggle('open');
  document.body.style.overflow = mobileMenu.classList.contains('open') ? 'hidden' : '';
});

mobileMenu.querySelectorAll('a').forEach(link => {
  link.addEventListener('click', () => {
    hamburger.classList.remove('active');
    mobileMenu.classList.remove('open');
    document.body.style.overflow = '';
  });
});

// ===== Scroll animations =====
function initScrollAnimations() {
  const elements = document.querySelectorAll(
    '.about-text p, .highlight-card, .timeline-item, .project-card, .skill-category, .contact-card'
  );
  elements.forEach(el => el.classList.add('fade-in'));

  const observer = new IntersectionObserver(
    entries => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          entry.target.classList.add('visible');
        }
      });
    },
    { threshold: 0.1, rootMargin: '0px 0px -40px 0px' }
  );

  elements.forEach(el => observer.observe(el));
}

// ===== Skill bar animation =====
function initSkillBars() {
  const observer = new IntersectionObserver(
    entries => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          const fills = entry.target.querySelectorAll('.skill-bar-fill');
          fills.forEach(fill => {
            fill.style.width = fill.style.width; // trigger reflow
          });
        }
      });
    },
    { threshold: 0.3 }
  );

  document.querySelectorAll('.skill-bars').forEach(el => observer.observe(el));
}

// ===== Init =====
document.addEventListener('DOMContentLoaded', () => {
  startTyping();
  initScrollAnimations();
  initSkillBars();
});
