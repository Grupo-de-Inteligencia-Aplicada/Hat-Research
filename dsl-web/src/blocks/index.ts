import * as Blockly from "blockly";
import AutomationBlock from './automation';
import setupEventBlocks, { type RuntimeEvent } from './events';
import generateToolbox from "./toolbox";
import type { Device } from "./devices";
import setupDeviceBlocks from "./devices";
import setupActionBlocks from './actions';
import setupConditionBlocks from "./conditions";

export function setupBlockly() {
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
  const deviceList: Device[] = [
    {
      id: "HassIntegration0@light.office_light",
      name: "Office Light"
    },
    {
      id: "HassIntegration0@light.desk_light",
      name: "Desk Light"
    },
    {
      id: "HassIntegration0@binary_sensor.door_sensor",
      name: "Office Door Sensor"
    },
  ];

  const toolbox = generateToolbox(eventList, deviceList);
  setupEventBlocks(eventList);
  setupDeviceBlocks(deviceList);
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
