function postGameSetup(gameInfo) {
  // Initialize game canvas
  $("canvas").removeAttr("style")
    .attr("id", "game-canvas")
    .css("aspect-ratio", `${gameInfo.aspect_ratio_width}/${gameInfo.aspect_ratio_height}`);
  const canvas = document.getElementById("game-canvas");
  const canvasPlaceholder = document.getElementById("game-canvas-placeholder");
  canvasPlaceholder.insertAdjacentElement("beforebegin", canvas);
  canvasPlaceholder.parentNode.removeChild(canvasPlaceholder);
}
