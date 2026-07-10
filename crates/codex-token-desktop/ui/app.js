const invoke = window.__TAURI__.core.invoke;

const WINDOW_WIDTH = 360;
const DEFAULT_LULU_WIDTH = 290;
const MIN_LULU_WIDTH = 47.5;
const LULU_ASPECT = 1173 / 879;

const state = {
  expanded: true,
  lastTotal: 0,
  pointer: null,
  moveFrame: 0,
  pendingMove: null,
  dragged: false,
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
dragHandle.addEventListener("pointerdown", beginWindowMove);
document.addEventListener("pointermove", moveWindow);
document.addEventListener("pointerup", finishWindowMove);
document.addEventListener("pointercancel", finishWindowMove);

sizeInput.addEventListener("input", () => {
  applyLuluSize(Number(sizeInput.value), true);
});

function loadLuluSize() {
  const saved = Number(window.localStorage.getItem("luluSize"));
  const size = Number.isFinite(saved) && saved > 0 ? saved : DEFAULT_LULU_WIDTH;
  applyLuluSize(size, false);
}

async function beginWindowMove(event) {
  if (event.button !== 0) {
    return;
  }

  const target = event.currentTarget;
  const pointerId = event.pointerId;
  const startX = event.screenX;
  const startY = event.screenY;

  state.pointer = {
    id: pointerId,
    target,
    startX,
    startY,
    windowX: 0,
    windowY: 0,
    active: false,
    ready: false,
  };
  target.setPointerCapture?.(pointerId);

  try {
    const position = await invoke("get_window_position");
    if (!state.pointer || state.pointer.id !== pointerId) {
      return;
    }
    state.pointer.windowX = position.x;
    state.pointer.windowY = position.y;
    state.pointer.ready = true;
  } catch (error) {
    statusLine.textContent = `拖动失败：${error}`;
    document.body.dataset.status = "error";
  }
}

function moveWindow(event) {
  const pointer = state.pointer;
  if (!pointer || !pointer.ready || event.pointerId !== pointer.id || event.buttons !== 1) {
    return;
  }

  const dx = event.screenX - pointer.startX;
  const dy = event.screenY - pointer.startY;
  if (!pointer.active && Math.hypot(dx, dy) < 4) {
    return;
  }

  pointer.active = true;
  state.dragged = true;
  scheduleWindowMove(pointer.windowX + dx, pointer.windowY + dy);
}

function scheduleWindowMove(x, y) {
  state.pendingMove = { x, y };
  if (state.moveFrame) {
    return;
  }

  state.moveFrame = window.requestAnimationFrame(async () => {
    const move = state.pendingMove;
    state.pendingMove = null;
    state.moveFrame = 0;
    if (!move) {
      return;
    }

    try {
      await invoke("set_window_position", move);
    } catch (error) {
      statusLine.textContent = `拖动失败：${error}`;
      document.body.dataset.status = "error";
    }
  });
}

function finishWindowMove(event) {
  const pointer = state.pointer;
  if (!pointer || event.pointerId !== pointer.id) {
    return;
  }

  pointer.target.releasePointerCapture?.(pointer.id);
  state.pointer = null;
  window.setTimeout(() => {
    state.dragged = false;
  }, 160);
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
