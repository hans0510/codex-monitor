const invoke = window.__TAURI__.core.invoke;
const appWindow = window.__TAURI__.window.getCurrentWindow();

const state = {
  expanded: true,
  lastTotal: 0,
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
const closeButton = document.getElementById("closeButton");

capybara.addEventListener("click", () => {
  state.expanded = !state.expanded;
  panel.classList.toggle("collapsed", !state.expanded);
});

dragHandle.addEventListener("mousedown", async (event) => {
  if (event.buttons === 1) {
    await appWindow.startDragging();
  }
});

closeButton.addEventListener("click", async () => {
  await appWindow.close();
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
