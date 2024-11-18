import * as Blockly from "blockly";
import { javascriptGenerator, Order } from 'blockly/javascript';
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

  events.forEach(e => {
    javascriptGenerator.forBlock['event_' + e.event] = (block, generator) => {
      return [e.event, Order.ATOMIC];
    };
  });
}
