import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';
import { DeviceBlockTypes } from './devices';

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
          "check": DeviceBlockTypes,
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
          "check": DeviceBlockTypes,
        }
      ],
      "previousStatement": "action_block",
      "nextStatement": "action_block",
      "colour": 10
    },
    {
      "type": "action_set_light_color",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Set light %1 color to %2",
      "args0": [
        {
          "type": "input_value",
          "name": "LIGHT_DEVICE",
          "check": "device_Light"
        },
        {
          "type": "field_colour_hsv_sliders",
          "name": "COLOR",
          "style": 'colour_blocks',
        },
      ],
      "previousStatement": "action_block",
      "nextStatement": "action_block",
      "colour": 15
    }

  ]);

  javascriptGenerator.forBlock['action_turn_on_device'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);
    return `run turn_on_device("${device}")\n`;
  };
  javascriptGenerator.forBlock['action_turn_off_device'] = (block, generator) => {
    const device = generator.valueToCode(block, 'NAME', Order.ATOMIC);
    return `run turn_off_device("${device}")\n`;
  };
  javascriptGenerator.forBlock['action_set_light_color'] = (block, generator) => {
    const device = generator.valueToCode(block, 'LIGHT_DEVICE', Order.ATOMIC);
    const color = block.getFieldValue('COLOR');
    return `run set_light_color("${device}", "${color}")\n`;
  };
}
