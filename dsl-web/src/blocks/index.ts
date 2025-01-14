import * as Blockly from "blockly";
import setupEventBlocks from './events';
import generateToolbox from "./toolbox";
import setupDeviceBlocks, { getLabelFor } from "./devices";
import setupActionBlocks from './actions';
import setupConditionBlocks from "./conditions";
import type { Device, HatApi, RuntimeEvent } from "../services/api";
import setupAutomationBlock from "./automation";

export const DEFAULT_TOOLTIP = "tooltip";

export function setupBlockly(api: HatApi, devices: Device[], possibleEvents: RuntimeEvent[]) {
  const toolbox = generateToolbox(possibleEvents, devices);
  setupEventBlocks(possibleEvents);
  setupDeviceBlocks(devices);
  setupActionBlocks();
  setupConditionBlocks();
  setupAutomationBlock();

  let options = {
    toolbox,
    trashcan: true,
    renderer: 'zelos',
  };

  let workspace = Blockly.inject("blocklyDiv", options);

  const customTooltip = function (div, element) {
    if (element instanceof Blockly.BlockSvg) {
      // You can access the block being moused over.
      // Here we get the color of the block to set the background color.
      div.style.backgroundColor = element.getColour();
    }
    const text = document.createElement('pre');

    text.style.padding = "15px";
    text.style.fontFamily = 'inherit';
    text.style.whiteSpace = 'pre-wrap';
    text.style.maxWidth = '400px';
    text.style.overflow = 'none';

    const blockType = element.type as string;

    if (blockType.startsWith('device_')) {
      const deviceId = blockType.substring("device_".length);

      text.innerHTML = "Loading...";

      (async () => {
        const device = await api.getDevice(deviceId);
        text.innerHTML = "";
        text.innerHTML += `Nome: ${device.name}\n`;
        text.innerHTML += `Tipo: ${getLabelFor(device.typ)}\n`;
        text.innerHTML += `Valor atual: ${device.state}\n`;
      })();
    }

    const container = document.createElement('div');
    container.style.display = 'flex';
    container.appendChild(text);
    div.appendChild(container);
  };
  // Register the function in the Blockly.Tooltip so that Blockly calls it
  // when needed.
  Blockly.Tooltip.setCustomTooltip(customTooltip);
  Blockly.registry

  return workspace;
}
