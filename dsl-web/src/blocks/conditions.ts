import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';
import { DeviceBlockTypes } from './devices';

export const binaryConditionBlocks = [
  {
    "type": "and",
    "message": "e",
    "code": "and",
    "tooltip": "Esse bloco verifica se as duas condições são verdadeiras.\nSe as duas condições forem verdadeiras, essa também será.",
  },
  {
    "type": "or",
    "message": "ou",
    "code": "or",
    "tooltip": "Esse bloco verifica se pelo menos uma das condições é verdadeira.\nSe pelo menos uma for verdadeira, essa também será.",
  },
];

export default function setupConditionBlocks() {
  binaryConditionBlocks.forEach(blockDefinition => {
    const blockType = 'condition_' + blockDefinition.type;

    Blockly.defineBlocksWithJsonArray([{
      "type": blockType,
      "tooltip": blockDefinition.tooltip,
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
      "tooltip": "Essa condição verifica se quem disparou a automação foi especificamente um dispositivo.",
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
      "type": "condition_event_time_between",
      "tooltip": "Essa condição verifica se o evento que disparou essa automação aconteceu em determinado período do dia.",
      "helpUrl": "",
      "message0": "Evento aconteceu entre %1 e %2 %3",
      "args0": [
        {
          "type": "field_input",
          "name": "TIME0",
          "text": "17:00"
        },
        {
          "type": "field_input",
          "name": "TIME1",
          "text": "18:00"
        },
        {
          "type": "input_dummy",
          "name": ""
        }
      ],
      "output": "Boolean",
      "colour": 225,
      "extensions": ["time_validator"],
    },
    {
      "type": "condition_device_is_on",
      "tooltip": "Essa condição verifica se o dispositivo selecionado está ativo/ligado.",
      "helpUrl": "",
      "message0": "Dispositivo %1 está ligado/ativo",
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
      "tooltip": "Essa condicão verifica se o dispositivo selecionado está inativo/desligado.",
      "helpUrl": "",
      "message0": "Dispositivo %1 está desligado/inativo",
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
      "tooltip": "Essa condição verifica se um detector de movimento está atualmente detectando movimento.",
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
    {
      "type": "condition_sensor_value",
      "tooltip": "Essa condição verifica o valor atual em um sensor.",
      "helpUrl": "",
      "message0": "Valor do sensor %1 é %2 %3",
      "args0": [
        {
          "type": "input_value",
          "name": "DEVICE",
          "check": [
            "device_MotionSensor",
            "device_Sensor"
          ],
        },
        {
          "type": "field_dropdown",
          "name": "CONDITION",
          "options": [
            ["igual a", "=="],
            ["maior que", ">"],
            ["menor que", "<"],
            ["maior ou igual a", ">="],
            ["menor ou igual a", "<="],
          ]
        },
        {
          "type": "field_number",
          "name": "VALUE",
          "value": 50,
          "precision": 0.1
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

  javascriptGenerator.forBlock['condition_event_time_between'] = function (block) {
    const start_time = block.getFieldValue('TIME0');
    const end_time = block.getFieldValue('TIME1');

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

  javascriptGenerator.forBlock['condition_sensor_value'] = (block, generator) => {
    const value = block.getFieldValue('VALUE');
    const device = generator.valueToCode(block, 'DEVICE', Order.ATOMIC);
    const conditionValue = block.getFieldValue('CONDITION');

    switch (conditionValue) {
      case '==':
        return [`number(get_device_state("${device}")) == number("${value}")`, Order.ATOMIC];
      case '>':
        return [`number(get_device_state("${device}")) > number("${value}")`, Order.ATOMIC];
      case '<':
        return [`number(get_device_state("${device}")) < number("${value}")`, Order.ATOMIC];
      case '>=':
        return [`number(get_device_state("${device}")) >= number("${value}")`, Order.ATOMIC];
      case '<=':
        return [`number(get_device_state("${device}")) <= number("${value}")`, Order.ATOMIC];
      default:
        return null;
    }

  };
}
