import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';
import { DeviceBlockTypes } from './devices';

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
      return [`(${firstCondition}) ${blockDefinition.code} (${secondCondition})`, Order.ATOMIC];
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
          "check": DeviceBlockTypes,
        }
      ],
      "output": "Boolean",
      "colour": 225
    },
    {
      "type": "event_time_between",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Event took place between %1 and %2 %3",
      "args0": [
        {
          "type": "field_input",
          "name": "START_TIME",
          "text": "7:00"
        },
        {
          "type": "field_input",
          "name": "END_TIME",
          "text": "8:00"
        },
        {
          "type": "input_dummy",
          "name": ""
        }
      ],
      "output": "Boolean",
      "colour": 225
    }
  ]);

  javascriptGenerator.forBlock['condition_event_was_from_device'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);

    return ['(get_device() == "' + device + '")', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_time_between'] = function (block) {
    const start_time = block.getFieldValue('START_TIME');
    const end_time = block.getFieldValue('END_TIME');

    return [
      `event_time() >= time("${start_time}") and event_time() <= time("${end_time}")`,
      Order.ATOMIC
    ];
  }
}
