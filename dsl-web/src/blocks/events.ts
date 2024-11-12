import * as Blockly from "blockly";
import type { RuntimeEvent } from "../services/api";

export default function defineEventBlocks(events: RuntimeEvent[]) {
  Blockly.defineBlocksWithJsonArray(events.map(e => ({
    "type": "event_" + e.event,
    "tooltip": "",
    "helpUrl": "",
    "message0": e.description + " %1",
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
