fetch("state.json", {
  method: "get",
  headers: {
    pragma: "no-cache",
    "cache-control": "no-cache",
  },
})
  .then((response) => response.json())
  .then(addToForm);

function addToForm(scoreboard) {
  for (const key of Object.keys(scoreboard)) {
    document.getElementById(key).value = scoreboard[key];
  }
}

function submitForm() {
  fetch("/api/control", {
    method: "post",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(
      Array.from(document.querySelectorAll("input")).reduce((acc, x) => {
        acc[x.id] = x.type === "number" ? +x.value : x.value;
        return acc;
      }, {})
    ),
  });
}
