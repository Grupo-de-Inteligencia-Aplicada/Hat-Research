import * as Blockly from "blockly";
import { javascriptGenerator, Order } from 'blockly/javascript';
import type { Device, RuntimeEvent } from "../services/api";
import { getIconFor, getLabelFor } from "./devices";
import { EVENT_BLOCK_COLOR } from "./colors";

export default function defineEventBlocks(devices: Device[], events: RuntimeEvent[]) {
  devices.forEach(d => {
    const deviceEvents = events.filter(e => e.relatedDeviceType == d.typ);

    Blockly.defineBlocksWithJsonArray([{
      "type": "event_dev_" + d.id,
      "tooltip": "tooltip",
      "helpUrl": "",
      "message0": `⠀${getIconFor(d.typ)} (${getLabelFor(d.typ)}) ${d.name} %1 ⠀ %2⠀`,
      "args0": [
        {
          "type": "input_dummy",
          "name": "",
        },
        {
          "type": "field_dropdown",
          "name": "EVENT",
          "options": deviceEvents.map(e => {
            return [e.description, e.event];
          }),
        }
      ],
      "output": "event_block",
      "colour": EVENT_BLOCK_COLOR
    }]);

    javascriptGenerator.forBlock['event_dev_' + d.id] = (block, _) => {
      const event = block.getFieldValue('EVENT');
      return [JSON.stringify({
        event: event,
        deviceId: d.id
      }), Order.ATOMIC];
    };
  });
}
