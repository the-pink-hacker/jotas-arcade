function postGameSetup(gameInfo) {
  // Setup dynamic css
  const style = document.createElement("style");
  style.innerHTML = `:root {
    aspect-ratio: ${gameInfo.aspect_ratio_width}/${gameInfo.aspect_ratio_height};
  }
  `;
  document.body.appendChild(style);

  // Fill game canvas placeholder
  $("canvas").removeAttr("style")
    .attr("id", "game-canvas");
  const canvas = document.getElementById("game-canvas");
  const canvasPlaceholder = document.getElementById("game-canvas-placeholder");
  canvasPlaceholder.insertAdjacentElement("beforebegin", canvas);
  canvasPlaceholder.parentNode.removeChild(canvasPlaceholder);
}
