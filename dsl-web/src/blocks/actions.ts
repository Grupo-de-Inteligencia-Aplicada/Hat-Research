import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';
import { DeviceBlockTypes } from './devices';

export default function setupActionBlocks() {
  Blockly.defineBlocksWithJsonArray([
    {
      "type": "action_turn_on_device",
      "tooltip": "Essa ação vai tentar ligar o dispositivo selecionado.",
      "helpUrl": "",
      "message0": "Ligar %1",
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
      "tooltip": "Essa ação vai tentar desligar o dispositivo selecionado.",
      "helpUrl": "",
      "message0": "Desligar %1",
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
      "tooltip": "Essa ação vai tentar configurar a cor de uma lâmpada.",
      "helpUrl": "",
      "message0": "Configurar cor da luz %1 para %2",
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
    },
    {
      "type": "action_set_light_brightness",
      "tooltip": "Essa ação vai tentar configurar o brilho de uma lâmpada.",
      "helpUrl": "",
      "message0": "Configurar brilho da luz %1 para %2%",
      "args0": [
        {
          "type": "input_value",
          "name": "LIGHT_DEVICE",
          "check": "device_Light"
        },
        {
          "type": "field_slider",
          "name": "BRIGHTNESS",
          "value": 50,
          "min": 0,
          "max": 100,
          "precision": 0.01
        },
      ],
      "previousStatement": "action_block",
      "nextStatement": "action_block",
      "colour": 15
    },
    {
      "type": "action_wait",
      "tooltip": "Essa ação vai esperar o tempo selecionado antes de continuar executando as próximas ações.",
      "helpUrl": "",
      "message0": "Wait %1 seconds",
      "args0": [
        {
          "type": "field_slider",
          "name": "SECONDS",
          "value": 50,
          "min": 0,
          "max": 100,
          "precision": 0.01
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
  javascriptGenerator.forBlock['action_set_light_brightness'] = (block, generator) => {
    const device = generator.valueToCode(block, 'LIGHT_DEVICE', Order.ATOMIC);
    const bri_percent = block.getFieldValue('BRIGHTNESS');
    const bri = (bri_percent / 100) * 255;
    return `run set_light_brightness("${device}", ${bri})\n`;
  };
  javascriptGenerator.forBlock['action_wait'] = (block, _generator) => {
    const secs = block.getFieldValue('SECONDS');
    return `run wait(${secs})\n`;
  };
}
