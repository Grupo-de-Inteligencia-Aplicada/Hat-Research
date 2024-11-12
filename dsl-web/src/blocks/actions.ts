import * as Blockly from 'blockly';

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
}
