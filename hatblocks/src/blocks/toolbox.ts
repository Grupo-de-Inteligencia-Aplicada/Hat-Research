import { deviceTypePriorities, type Device, type RuntimeEvent } from "../services/api";
import * as Colors from "./colors";
import { binaryConditionBlocks } from './conditions';
import { getBlockTypeFor } from "./devices";

export default function generateToolbox(events: RuntimeEvent[], devices: Device[]) {
  devices.sort((a, b) => (deviceTypePriorities[b.typ] - deviceTypePriorities[a.typ]));

  return {
    "kind": "categoryToolbox",
    "contents": [
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Automação",
        "colour": Colors.AUTOMATION_BLOCK_COLOR,
        "contents": [
          { "kind": "block", "type": "automation" },
          { "kind": "block", "type": "automation_time_based" },
        ]
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Eventos",
        "colour": Colors.EVENT_BLOCK_COLOR,
        "contents": devices.map(d => ({
          "kind": "block",
          "type": "event_dev_" + d.id
        }))
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Condições",
        "colour": Colors.CONDITION_BLOCK_COLOR,
        "contents": binaryConditionBlocks.map(blockDef => `condition_${blockDef.type}`).concat([
          "condition_event_was_from_device",
          "condition_event_time_between",
          "condition_device_is_on",
          "condition_device_is_off",
          "condition_motion_sensor",
          "condition_sensor_value",
        ])
          .map(a => ({
            "kind": "block",
            "type": a
          }))
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Ações",
        "colour": Colors.ACTION_BLOCK_COLOR,
        "contents": [
          "action_turn_on_device",
          "action_turn_off_device",
          "action_set_light_color",
          "action_set_light_brightness",
          "action_wait",
        ].map(a => ({
          "kind": "block",
          "type": a
        }))
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Dispositivos",
        "colour": Colors.DEVICE_BLOCK_COLOR,
        "contents": devices.map(d => ({
          "kind": "block",
          "type": getBlockTypeFor(d),
        }))
      },
    ]
  };
};

