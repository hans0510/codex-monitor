const invoke = window.__TAURI__.core.invoke;
const appWindow = window.__TAURI__.window.getCurrentWindow();

const state = {
  expanded: true,
  lastTotal: 0,
  companionPointer: null,
  companionDragged: false,
};

const capybara = document.getElementById("capybara");
const panel = document.getElementById("statsPanel");
const rangeGrid = document.getElementById("rangeGrid");
const statusLine = document.getElementById("statusLine");
const homeLine = document.getElementById("homeLine");
const allTotal = document.getElementById("allTotal");
const inputTotal = document.getElementById("inputTotal");
const cachedTotal = document.getElementById("cachedTotal");
const outputTotal = document.getElementById("outputTotal");
const reasoningTotal = document.getElementById("reasoningTotal");
const latestSession = document.getElementById("latestSession");
const latestMeta = document.getElementById("latestMeta");
const dragHandle = document.getElementById("dragHandle");

capybara.addEventListener("click", () => {
  if (state.companionDragged) {
    state.companionDragged = false;
    return;
  }
  state.expanded = !state.expanded;
  panel.classList.toggle("collapsed", !state.expanded);
});

capybara.addEventListener("mousedown", (event) => {
  if (event.buttons !== 1) {
    return;
  }
  state.companionPointer = {
    x: event.clientX,
    y: event.clientY,
  };
});

document.addEventListener("mousemove", async (event) => {
  if (!state.companionPointer || event.buttons !== 1) {
    return;
  }

  const dx = event.clientX - state.companionPointer.x;
  const dy = event.clientY - state.companionPointer.y;
  if (Math.hypot(dx, dy) >= 4) {
    state.companionPointer = null;
    state.companionDragged = true;
    await appWindow.startDragging();
  }
});

document.addEventListener("mouseup", () => {
  state.companionPointer = null;
  window.setTimeout(() => {
    state.companionDragged = false;
  }, 120);
});

dragHandle.addEventListener("mousedown", async (event) => {
  if (event.buttons === 1) {
    await appWindow.startDragging();
  }
});

async function refreshUsage() {
  try {
    const usage = await invoke("get_usage");
    renderUsage(usage);
  } catch (error) {
    statusLine.textContent = `读取失败：${error}`;
    document.body.dataset.status = "error";
  }
}

function renderUsage(usage) {
  document.body.dataset.status = usage.status;
  statusLine.textContent = usage.message;
  homeLine.textContent = usage.codexHome ? usage.codexHome : "";

  rangeGrid.replaceChildren(
    ...usage.ranges.map((range) => {
      const item = document.createElement("div");
      item.className = "range-item";
      item.innerHTML = `<span>${range.label}</span><strong>${range.usage.total}</strong>`;
      return item;
    }),
  );

  const all = usage.ranges.find((range) => range.label === "All");
  if (all) {
    allTotal.textContent = all.usage.total;
    inputTotal.textContent = all.usage.input;
    cachedTotal.textContent = all.usage.cached;
    outputTotal.textContent = all.usage.output;
    reasoningTotal.textContent = all.usage.reasoning;
    capybara.classList.toggle("bumped", all.usage.totalRaw > state.lastTotal);
    state.lastTotal = all.usage.totalRaw;
    window.setTimeout(() => capybara.classList.remove("bumped"), 420);
  }

  if (usage.latestSession) {
    latestSession.textContent = `${usage.latestSession.id} · ${usage.latestSession.total}`;
    latestMeta.textContent = `${usage.latestSession.eventCount} events · ${usage.latestSession.lastEventAt}`;
  } else {
    latestSession.textContent = "-";
    latestMeta.textContent = "等待 session";
  }
}

refreshUsage();
window.setInterval(refreshUsage, 2000);
