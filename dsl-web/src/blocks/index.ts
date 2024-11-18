import * as Blockly from "blockly";
import setupEventBlocks from './events';
import generateToolbox from "./toolbox";
import setupDeviceBlocks from "./devices";
import setupActionBlocks from './actions';
import setupConditionBlocks from "./conditions";
import type { Device, RuntimeEvent } from "../services/api";
import setupAutomationBlock from "./automation";

export function setupBlockly(devices: Device[], possibleEvents: RuntimeEvent[]) {
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

  return workspace;
}
