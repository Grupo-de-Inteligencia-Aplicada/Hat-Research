import * as Blockly from 'blockly';
import type { Device, DeviceType } from '../services/api';

function getIconFor(typ: DeviceType): string {
  switch (typ) {
    case 'Dummy':
      return "";
    case 'DoorSensor':
      return "🚪";
    case 'Light':
      return "💡";
    case 'Sensor':
      return "📟";
    case 'PowerOutlet':
      return "🔌";
    case 'MotionSensor':
      return "📡";
    case 'Unknown':
      return "";
  }
}

export default function setupDeviceBlocks(devices: Device[]) {

  Blockly.defineBlocksWithJsonArray(devices.map(d => ({
    "type": "device_" + d.id,
    "tooltip": "",
    "helpUrl": "",
    "message0": getIconFor(d.typ) + " " + d.name + " %1",
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
