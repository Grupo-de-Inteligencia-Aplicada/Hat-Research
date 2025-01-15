import * as Blockly from 'blockly';
import { javascriptGenerator, Order } from 'blockly/javascript';
import { stripLines } from '../utils';

export default function setupAutomationBlock() {
  Blockly.defineBlocksWithJsonArray([
    {
      "type": "automation",
      "tooltip": stripLines(`Esse bloco representa uma automação em sua casa.
                  Uma automação é composta por uma série de ações que vão ser executadas
                  quando um determinado evento acontecer.
                  Automações podem conter também uma condição. Dessa forma, uma automação só
                  vai executar as suas ações se essa condição for verdadeira no momento em
                  que ela for executada.

                  Importante: condições são OPCIONAIS, ou seja, não é necessário colocar uma.
                  Quando uma automação não têm condições, ela vai sempre executar as ações assim que for disparada.

                  A execução de uma automação ocorre da seguinte forma:
                  
                  Evento que dispara ela acontece → Verifica a condição → Se verdadeira, executa as ações, uma a uma.`),
      "helpUrl": "",
      "message0": "Automação %1 %2 %3 Disparar quando %4 %5 %6 E se %7 %8 %9 %10 Então, fazer %11 %12",
      "args0": [
        {
          "type": "field_input",
          "name": "NAME",
          "text": "Nome da automação"
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
      "colour": 210,
      "extensions": ["automation_name_validator"],
    },
    {
      "type": "automation_time_based",
      "tooltip": stripLines(`Essa é uma automação baseada em tempo. Diferente da automação anterior
                             essa automação não precisa de um evento para ser disparada, mas sim, de um horário do dia.
                             Dessa forma, sempre que o horário chegar no horário especificado, a automação será disparada.

                             Exemplo: Uma automação para desligar as luzes às 23:00.`),
      "helpUrl": "",
      "message0": "Automação %1 %2 %3 Disparar às %4 %5 %6 E se %7 %8 %9 %10 Então, fazer %11 %12",
      "args0": [
        {
          "type": "field_input",
          "name": "NAME",
          "text": "Nome da automação"
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
          "type": "field_input",
          "name": "TIME0",
          "text": "18:00"
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
      "colour": 210,
      "extensions": ["time_validator", "automation_name_validator"],
    },
  ]);

  javascriptGenerator.forBlock['automation'] = (block, generator) => {
    const name = block.getFieldValue('NAME');
    const eventValue = generator.valueToCode(block, 'EVENT', Order.ATOMIC).trim();
    const event = eventValue.length == 0 ? "None" : eventValue;
    const condition = generator.valueToCode(block, 'CONDITIONS', Order.ATOMIC);
    const body = generator.statementToCode(block, 'ACTIONS');
    return 'automation "' + name + '" (' + event + ') {' + (condition ? '\n  if ' + condition : '') + '\n' + body + '}\n\n';
  };

  javascriptGenerator.forBlock['automation_time_based'] = (block, generator) => {
    const name = block.getFieldValue('NAME');
    const time = block.getFieldValue('TIME0');
    const condition = generator.valueToCode(block, 'CONDITIONS', Order.ATOMIC);
    const body = generator.statementToCode(block, 'ACTIONS');
    return 'automation "' + name + '" (ClockTickEvent) {\n  if event_time() == time("' + time + '")' + (condition ? '\n  if ' + condition : '') + '\n' + body + '}\n\n';
  };
}

