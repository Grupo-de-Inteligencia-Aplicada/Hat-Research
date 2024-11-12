import * as Blockly from "blockly";
import AutomationBlock from './automation';
import setupEventBlocks, { type RuntimeEvent } from './events';
import generateToolbox from "./toolbox";
import setupDeviceBlocks from "./devices";
import setupActionBlocks from './actions';
import setupConditionBlocks from "./conditions";
import type { Device } from "../services/api";

export function setupBlockly(devices: Device[]) {
  const eventList: RuntimeEvent[] = [
    {
      name: "dummy",
      label: "Dummy Event"
    },
    {
      name: "DoorOpenEvent",
      label: "Door opened"
    },
  ];

  const toolbox = generateToolbox(eventList, devices);
  setupEventBlocks(eventList);
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
