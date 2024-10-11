import { getThemeDefinitions } from "./api.js";

"use strict";

document.addEventListener("DOMContentLoaded", initializeThemes);

async function initializeThemes() {
  const definitions = await getThemeDefinitions();

  for (const definition of definitions) {
    const startMonth = definition.startDateInclusive.month;
    const startDay = definition.startDateInclusive.day;

    const endMonth = definition.endDateInclusive.month;
    const endDay = definition.endDateInclusive.day;

    if (isNowInRange(startMonth, startDay, endMonth, endDay)) {
      addTheme(definition.name);
    }
  }
}

function addTheme(themeName) {
  const scriptNode = document.createElement("script");
  scriptNode.src = `/static/themes/${themeName}/${themeName}.js`;
  document.body.appendChild(scriptNode);
}

function isNowInRange(startMonth, startDay, endMonth, endDay) {
  const now = new Date();

  // getMonth is zero indexed for some fucking reason
  const startMonthDaysTotal = daysInMonth(startMonth - 1, now.getFullYear());
  const startDate = new Date(now.getFullYear(), startMonth - 1, Math.min(startDay, startMonthDaysTotal), 0, 0, 0, 0);

  let endYear = now.getFullYear();
  if (startMonth > endMonth) {
    endYear++;
  }

  const endMonthDaysTotal = daysInMonth(endMonth - 1, endYear);
  const endDate = new Date(endYear, endMonth - 1, Math.min(endDay, endMonthDaysTotal), 23, 59, 59, 0);

  return now >= startDate && now <= endDate;
}

function daysInMonth(month, year) {
  return new Date(year, month, 0).getDate();
}
