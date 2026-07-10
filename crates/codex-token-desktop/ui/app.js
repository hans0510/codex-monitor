const invoke = window.__TAURI__.core.invoke;
const appWindow = window.__TAURI__.window.getCurrentWindow();

const WINDOW_WIDTH = 360;
const DEFAULT_LULU_WIDTH = 290;
const MIN_LULU_WIDTH = 47.5;
const LULU_ASPECT = 1173 / 879;

const state = {
  expanded: true,
  lastTotal: 0,
  pointer: null,
  dragged: false,
  dragActive: false,
  dragIdleTimer: 0,
  refreshInFlight: false,
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
const fiveHourRemaining = document.getElementById("fiveHourRemaining");
const fiveHourReset = document.getElementById("fiveHourReset");
const weeklyRemaining = document.getElementById("weeklyRemaining");
const weeklyReset = document.getElementById("weeklyReset");
const dragHandle = document.getElementById("dragHandle");
const sizeInput = document.getElementById("luluSize");
const sizeValue = document.getElementById("luluSizeValue");

capybara.addEventListener("click", () => {
  if (state.dragged) {
    state.dragged = false;
    return;
  }
  state.expanded = !state.expanded;
  panel.classList.toggle("collapsed", !state.expanded);
});

capybara.addEventListener("pointerdown", beginWindowMove);
dragHandle.addEventListener("pointerdown", beginImmediateWindowDrag);
document.addEventListener("pointermove", startWindowDrag);
document.addEventListener("pointerup", finishWindowMove);
document.addEventListener("pointercancel", finishWindowMove);
void appWindow.onMoved(() => {
  if (state.dragActive) {
    markWindowMoved();
  }
});

sizeInput.addEventListener("input", () => {
  applyLuluSize(Number(sizeInput.value), true);
});

function loadLuluSize() {
  const saved = Number(window.localStorage.getItem("luluSize"));
  const size = Number.isFinite(saved) && saved > 0 ? saved : DEFAULT_LULU_WIDTH;
  applyLuluSize(size, false);
}

function beginWindowMove(event) {
  if (event.button !== 0) {
    return;
  }

  const target = event.currentTarget;
  state.pointer = {
    id: event.pointerId,
    target,
    startX: event.screenX,
    startY: event.screenY,
  };
  target.setPointerCapture?.(event.pointerId);
}

function beginImmediateWindowDrag(event) {
  if (event.button !== 0) {
    return;
  }

  state.pointer = null;
  state.dragged = true;
  markWindowMoved();
  void startNativeWindowDrag();
}

function startWindowDrag(event) {
  const pointer = state.pointer;
  if (!pointer || event.pointerId !== pointer.id || event.buttons !== 1) {
    return;
  }

  const dx = event.screenX - pointer.startX;
  const dy = event.screenY - pointer.startY;
  if (Math.hypot(dx, dy) < 3) {
    return;
  }

  pointer.target.releasePointerCapture?.(pointer.id);
  state.pointer = null;
  state.dragged = true;
  markWindowMoved();
  void startNativeWindowDrag();
}

async function startNativeWindowDrag() {
  try {
    await appWindow.startDragging();
  } catch (error) {
    state.dragActive = false;
    state.dragged = false;
    statusLine.textContent = `拖动失败：${error}`;
    document.body.dataset.status = "error";
  }
}

function markWindowMoved() {
  state.dragActive = true;
  window.clearTimeout(state.dragIdleTimer);
  state.dragIdleTimer = window.setTimeout(() => {
    state.dragActive = false;
    state.dragged = false;
    void refreshUsage();
  }, 180);
}

function finishWindowMove(event) {
  const pointer = state.pointer;
  if (!pointer || event.pointerId !== pointer.id) {
    return;
  }

  pointer.target.releasePointerCapture?.(pointer.id);
  state.pointer = null;
}

function applyLuluSize(width, persist) {
  const safeWidth = Math.min(340, Math.max(MIN_LULU_WIDTH, width));
  const safeHeight = Math.round(safeWidth * LULU_ASPECT);
  capybara.style.width = `${safeWidth}px`;
  capybara.style.height = `${safeHeight}px`;
  capybara.style.left = `${Math.round((WINDOW_WIDTH - safeWidth) / 2)}px`;
  sizeInput.value = String(safeWidth);
  sizeValue.textContent = `${Math.round((safeWidth / DEFAULT_LULU_WIDTH) * 100)}%`;
  if (persist) {
    window.localStorage.setItem("luluSize", String(safeWidth));
  }
}

async function refreshUsage() {
  if (state.refreshInFlight || state.dragActive) {
    return;
  }

  state.refreshInFlight = true;
  try {
    const usage = await invoke("get_usage");
    renderUsage(usage);
  } catch (error) {
    statusLine.textContent = `读取失败：${error}`;
    document.body.dataset.status = "error";
  } finally {
    state.refreshInFlight = false;
  }
}

function renderUsage(usage) {
  document.body.dataset.status = usage.status;
  statusLine.textContent = usage.message;
  homeLine.textContent = usage.codexHome ? usage.codexHome : "";

  rangeGrid.replaceChildren(
    ...usage.ranges
      .filter((range) => range.label !== "All")
      .map((range) => {
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

  renderQuota(usage.quotas?.fiveHour, fiveHourRemaining, fiveHourReset);
  renderQuota(usage.quotas?.weekly, weeklyRemaining, weeklyReset);
}

function renderQuota(quota, remainingElement, resetElement) {
  if (!quota) {
    remainingElement.textContent = "剩余 --";
    resetElement.textContent = "暂无额度数据";
    return;
  }

  remainingElement.textContent = `剩余 ${formatPercent(quota.remainingPercent)}%`;
  resetElement.textContent = `刷新 ${formatResetTime(quota.resetsAt)}`;
}

function formatPercent(value) {
  const rounded = Math.round(Number(value) * 10) / 10;
  return Number.isInteger(rounded) ? String(rounded) : rounded.toFixed(1);
}

function formatResetTime(unixSeconds) {
  const date = new Date(Number(unixSeconds) * 1000);
  if (Number.isNaN(date.getTime())) {
    return "时间未知";
  }

  return new Intl.DateTimeFormat("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  }).format(date);
}

loadLuluSize();
refreshUsage();
window.setInterval(refreshUsage, 2000);
