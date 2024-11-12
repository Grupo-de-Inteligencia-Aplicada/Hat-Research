import * as Blockly from "blockly";

export interface RuntimeEvent {
  name: string;
  label: string;
};

export default function defineEventBlocks(events: RuntimeEvent[]) {
  Blockly.defineBlocksWithJsonArray(events.map(e => ({
    "type": "event_" + e.name,
    "tooltip": "",
    "helpUrl": "",
    "message0": e.label + " %1",
    "args0": [
      {
        "type": "input_dummy",
        "name": ""
      }
    ],
    "output": "event_block",
    "colour": 70
  })));
}
