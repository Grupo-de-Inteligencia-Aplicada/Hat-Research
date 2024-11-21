import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';

export const binaryConditionBlocks = [
  {
    "type": "and",
    "message": "and",
    "code": "and",
  },
  {
    "type": "or",
    "message": "or",
    "code": "or",
  },
];

export default function setupConditionBlocks() {
  binaryConditionBlocks.forEach(blockDefinition => {
    const blockType = 'condition_' + blockDefinition.type;

    Blockly.defineBlocksWithJsonArray([{
      "type": blockType,
      "tooltip": "",
      "helpUrl": "",
      "message0": "%1 " + blockDefinition.message + " %2 %3",
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
    }]);

    javascriptGenerator.forBlock[blockType] = (block, generator) => {
      const firstCondition = generator.valueToCode(block, 'CONDITION1', Order.ATOMIC);
      const secondCondition = generator.valueToCode(block, 'CONDITION2', Order.ATOMIC);
      return [`${firstCondition} ${blockDefinition.code} ${secondCondition}`, Order.ATOMIC];
    };
  });

  Blockly.defineBlocksWithJsonArray([
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

  javascriptGenerator.forBlock['condition_event_was_from_device'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);

    return ['get_device() == "' + device + '"', Order.ATOMIC];
  };
}
