import * as Blockly from 'blockly';

export default function setupConditionBlocks() {
  Blockly.defineBlocksWithJsonArray([
    {
      "type": "condition_and",
      "tooltip": "",
      "helpUrl": "",
      "message0": "%1 And %2 %3",
      "args0": [
        {
          "type": "input_value",
          "name": "CONDITION1",
          "check": "Boolean"
        },
        {
          "type": "input_dummy",
          "name": ""
        },
        {
          "type": "input_value",
          "name": "CONDITION2",
          "check": "Boolean"
        }
      ],
      "output": "Boolean",
      "colour": 120
    },
    {
      "type": "condition_event_was_from_device",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Event was from %1",
      "args0": [
        {
          "type": "input_value",
          "name": "NAME",
          "check": "device_block"
        }
      ],
      "output": "Boolean",
      "colour": 225
    },
  ]);
}
