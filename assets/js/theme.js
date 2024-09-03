function applyColorSchemePreference() {
  if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }
}

function setColorSchemePreference(preference) {
  localStorage.theme = preference;
}

function unsetColorSchemePreference() {
  localStorage.removeItem('theme');
}

window.addEventListener('load', () => applyColorSchemePreference());
