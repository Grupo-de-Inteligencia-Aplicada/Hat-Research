import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';
import { DeviceBlockTypes } from './devices';

export const binaryConditionBlocks = [
  {
    "type": "and",
    "message": "e",
    "code": "and",
  },
  {
    "type": "or",
    "message": "ou",
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
      const firstConditionValue = generator.valueToCode(block, 'CONDITION1', Order.ATOMIC).trim();
      const firstCondition = firstConditionValue.length == 0 ? 'false' : firstConditionValue;
      const secondConditionValue = generator.valueToCode(block, 'CONDITION2', Order.ATOMIC).trim();
      const secondCondition = secondConditionValue.length == 0 ? 'false' : secondConditionValue;
      return [`(${firstCondition}) ${blockDefinition.code} (${secondCondition})`, Order.ATOMIC];
    };
  });

  Blockly.defineBlocksWithJsonArray([
    {
      "type": "condition_event_was_from_device",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Evento veio do dispositivo %1",
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
      "message0": "Evento aconteceu entre %1 e %2 %3",
      "args0": [
        {
          "type": "field_input",
          "name": "START_TIME",
          "text": "17:00"
        },
        {
          "type": "field_input",
          "name": "END_TIME",
          "text": "18:00"
        },
        {
          "type": "input_dummy",
          "name": ""
        }
      ],
      "output": "Boolean",
      "colour": 225
    },
    {
      "type": "condition_device_is_on",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Dispositivo %1 está ativo",
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
      "type": "condition_device_is_off",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Dispositivo %1 está inativo",
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
      "type": "condition_motion_sensor",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Sensor de movimento %2 %1 está detectando movimento",
      "args0": [
        {
          "type": "field_dropdown",
          "name": "CONDITION",
          "options": [
            [
              "is",
              "IS"
            ],
            [
              "is not",
              "ISNOT"
            ]
          ]
        },
        {
          "type": "input_value",
          "name": "DEVICE",
          "check": "device_MotionSensor",
        }
      ],
      "output": "Boolean",
      "colour": 225
    },
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

  javascriptGenerator.forBlock['condition_device_is_on'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);

    return ['is_device_on("' + device + '")', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['condition_device_is_off'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);

    return ['is_device_off("' + device + '")', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['condition_motion_sensor'] = (block, generator) => {
    const conditionValue = block.getFieldValue('CONDITION');
    const fn = conditionValue == 'IS' ? 'is_device_on' : 'is_device_off';
    const device = generator.valueToCode(block, 'DEVICE', Order.ATOMIC);

    return [`${fn}("${device}")`, Order.ATOMIC];
  };
}
