const fs = require("fs");
const toml = require("toml");

// Read the gloves.toml file
const fileContent = fs.readFileSync("ny_swords.toml", "utf8");

// Parse the TOML content
const parsedData = toml.parse(fileContent);

// Transform the data structure
const transformedData = {};
for (const two_hand_sword of parsedData.two_hand_sword) {
  const key = two_hand_sword.base_type
    .toLowerCase()
    .replace(/\s+/g, "_")
    .replace("'", ""); // Remove single quotes
  transformedData[key] = two_hand_sword;
}

// Convert the transformed data back to TOML format
let newTomlContent = "";
for (const key in transformedData) {
  const escapedKey = key.replace(/'/g, "\\'"); // Escape single quotes
  newTomlContent += `[swords.${escapedKey}]\n`;
  const two_hand_sword = transformedData[key];
  for (const prop in two_hand_sword) {
    newTomlContent += `${prop} = "${two_hand_sword[prop]}"\n`;
  }
  newTomlContent += "\n";
}

// Write the new TOML content to a new file
fs.writeFileSync("output.toml", newTomlContent, "utf8");

console.log("Transformation complete.");
