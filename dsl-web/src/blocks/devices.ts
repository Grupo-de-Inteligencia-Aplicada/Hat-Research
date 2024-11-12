import * as Blockly from 'blockly';
import type { Device } from '../services/api';

export default function setupDeviceBlocks(devices: Device[]) {

  Blockly.defineBlocksWithJsonArray(devices.map(d => ({
    "type": "device_" + d.id,
    "tooltip": "",
    "helpUrl": "",
    "message0": d.name + " %1",
    "args0": [
      {
        "type": "input_dummy",
        "name": ""
      }
    ],
    "output": "device_block",
    "colour": 190
  })))

}
