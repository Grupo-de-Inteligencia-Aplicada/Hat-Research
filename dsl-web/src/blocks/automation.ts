import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';

export default function setupAutomationBlock() {
  Blockly.defineBlocksWithJsonArray([
    {
      "type": "automation",
      "tooltip": "",
      "helpUrl": "",
      "message0": "Automation %1 %2 %3 Trigger when %4 %5 %6 And if %7 %8 %9 %10 Then run %11 %12",
      "args0": [
        {
          "type": "field_input",
          "name": "NAME",
          "text": "Automation name"
        },
        {
          "type": "input_dummy",
          "name": ""
        },
        {
          "type": "input_end_row",
          "name": ""
        },
        {
          "type": "input_dummy",
          "name": ""
        },
        {
          "type": "input_value",
          "name": "EVENT",
          "check": "event_block"
        },
        {
          "type": "input_end_row",
          "name": ""
        },
        {
          "type": "input_dummy",
          "name": ""
        },
        {
          "type": "input_end_row",
          "name": ""
        },
        {
          "type": "input_value",
          "name": "CONDITIONS",
          "check": "Boolean"
        },
        {
          "type": "input_end_row",
          "name": ""
        },
        {
          "type": "input_dummy",
          "name": ""
        },
        {
          "type": "input_statement",
          "name": "ACTIONS",
          "check": "action_block"
        }
      ],
      "colour": 210
    }
  ]);

  javascriptGenerator.forBlock['automation'] = (block, generator) => {
    const name = block.getFieldValue('NAME');
    const event = generator.valueToCode(block, 'EVENT', Order.ATOMIC);
    const condition = generator.valueToCode(block, 'CONDITIONS', Order.ATOMIC);
    const body = generator.statementToCode(block, 'ACTIONS');
    return 'automation "' + name + '" (' + event + ') {' + (condition ? '\n  if ' + condition : '') + '\n' + body + '}\n\n';
  };
}

