/* ═══════════════════════════════════════════════════════════
   RustCluster — Frontend SPA Application
   Vercel-like dashboard for managing bots & apps
   All icons are inline SVGs — no emojis
   ═══════════════════════════════════════════════════════════ */

// ── SVG Icon Library ───────────────────────────────────────
const icons = {
    bolt: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>`,
    dashboard: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>`,
    plus: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>`,
    settings: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>`,
    play: `<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" stroke="none"><polygon points="5 3 19 12 5 21 5 3"/></svg>`,
    stop: `<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" stroke="none"><rect x="4" y="4" width="16" height="16" rx="2"/></svg>`,
    refresh: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"/><polyline points="1 20 1 14 7 14"/><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/></svg>`,
    rocket: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"/><path d="M12 15l-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>`,
    github: `<svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>`,
    box: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>`,
    terminal: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 17 10 11 4 5"/><line x1="12" y1="19" x2="20" y2="19"/></svg>`,
    link: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>`,
    trash: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>`,
    search: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>`,
    x: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>`,
    check: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>`,
    info: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>`,
    logout: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg>`,
    clock: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>`,
    server: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="2" width="20" height="8" rx="2" ry="2"/><rect x="2" y="14" width="20" height="8" rx="2" ry="2"/><line x1="6" y1="6" x2="6.01" y2="6"/><line x1="6" y1="18" x2="6.01" y2="18"/></svg>`,
    cpu: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="4" width="16" height="16" rx="2" ry="2"/><rect x="9" y="9" width="6" height="6"/><line x1="9" y1="1" x2="9" y2="4"/><line x1="15" y1="1" x2="15" y2="4"/><line x1="9" y1="20" x2="9" y2="23"/><line x1="15" y1="20" x2="15" y2="23"/><line x1="20" y1="9" x2="23" y2="9"/><line x1="20" y1="14" x2="23" y2="14"/><line x1="1" y1="9" x2="4" y2="9"/><line x1="1" y1="14" x2="4" y2="14"/></svg>`,
    deploy: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" y1="19" x2="12" y2="5"/><polyline points="5 12 12 5 19 12"/></svg>`,
    externalLink: `<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>`,
    arrowRight: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>`,
    lock: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>`,
    git: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><path d="M13 6h3a2 2 0 0 1 2 2v7"/><line x1="6" y1="9" x2="6" y2="21"/></svg>`,
    layers: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>`,
    activity: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/></svg>`,
    node: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 6v12M6 12h12"/></svg>`,
    python: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2C6.48 2 2 4.02 2 6.5V10c0 2.48 4.48 4.5 10 4.5s10-2.02 10-4.5V6.5C22 4.02 17.52 2 12 2z"/><path d="M2 14v3.5C2 19.98 6.48 22 12 22s10-2.02 10-4.5V14"/></svg>`,
    rust: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="4"/></svg>`,
    sync: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/></svg>`,
    list: `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="8" y1="6" x2="21" y2="6"/><line x1="8" y1="12" x2="21" y2="12"/><line x1="8" y1="18" x2="21" y2="18"/><line x1="3" y1="6" x2="3.01" y2="6"/><line x1="3" y1="12" x2="3.01" y2="12"/><line x1="3" y1="18" x2="3.01" y2="18"/></svg>`,
    memory: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="6" width="20" height="12" rx="2"/><line x1="6" y1="10" x2="6" y2="14"/><line x1="10" y1="10" x2="10" y2="14"/><line x1="14" y1="10" x2="14" y2="14"/><line x1="18" y1="10" x2="18" y2="14"/></svg>`,
    checkCircle: `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>`,
};

function icon(name, size) {
    const svg = icons[name] || icons.box;
    if (size) return svg.replace(/width="\d+"/, `width="${size}"`).replace(/height="\d+"/, `height="${size}"`);
    return svg;
}

// ── State ──────────────────────────────────────────────────
const state = {
    token: localStorage.getItem('rc_token') || '',
    user: JSON.parse(localStorage.getItem('rc_user') || 'null'),
    projects: [],
    stats: null,
    currentProject: null,
    currentDeployments: [],
    githubStatus: null,
    githubRepos: [],
    pollInterval: null,
};

// ── API Client ─────────────────────────────────────────────
async function api(path, opts = {}) {
    const headers = { 'Content-Type': 'application/json' };
    if (state.token) headers['Authorization'] = `Bearer ${state.token}`;
    const res = await fetch(`/api${path}`, { ...opts, headers: { ...headers, ...opts.headers } });
    if (res.status === 401) { logout(); throw new Error('Unauthorized'); }
    const data = await res.json().catch(() => ({}));
    if (!res.ok) throw new Error(data.message || `HTTP ${res.status}`);
    return data;
}

// ── Toast System ───────────────────────────────────────────
function toast(message, type = 'info') {
    const container = document.getElementById('toast-container');
    const el = document.createElement('div');
    const toastIcons = { success: icon('check'), error: icon('x'), info: icon('info') };
    el.className = `toast toast-${type}`;
    el.innerHTML = `<span style="display:flex">${toastIcons[type] || toastIcons.info}</span><span>${message}</span>`;
    container.appendChild(el);
    setTimeout(() => { el.classList.add('toast-exit'); setTimeout(() => el.remove(), 300); }, 4000);
}

// ── Router ─────────────────────────────────────────────────
function navigate(hash) { window.location.hash = hash; }
function getRoute() {
    const hash = window.location.hash.slice(1) || '/';
    const parts = hash.split('/').filter(Boolean);
    return { path: hash, parts };
}
window.addEventListener('hashchange', () => render());

// ── Auth ───────────────────────────────────────────────────
async function login(email, password) {
    const data = await api('/auth/login', {
        method: 'POST', body: JSON.stringify({ email, password })
    });
    state.token = data.token;
    state.user = { username: data.username, email: data.email };
    localStorage.setItem('rc_token', data.token);
    localStorage.setItem('rc_user', JSON.stringify(state.user));
    navigate('/');
}

function logout() {
    state.token = ''; state.user = null;
    localStorage.removeItem('rc_token');
    localStorage.removeItem('rc_user');
    if (state.pollInterval) clearInterval(state.pollInterval);
    navigate('/login');
}

function isLoggedIn() { return !!state.token && !!state.user; }

// ── Data Fetching ──────────────────────────────────────────
async function fetchProjects() {
    try { state.projects = await api('/projects'); } catch(e) { state.projects = []; }
}
async function fetchStats() {
    try { state.stats = await api('/system/stats'); } catch(e) { state.stats = null; }
}
async function fetchProject(id) {
    try { state.currentProject = await api(`/projects/${id}`); } catch(e) { state.currentProject = null; }
}
async function fetchDeployments(id) {
    try {
        const d = await api(`/projects/${id}/deployments`);
        state.currentDeployments = d.deployments || [];
    } catch(e) { state.currentDeployments = []; }
}
async function fetchGitHubStatus() {
    try { state.githubStatus = await api('/github/setup/status'); } catch(e) { state.githubStatus = null; }
}
async function fetchGitHubRepos() {
    try {
        const d = await api('/github/repos');
        state.githubRepos = d.repos || [];
    } catch(e) { state.githubRepos = []; }
}

// ── Render Engine ──────────────────────────────────────────
async function render() {
    const app = document.getElementById('app');
    const route = getRoute();

    if (!isLoggedIn() && route.path !== '/login') { navigate('/login'); return; }
    if (isLoggedIn() && route.path === '/login') { navigate('/'); return; }
    if (route.path === '/login') { app.innerHTML = renderLoginPage(); bindLoginEvents(); return; }

    if (!state.projects.length) await fetchProjects();

    let pageContent = '';
    let activeNav = 'dashboard';

    if (route.parts[0] === 'projects' && route.parts[1] === 'new') {
        pageContent = await renderNewProjectPage();
        activeNav = 'new';
    } else if (route.parts[0] === 'projects' && route.parts[1]) {
        const projectId = route.parts[1];
        await fetchProject(projectId);
        await fetchDeployments(projectId);
        const tab = route.parts[2] || 'overview';
        pageContent = renderProjectDetailPage(tab);
        activeNav = 'projects';
    } else if (route.parts[0] === 'settings') {
        await fetchGitHubStatus();
        pageContent = renderSettingsPage(route.parts[1] || 'github');
        activeNav = 'settings';
    } else {
        await fetchStats();
        pageContent = renderDashboardPage();
        activeNav = 'dashboard';
    }

    const running = state.projects.filter(p => p.status === 'running').length;
    app.innerHTML = `
        <div class="layout">
            ${renderSidebar(activeNav, running)}
            <div class="main-content">
                <div class="page-container">${pageContent}</div>
            </div>
        </div>`;

    bindPageEvents();
    startPolling();
}

// ── Sidebar ────────────────────────────────────────────────
function renderSidebar(active, runningCount) {
    const initial = state.user?.username?.[0]?.toUpperCase() || 'A';
    return `
    <aside class="sidebar">
        <div class="sidebar-brand">
            <div class="logo">${icon('bolt', 18)}</div>
            <span class="brand-name">RustCluster</span>
            <span class="brand-tag">v0.1</span>
        </div>
        <nav class="sidebar-nav">
            <div class="nav-section">
                <div class="nav-section-title">Overview</div>
                <div class="nav-item ${active==='dashboard'?'active':''}" onclick="navigate('/')">
                    <span class="nav-icon">${icon('dashboard')}</span> Dashboard
                </div>
            </div>
            <div class="nav-section">
                <div class="nav-section-title">Projects</div>
                <div class="nav-item ${active==='new'?'active':''}" onclick="navigate('/projects/new')">
                    <span class="nav-icon">${icon('plus')}</span> New Project
                </div>
                ${state.projects.slice(0, 8).map(p => `
                    <div class="nav-item ${active==='projects' && state.currentProject?.project?.id===p.id ? 'active':''}"
                         onclick="navigate('/projects/${p.id}')">
                        <span class="nav-icon" style="color:${p.status==='running' ? 'var(--success)' : 'var(--text-tertiary)'}">${icon('server')}</span>
                        <span style="flex:1;overflow:hidden;text-overflow:ellipsis">${p.name}</span>
                        ${p.status==='running'?'<span class="badge">Live</span>':''}
                    </div>`).join('')}
            </div>
            <div class="nav-section">
                <div class="nav-section-title">Configure</div>
                <div class="nav-item ${active==='settings'?'active':''}" onclick="navigate('/settings/github')">
                    <span class="nav-icon">${icon('settings')}</span> Settings
                </div>
            </div>
        </nav>
        <div class="sidebar-footer">
            <div class="sidebar-user" onclick="logout()">
                <div class="user-avatar">${initial}</div>
                <div class="user-info">
                    <div class="user-name">${state.user?.username || 'Admin'}</div>
                    <div class="user-email">${state.user?.email || ''}</div>
                </div>
                <span style="color:var(--text-tertiary);display:flex" title="Logout">${icon('logout')}</span>
            </div>
        </div>
    </aside>`;
}

// ── Login Page ─────────────────────────────────────────────
function renderLoginPage() {
    return `
    <div class="login-page">
        <div class="login-card">
            <div class="login-logo">${icon('bolt', 24)}</div>
            <h1 class="login-title">RustCluster</h1>
            <p class="login-subtitle">Sign in to your deployment dashboard</p>
            <div class="login-error" id="login-error"></div>
            <form id="login-form">
                <div class="form-group">
                    <label class="form-label">Email</label>
                    <input type="email" class="form-input" id="login-email" placeholder="admin@example.com" required>
                </div>
                <div class="form-group">
                    <label class="form-label">Password</label>
                    <input type="password" class="form-input" id="login-password" placeholder="Enter password" required>
                </div>
                <button type="submit" class="btn btn-primary w-full btn-lg" id="login-btn">Sign In</button>
            </form>
        </div>
    </div>`;
}

function bindLoginEvents() {
    const form = document.getElementById('login-form');
    if (!form) return;
    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        const btn = document.getElementById('login-btn');
        const errEl = document.getElementById('login-error');
        btn.disabled = true; btn.textContent = 'Signing in...';
        errEl.style.display = 'none';
        try {
            await login(
                document.getElementById('login-email').value,
                document.getElementById('login-password').value
            );
        } catch (err) {
            errEl.textContent = err.message || 'Invalid credentials';
            errEl.style.display = 'block';
            btn.disabled = false; btn.textContent = 'Sign In';
        }
    });
}

// ── Dashboard Page ─────────────────────────────────────────
function renderDashboardPage() {
    const s = state.stats || {};
    const memGB = (b) => (b / 1073741824).toFixed(1);
    return `
    <div class="page-header">
        <h1>Dashboard</h1>
        <p>Overview of all your deployed projects</p>
    </div>
    <div class="stats-grid">
        <div class="stat-card">
            <div class="stat-label">${icon('layers')} Total Projects</div>
            <div class="stat-value">${s.total_projects ?? state.projects.length}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">${icon('activity')} Running</div>
            <div class="stat-value text-success">${s.running_projects ?? state.projects.filter(p=>p.status==='running').length}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">${icon('deploy')} Deployments</div>
            <div class="stat-value">${s.total_deployments ?? 0}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">${icon('memory')} Memory</div>
            <div class="stat-value">${s.memory_percent ? s.memory_percent.toFixed(0) + '%' : '--'}</div>
            <div class="stat-sub">${s.memory_used ? memGB(s.memory_used) + ' / ' + memGB(s.memory_total) + ' GB' : ''}</div>
        </div>
    </div>

    <div class="flex items-center justify-between mb-6">
        <h2>Projects</h2>
        <button class="btn btn-primary" onclick="navigate('/projects/new')">${icon('plus')} New Project</button>
    </div>

    ${state.projects.length === 0 ? `
        <div class="empty-state">
            <div class="empty-icon">${icon('box', 48)}</div>
            <div class="empty-title">No projects yet</div>
            <div class="empty-desc">Create your first project to deploy a Telegram bot or app.</div>
            <button class="btn btn-primary btn-lg" onclick="navigate('/projects/new')">Create Project</button>
        </div>
    ` : `
        <div class="projects-grid">
            ${state.projects.map(p => `
                <div class="project-card" onclick="navigate('/projects/${p.id}')">
                    <div class="project-card-header">
                        <div>
                            <div class="project-name">
                                ${p.name}
                                <span class="status-badge status-${p.status}">
                                    <span class="status-dot"></span> ${p.status}
                                </span>
                            </div>
                            <div class="project-desc">${p.description || 'No description'}</div>
                        </div>
                    </div>
                    <div class="project-card-footer">
                        <span class="runtime-badge">${getRuntimeIcon(p.runtime)} ${p.runtime}</span>
                        <span>${icon('clock')} ${timeAgo(p.updated_at)}</span>
                    </div>
                </div>`).join('')}
        </div>`}`;
}

// ── New Project Page ───────────────────────────────────────
async function renderNewProjectPage() {
    await fetchGitHubStatus();
    const hasGH = state.githubStatus?.configured;
    if (hasGH) await fetchGitHubRepos();

    return `
    <div class="page-header">
        <h1>New Project</h1>
        <p>Deploy a new Telegram bot or application</p>
    </div>
    <div class="card">
        <div class="card-body">
            <form id="new-project-form">
                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">Project Name *</label>
                        <input class="form-input" id="np-name" placeholder="my-telegram-bot" required>
                    </div>
                    <div class="form-group">
                        <label class="form-label">Runtime</label>
                        <select class="form-select" id="np-runtime">
                            <option value="node">Node.js</option>
                            <option value="python">Python</option>
                            <option value="rust">Rust</option>
                            <option value="go">Go</option>
                            <option value="other">Other</option>
                        </select>
                    </div>
                </div>
                <div class="form-group">
                    <label class="form-label">Description</label>
                    <input class="form-input" id="np-desc" placeholder="My awesome Telegram bot">
                </div>

                ${hasGH ? `
                <div class="form-group">
                    <label class="form-label">${icon('github')} GitHub Repository</label>
                    <div class="search-input-wrapper">
                        <span class="search-icon">${icon('search')}</span>
                        <input class="form-input" id="np-repo-search" placeholder="Search repositories...">
                    </div>
                    <div class="repo-list" id="repo-list">
                        ${state.githubRepos.map(r => `
                            <div class="repo-item" data-clone="${r.clone_url}" data-branch="${r.default_branch}" onclick="selectRepo(this)">
                                <div>
                                    <div class="repo-name">${r.full_name}</div>
                                    <div class="repo-meta">${icon('git')} ${r.default_branch} ${r.private ? '<span class="repo-private">' + icon('lock') + ' Private</span>' : ''}</div>
                                </div>
                                <span style="color:var(--text-tertiary);display:flex">${icon('arrowRight')}</span>
                            </div>`).join('')}
                        ${state.githubRepos.length === 0 ? '<p class="text-muted text-sm" style="padding:12px">No repos found. Install the GitHub App on your repos first.</p>' : ''}
                    </div>
                    <input type="hidden" id="np-repo" value="">
                </div>` : `
                <div class="form-group">
                    <label class="form-label">Repository URL (optional)</label>
                    <input class="form-input" id="np-repo" placeholder="https://github.com/user/repo">
                    <div class="form-hint">Or <a href="#/settings/github">connect GitHub</a> to browse your repos</div>
                </div>`}

                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">Branch</label>
                        <input class="form-input" id="np-branch" value="main" placeholder="main">
                    </div>
                    <div class="form-group">
                        <label class="form-label">Build Command</label>
                        <input class="form-input" id="np-build" placeholder="npm install && npm run build">
                    </div>
                </div>
                <div class="form-group">
                    <label class="form-label">Start Command *</label>
                    <input class="form-input" id="np-start" placeholder="npm start" required>
                    <div class="form-hint">The command to start your bot/app</div>
                </div>

                <div class="form-group">
                    <label class="form-label">Environment Variables</label>
                    <div id="env-editor">
                        <div class="env-row">
                            <input class="form-input" placeholder="KEY" data-env-key>
                            <input class="form-input" placeholder="VALUE" data-env-val>
                            <button type="button" class="btn btn-ghost btn-icon" onclick="this.closest('.env-row').remove()">${icon('x')}</button>
                        </div>
                    </div>
                    <button type="button" class="btn btn-ghost btn-sm mt-4" onclick="addEnvRow()">${icon('plus')} Add Variable</button>
                </div>

                <div class="flex items-center gap-3">
                    <label style="display:flex;align-items:center;gap:8px;cursor:pointer">
                        <input type="checkbox" id="np-autodeploy"> Auto-deploy on push
                    </label>
                </div>

                <div class="flex justify-between mt-6">
                    <button type="button" class="btn btn-secondary" onclick="navigate('/')">Cancel</button>
                    <button type="submit" class="btn btn-primary" id="np-submit">${icon('plus')} Create Project</button>
                </div>
            </form>
        </div>
    </div>`;
}

// ── Project Detail Page ────────────────────────────────────
function renderProjectDetailPage(tab) {
    const p = state.currentProject?.project;
    const proc = state.currentProject?.process;
    if (!p) return '<div class="empty-state"><div class="empty-icon">' + icon('x', 48) + '</div><div class="empty-title">Project not found</div></div>';

    return `
    <div class="detail-header">
        <div class="detail-header-left">
            <div class="detail-icon">${getRuntimeIcon(p.runtime, 22)}</div>
            <div>
                <h1 style="margin-bottom:4px">${p.name}</h1>
                <div class="flex items-center gap-3">
                    <span class="status-badge status-${p.status}"><span class="status-dot"></span> ${p.status}</span>
                    <span class="runtime-badge">${p.runtime}</span>
                    ${proc ? `<span class="text-xs text-muted">${icon('clock')} ${formatUptime(proc.uptime_seconds)}</span>` : ''}
                </div>
            </div>
        </div>
        <div class="detail-actions">
            ${p.status === 'running' ? `
                <button class="btn btn-secondary btn-sm" onclick="projectAction('${p.id}','restart')">${icon('refresh')} Restart</button>
                <button class="btn btn-danger btn-sm" onclick="projectAction('${p.id}','stop')">${icon('stop')} Stop</button>
            ` : `
                <button class="btn btn-success btn-sm" onclick="projectAction('${p.id}','start')">${icon('play')} Start</button>
            `}
            <button class="btn btn-primary btn-sm" onclick="projectAction('${p.id}','deploy')">${icon('rocket')} Deploy</button>
        </div>
    </div>

    <div class="tabs">
        <button class="tab ${tab==='overview'?'active':''}" onclick="navigate('/projects/${p.id}')">Overview</button>
        <button class="tab ${tab==='deployments'?'active':''}" onclick="navigate('/projects/${p.id}/deployments')">Deployments</button>
        <button class="tab ${tab==='env'?'active':''}" onclick="navigate('/projects/${p.id}/env')">Variables</button>
        <button class="tab ${tab==='settings'?'active':''}" onclick="navigate('/projects/${p.id}/settings')">Settings</button>
    </div>

    ${tab === 'overview' ? renderProjectOverview(p, proc) : ''}
    ${tab === 'deployments' ? renderDeploymentsTab(p) : ''}
    ${tab === 'env' ? renderEnvTab(p) : ''}
    ${tab === 'settings' ? renderSettingsTab(p) : ''}`;
}

function renderProjectOverview(p, proc) {
    const latest = state.currentDeployments[0];
    return `
    <div style="display:grid;grid-template-columns:1fr 1fr;gap:var(--space-5)">
        <div class="card">
            <div class="card-header"><h3>${icon('settings')} Configuration</h3></div>
            <div class="card-body">
                <div class="mb-4"><span class="text-muted text-sm">Repository</span><br><code>${p.repo_url || '--'}</code></div>
                <div class="mb-4"><span class="text-muted text-sm">Branch</span><br><code>${p.branch}</code></div>
                <div class="mb-4"><span class="text-muted text-sm">Build Command</span><br><code>${p.build_command || '--'}</code></div>
                <div><span class="text-muted text-sm">Start Command</span><br><code>${p.start_command}</code></div>
            </div>
        </div>
        <div class="card">
            <div class="card-header"><h3>${icon('rocket')} Latest Deployment</h3></div>
            <div class="card-body">
                ${latest ? `
                    <div class="mb-4">
                        <span class="status-badge status-${latest.status === 'success' ? 'running' : latest.status === 'failed' ? 'crashed' : 'building'}">
                            <span class="status-dot"></span> ${latest.status}
                        </span>
                    </div>
                    <div class="mb-4"><span class="text-muted text-sm">Commit</span><br><code>${latest.commit_sha || '--'}</code></div>
                    <div class="mb-4"><span class="text-muted text-sm">Message</span><br>${latest.commit_message || '--'}</div>
                    <div><span class="text-muted text-sm">Triggered</span><br>${latest.trigger} &middot; ${timeAgo(latest.created_at)}</div>
                ` : '<p class="text-muted">No deployments yet</p>'}
            </div>
        </div>
    </div>
    ${latest?.logs ? `
    <div class="mt-6">
        <h3 class="mb-4">${icon('terminal')} Build Logs</h3>
        <div class="terminal">
            <div class="terminal-header">
                <div class="terminal-dots"><span class="dot-red"></span><span class="dot-yellow"></span><span class="dot-green"></span></div>
                <span class="terminal-title">deploy -- ${latest.commit_sha || 'latest'}</span>
            </div>
            <div class="terminal-body">${colorizeLogs(latest.logs)}</div>
        </div>
    </div>` : ''}`;
}

function renderDeploymentsTab(p) {
    if (!state.currentDeployments.length) {
        return '<div class="empty-state"><div class="empty-icon">' + icon('list', 48) + '</div><div class="empty-title">No deployments</div><div class="empty-desc">Deploy your project to see the history here.</div></div>';
    }
    return `
    <div>
        ${state.currentDeployments.map(d => `
            <div class="deployment-item" onclick="showDeployLogs('${d.id}')">
                <span class="status-badge status-${d.status === 'success' ? 'running' : d.status === 'failed' ? 'crashed' : d.status === 'building' ? 'building' : 'pending'}">
                    <span class="status-dot"></span>
                </span>
                <code class="deploy-commit">${d.commit_sha ? d.commit_sha.slice(0,7) : '--'}</code>
                <span class="deploy-msg">${d.commit_message || 'Manual deployment'}</span>
                <span class="deploy-trigger">${d.trigger}</span>
                <span class="deploy-time">${icon('clock')} ${timeAgo(d.created_at)}</span>
            </div>`).join('')}
    </div>`;
}

function renderEnvTab(p) {
    const vars = Object.entries(p.env_vars || {});
    return `
    <div class="card">
        <div class="card-header">
            <h3>${icon('lock')} Environment Variables</h3>
            <button class="btn btn-primary btn-sm" onclick="showEnvModal('${p.id}')">Edit Variables</button>
        </div>
        <div class="card-body">
            ${vars.length === 0 ? '<p class="text-muted">No environment variables configured</p>' : `
                <table>
                    <thead><tr><th>Key</th><th>Value</th></tr></thead>
                    <tbody>
                        ${vars.map(([k]) => `<tr><td><code>${k}</code></td><td style="color:var(--text-tertiary)"><code>${'\u2022'.repeat(12)}</code></td></tr>`).join('')}
                    </tbody>
                </table>
            `}
        </div>
    </div>`;
}

function renderSettingsTab(p) {
    return `
    <div class="card mb-6">
        <div class="card-header"><h3>${icon('settings')} Project Settings</h3></div>
        <div class="card-body">
            <form id="project-settings-form">
                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">Name</label>
                        <input class="form-input" id="ps-name" value="${p.name}">
                    </div>
                    <div class="form-group">
                        <label class="form-label">Runtime</label>
                        <select class="form-select" id="ps-runtime">
                            ${['node','python','rust','go','other'].map(r => `<option value="${r}" ${p.runtime===r?'selected':''}>${r}</option>`).join('')}
                        </select>
                    </div>
                </div>
                <div class="form-group">
                    <label class="form-label">Description</label>
                    <input class="form-input" id="ps-desc" value="${p.description || ''}">
                </div>
                <div class="form-group">
                    <label class="form-label">Repository URL</label>
                    <input class="form-input" id="ps-repo" value="${p.repo_url || ''}">
                </div>
                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">Branch</label>
                        <input class="form-input" id="ps-branch" value="${p.branch}">
                    </div>
                    <div class="form-group">
                        <label class="form-label">Build Command</label>
                        <input class="form-input" id="ps-build" value="${p.build_command || ''}">
                    </div>
                </div>
                <div class="form-group">
                    <label class="form-label">Start Command</label>
                    <input class="form-input" id="ps-start" value="${p.start_command}">
                </div>
                <div class="flex items-center gap-3 mb-6">
                    <label style="display:flex;align-items:center;gap:8px;cursor:pointer">
                        <input type="checkbox" id="ps-autodeploy" ${p.auto_deploy?'checked':''}> Auto-deploy on push
                    </label>
                </div>
                <button type="submit" class="btn btn-primary">${icon('check')} Save Changes</button>
            </form>
        </div>
    </div>
    <div class="card" style="border-color:rgba(239,68,68,0.3)">
        <div class="card-header"><h3 class="text-error">${icon('trash')} Danger Zone</h3></div>
        <div class="card-body">
            <p class="text-muted mb-4">Permanently delete this project and all its deployment history.</p>
            <button class="btn btn-danger" onclick="deleteProject('${p.id}','${p.name}')">${icon('trash')} Delete Project</button>
        </div>
    </div>`;
}

// ── Settings Page (GitHub) ─────────────────────────────────
function renderSettingsPage(tab) {
    const gh = state.githubStatus;
    return `
    <div class="page-header">
        <h1>Settings</h1>
        <p>Configure your RustCluster instance</p>
    </div>
    <div class="tabs">
        <button class="tab ${tab==='github'?'active':''}" onclick="navigate('/settings/github')">${icon('github')} GitHub</button>
    </div>
    ${tab === 'github' ? renderGitHubSettings(gh) : ''}`;
}

function renderGitHubSettings(gh) {
    if (!gh || !gh.configured) {
        return `
        <div class="github-card">
            <div class="github-icon">${icon('github', 40)}</div>
            <h2 style="margin-bottom:8px">Connect GitHub</h2>
            <p class="text-muted mb-6">Create a GitHub App to import repositories and enable auto-deployments.</p>
            <button class="btn btn-primary btn-lg" id="create-gh-app-btn" onclick="createGitHubApp()">
                ${icon('link')} Create GitHub App
            </button>
            <p class="text-xs text-muted mt-4">This will redirect you to GitHub to create and install the app.</p>
        </div>`;
    }

    const app = gh.app;
    const installs = gh.installations || [];
    return `
    <div class="github-connected">
        <span style="display:flex;color:var(--success)">${icon('checkCircle', 20)}</span>
        <div style="flex:1">
            <strong>GitHub App Connected</strong>
            <div class="text-xs text-muted">${app.app_name} &middot; ID: ${app.app_id}</div>
        </div>
        <a href="${app.html_url}" target="_blank" class="btn btn-secondary btn-sm">${icon('externalLink')} View on GitHub</a>
    </div>

    <div class="card mb-6">
        <div class="card-header">
            <h3>Installations (${installs.length})</h3>
            <div class="flex gap-2">
                <button class="btn btn-secondary btn-sm" onclick="syncGitHub()">${icon('sync')} Sync</button>
                <a href="${app.html_url}/installations/new" target="_blank" class="btn btn-primary btn-sm">${icon('plus')} Install on Account</a>
            </div>
        </div>
        <div class="card-body">
            ${installs.length === 0 ? '<p class="text-muted">No installations yet. Click "Install on Account" to give the app access to your repositories.</p>' : `
                <table>
                    <thead><tr><th>Account</th><th>Type</th><th>Repos</th><th></th></tr></thead>
                    <tbody>
                        ${installs.map(i => `
                            <tr>
                                <td><strong>${i.account_login}</strong></td>
                                <td>${i.account_type}</td>
                                <td>${i.repos_count} repos</td>
                                <td><a href="${app.html_url}/installations/new" target="_blank" class="text-sm">Configure</a></td>
                            </tr>`).join('')}
                    </tbody>
                </table>
            `}
        </div>
    </div>

    <div class="card" style="border-color:rgba(239,68,68,0.3)">
        <div class="card-header"><h3 class="text-error">${icon('trash')} Danger Zone</h3></div>
        <div class="card-body">
            <p class="text-muted mb-4">Remove the GitHub App connection. You'll need to create a new one to re-connect.</p>
            <button class="btn btn-danger" onclick="deleteGitHubApp()">${icon('x')} Disconnect GitHub</button>
        </div>
    </div>`;
}

// ── Actions ────────────────────────────────────────────────
async function projectAction(id, action) {
    try {
        await api(`/projects/${id}/${action}`, { method: 'POST' });
        toast(`Project ${action}ed successfully`, 'success');
        await fetchProjects();
        render();
    } catch (e) { toast(e.message, 'error'); }
}

async function deleteProject(id, name) {
    if (!confirm(`Are you sure you want to delete "${name}"? This cannot be undone.`)) return;
    try {
        await api(`/projects/${id}`, { method: 'DELETE' });
        toast('Project deleted', 'success');
        state.projects = state.projects.filter(p => p.id !== id);
        navigate('/');
    } catch (e) { toast(e.message, 'error'); }
}

async function showDeployLogs(deployId) {
    try {
        const d = await api(`/deployments/${deployId}`);
        const dep = d.deployment;
        const overlay = document.createElement('div');
        overlay.className = 'modal-overlay';
        overlay.onclick = (e) => { if (e.target === overlay) overlay.remove(); };
        overlay.innerHTML = `
        <div class="modal" style="max-width:700px">
            <div class="modal-header">
                <h3>${icon('terminal')} Deployment Logs</h3>
                <button class="btn btn-ghost btn-icon" onclick="this.closest('.modal-overlay').remove()">${icon('x')}</button>
            </div>
            <div class="modal-body" style="padding:0">
                <div class="terminal" style="border:none;border-radius:0">
                    <div class="terminal-header">
                        <div class="terminal-dots"><span class="dot-red"></span><span class="dot-yellow"></span><span class="dot-green"></span></div>
                        <span class="terminal-title">${dep.commit_sha ? dep.commit_sha.slice(0,7) : 'deploy'} &middot; ${dep.status}</span>
                    </div>
                    <div class="terminal-body" style="max-height:400px">${colorizeLogs(dep.logs || 'No logs available')}</div>
                </div>
            </div>
        </div>`;
        document.body.appendChild(overlay);
    } catch (e) { toast(e.message, 'error'); }
}

function showEnvModal(projectId) {
    const p = state.currentProject?.project;
    if (!p) return;
    const vars = Object.entries(p.env_vars || {});
    const overlay = document.createElement('div');
    overlay.className = 'modal-overlay';
    overlay.onclick = (e) => { if (e.target === overlay) overlay.remove(); };
    overlay.innerHTML = `
    <div class="modal">
        <div class="modal-header">
            <h3>${icon('lock')} Environment Variables</h3>
            <button class="btn btn-ghost btn-icon" onclick="this.closest('.modal-overlay').remove()">${icon('x')}</button>
        </div>
        <div class="modal-body">
            <div id="modal-env-editor">
                ${vars.map(([k, v]) => `
                    <div class="env-row">
                        <input class="form-input" value="${k}" data-env-key>
                        <input class="form-input" value="${v}" data-env-val type="password">
                        <button type="button" class="btn btn-ghost btn-icon" onclick="this.closest('.env-row').remove()">${icon('x')}</button>
                    </div>`).join('')}
                ${vars.length === 0 ? `
                    <div class="env-row">
                        <input class="form-input" placeholder="KEY" data-env-key>
                        <input class="form-input" placeholder="VALUE" data-env-val>
                        <button type="button" class="btn btn-ghost btn-icon" onclick="this.closest('.env-row').remove()">${icon('x')}</button>
                    </div>` : ''}
            </div>
            <button type="button" class="btn btn-ghost btn-sm mt-4" onclick="addEnvRow('modal-env-editor')">${icon('plus')} Add Variable</button>
        </div>
        <div class="modal-footer">
            <button class="btn btn-secondary" onclick="this.closest('.modal-overlay').remove()">Cancel</button>
            <button class="btn btn-primary" onclick="saveEnvVars('${projectId}')">${icon('check')} Save Variables</button>
        </div>
    </div>`;
    document.body.appendChild(overlay);
}

async function saveEnvVars(projectId) {
    const editor = document.getElementById('modal-env-editor');
    const rows = editor.querySelectorAll('.env-row');
    const env_vars = {};
    rows.forEach(row => {
        const k = row.querySelector('[data-env-key]').value.trim();
        const v = row.querySelector('[data-env-val]').value;
        if (k) env_vars[k] = v;
    });
    try {
        await api(`/projects/${projectId}/env`, { method: 'PUT', body: JSON.stringify({ env_vars }) });
        toast('Variables saved', 'success');
        document.querySelector('.modal-overlay')?.remove();
        render();
    } catch (e) { toast(e.message, 'error'); }
}

async function createGitHubApp() {
    const btn = document.getElementById('create-gh-app-btn');
    if (btn) { btn.disabled = true; btn.innerHTML = icon('sync') + ' Creating...'; }
    try {
        const data = await api('/github/setup/create', { method: 'POST' });
        const form = document.createElement('form');
        form.method = 'POST';
        form.action = 'https://github.com/settings/apps/new';
        const input = document.createElement('input');
        input.type = 'hidden'; input.name = 'manifest';
        input.value = JSON.stringify(data.manifest);
        form.appendChild(input);
        document.body.appendChild(form);
        form.submit();
    } catch (e) {
        toast(e.message, 'error');
        if (btn) { btn.disabled = false; btn.innerHTML = icon('link') + ' Create GitHub App'; }
    }
}

async function deleteGitHubApp() {
    if (!confirm('Disconnect GitHub? You will lose access to all linked repositories.')) return;
    try {
        await api('/github/setup/delete', { method: 'DELETE' });
        toast('GitHub disconnected', 'success');
        render();
    } catch (e) { toast(e.message, 'error'); }
}

async function syncGitHub() {
    try {
        const data = await api('/github/sync', { method: 'POST' });
        toast(data.message, 'success');
        render();
    } catch (e) { toast(e.message, 'error'); }
}

// ── Event Binding ──────────────────────────────────────────
function bindPageEvents() {
    const npForm = document.getElementById('new-project-form');
    if (npForm) {
        npForm.addEventListener('submit', async (e) => {
            e.preventDefault();
            const btn = document.getElementById('np-submit');
            btn.disabled = true; btn.innerHTML = icon('sync') + ' Creating...';
            const env_vars = collectEnvVars('env-editor');
            try {
                await api('/projects', {
                    method: 'POST',
                    body: JSON.stringify({
                        name: document.getElementById('np-name').value,
                        description: document.getElementById('np-desc').value,
                        repo_url: document.getElementById('np-repo').value,
                        branch: document.getElementById('np-branch').value,
                        build_command: document.getElementById('np-build').value,
                        start_command: document.getElementById('np-start').value,
                        runtime: document.getElementById('np-runtime').value,
                        env_vars,
                        auto_deploy: document.getElementById('np-autodeploy').checked,
                    })
                });
                toast('Project created!', 'success');
                await fetchProjects();
                navigate('/');
            } catch (e) {
                toast(e.message, 'error');
                btn.disabled = false; btn.innerHTML = icon('plus') + ' Create Project';
            }
        });

        const search = document.getElementById('np-repo-search');
        if (search) {
            search.addEventListener('input', () => {
                const q = search.value.toLowerCase();
                document.querySelectorAll('#repo-list .repo-item').forEach(el => {
                    el.style.display = el.querySelector('.repo-name').textContent.toLowerCase().includes(q) ? '' : 'none';
                });
            });
        }
    }

    const psForm = document.getElementById('project-settings-form');
    if (psForm) {
        const p = state.currentProject?.project;
        psForm.addEventListener('submit', async (e) => {
            e.preventDefault();
            try {
                await api(`/projects/${p.id}`, {
                    method: 'PUT',
                    body: JSON.stringify({
                        name: document.getElementById('ps-name').value,
                        description: document.getElementById('ps-desc').value,
                        repo_url: document.getElementById('ps-repo').value,
                        branch: document.getElementById('ps-branch').value,
                        build_command: document.getElementById('ps-build').value,
                        start_command: document.getElementById('ps-start').value,
                        runtime: document.getElementById('ps-runtime').value,
                        auto_deploy: document.getElementById('ps-autodeploy').checked,
                    })
                });
                toast('Settings saved', 'success');
                await fetchProjects();
                render();
            } catch (e) { toast(e.message, 'error'); }
        });
    }
}

// ── Helpers ────────────────────────────────────────────────
function selectRepo(el) {
    document.querySelectorAll('.repo-item').forEach(r => r.classList.remove('selected'));
    el.classList.add('selected');
    document.getElementById('np-repo').value = el.dataset.clone;
    const branchInput = document.getElementById('np-branch');
    if (branchInput && el.dataset.branch) branchInput.value = el.dataset.branch;
}

function addEnvRow(containerId = 'env-editor') {
    const container = document.getElementById(containerId);
    const row = document.createElement('div');
    row.className = 'env-row';
    row.innerHTML = `
        <input class="form-input" placeholder="KEY" data-env-key>
        <input class="form-input" placeholder="VALUE" data-env-val>
        <button type="button" class="btn btn-ghost btn-icon" onclick="this.closest('.env-row').remove()">${icon('x')}</button>`;
    container.appendChild(row);
}

function collectEnvVars(containerId) {
    const container = document.getElementById(containerId);
    if (!container) return {};
    const rows = container.querySelectorAll('.env-row');
    const env = {};
    rows.forEach(row => {
        const k = row.querySelector('[data-env-key]')?.value.trim();
        const v = row.querySelector('[data-env-val]')?.value;
        if (k) env[k] = v || '';
    });
    return env;
}

function getRuntimeIcon(runtime, size) {
    const map = { node: 'node', python: 'python', rust: 'rust', go: 'server' };
    return icon(map[runtime] || 'box', size);
}

function timeAgo(dateStr) {
    if (!dateStr) return '--';
    const d = new Date(dateStr);
    const now = new Date();
    const secs = Math.floor((now - d) / 1000);
    if (secs < 60) return 'just now';
    if (secs < 3600) return Math.floor(secs / 60) + 'm ago';
    if (secs < 86400) return Math.floor(secs / 3600) + 'h ago';
    return Math.floor(secs / 86400) + 'd ago';
}

function formatUptime(seconds) {
    if (!seconds) return '--';
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    if (h > 0) return `${h}h ${m}m`;
    return `${m}m`;
}

function colorizeLogs(text) {
    if (!text) return '';
    return text
        .replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')
        .replace(/(success|completed|done|ready)/gi, '<span class="log-success">$1</span>')
        .replace(/(error|failed|fatal|panic)/gi, '<span class="log-error">$1</span>')
        .replace(/(warning|warn|deprecated)/gi, '<span class="log-warn">$1</span>')
        .replace(/(info|building|cloning|installing|pulling)/gi, '<span class="log-info">$1</span>');
}

function startPolling() {
    if (state.pollInterval) clearInterval(state.pollInterval);
    state.pollInterval = setInterval(async () => {
        const route = getRoute();
        if (route.path === '/' || route.path === '') {
            await fetchProjects();
            await fetchStats();
            const statsGrid = document.querySelector('.stats-grid');
            if (statsGrid && state.stats) {
                const s = state.stats;
                const cards = statsGrid.querySelectorAll('.stat-value');
                if (cards[0]) cards[0].textContent = s.total_projects;
                if (cards[1]) cards[1].textContent = s.running_projects;
                if (cards[2]) cards[2].textContent = s.total_deployments;
                if (cards[3]) cards[3].textContent = s.memory_percent ? s.memory_percent.toFixed(0) + '%' : '--';
            }
        }
    }, 5000);
}

// ── Init ───────────────────────────────────────────────────
render();
