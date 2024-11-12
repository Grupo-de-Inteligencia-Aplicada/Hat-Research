import * as Blockly from "blockly";
import AutomationBlock from './automation';
import setupEventBlocks from './events';
import generateToolbox from "./toolbox";
import setupDeviceBlocks from "./devices";
import setupActionBlocks from './actions';
import setupConditionBlocks from "./conditions";
import type { Device, RuntimeEvent } from "../services/api";

export function setupBlockly(devices: Device[], possibleEvents: RuntimeEvent[]) {
  const toolbox = generateToolbox(possibleEvents, devices);
  setupEventBlocks(possibleEvents);
  setupDeviceBlocks(devices);
  setupActionBlocks();
  setupConditionBlocks();

  Blockly.defineBlocksWithJsonArray([
    AutomationBlock
  ]);

  let options = {
    toolbox,
    trashcan: true,
    renderer: 'zelos',
  };

  let workspace = Blockly.inject("blocklyDiv", options);

  return workspace;
}
