import * as Blockly from 'blockly';
import {javascriptGenerator, Order} from 'blockly/javascript';

export default function setupActionBlocks() {
  Blockly.defineBlocksWithJsonArray([
    {
      "type": "action_turn_on_device",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Turn on %1",
      "args0": [
        {
          "type": "input_value",
          "name": "NAME",
          "check": "device_block"
        }
      ],
      "previousStatement": "action_block",
      "nextStatement": "action_block",
      "colour": 10
    },
    {
      "type": "action_turn_off_device",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Turn off %1",
      "args0": [
        {
          "type": "input_value",
          "name": "NAME",
          "check": "device_block"
        }
      ],
      "previousStatement": "action_block",
      "nextStatement": "action_block",
      "colour": 10
    },
  ]);

  javascriptGenerator.forBlock['action_turn_on_device'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);
    return `run turn_on_device("${device}")\n`;
  };
  javascriptGenerator.forBlock['action_turn_off_device'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);
    return `run turn_off_device("${device}")\n`;
  };
}
