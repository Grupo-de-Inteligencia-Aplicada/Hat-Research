import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';
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

export function getBlockTypeFor(device: Device) {
  return `device_${device.integration}@${device.id}`
}

export default function setupDeviceBlocks(devices: Device[]) {

  Blockly.defineBlocksWithJsonArray(devices.map(d => ({
    "type": getBlockTypeFor(d),
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

  devices.forEach(d => {
    javascriptGenerator.forBlock[getBlockTypeFor(d)] = (block, generator) => {
      return [`${d.integration}@${d.id}`, Order.ATOMIC];
    };
  });

}
