import { deviceTypePriorities, type Device, type RuntimeEvent } from "../services/api";

export default function generateToolbox(events: RuntimeEvent[], devices: Device[]) {
  devices.sort((a, b) => (deviceTypePriorities[b.typ] - deviceTypePriorities[a.typ]));

  return {
    "kind": "categoryToolbox",
    "contents": [
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Automação",
        "colour": 210,
        "contents": [
          { "kind": "block", "type": "automation" },
        ]
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Eventos",
        "colour": 70,
        "contents": events.map(e => ({
          "kind": "block",
          "type": "event_" + e.event
        }))
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Dispositivos",
        "colour": 190,
        "contents": devices.map(d => ({
          "kind": "block",
          "type": "device_" + d.id,
        }))
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Condições",
        "colour": 120,
        "contents": [
          "condition_and",
          "condition_or",
          "condition_event_was_from_device"
        ].map(a => ({
          "kind": "block",
          "type": a
        }))
      },
      {
        "kind": "category",
        "flyoutOpen": true,
        "name": "Actions",
        "colour": 10,
        "contents": [
          "action_turn_on_device",
          "action_turn_off_device",
        ].map(a => ({
          "kind": "block",
          "type": a
        }))
      },
    ]
  };
};

