import * as Blockly from "blockly";
import {javascriptGenerator, Order} from 'blockly/javascript';

export default function defineBlocks() {
  Blockly.defineBlocksWithJsonArray([
    // Event Blocks
    {
      "type": "event_DoorOpenEvent",
      "message0": "Door Open Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a door is opened",
      "helpUrl": ""
    },
    {
      "type": "event_DoorCloseEvent",
      "message0": "Door Close Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a door is closed",
      "helpUrl": ""
    },
    {
      "type": "event_LightOnEvent",
      "message0": "Light On Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a light is turned on",
      "helpUrl": ""
    },
    {
      "type": "event_LightOffEvent",
      "message0": "Light Off Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a light is turned off",
      "helpUrl": ""
    },
    {
      "type": "event_PowerOutletOnEvent",
      "message0": "Power Outlet On Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a power outlet is turned on",
      "helpUrl": ""
    },
    {
      "type": "event_PowerOutletOffEvent",
      "message0": "Power Outlet Off Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a power outlet is turned off",
      "helpUrl": ""
    },
    {
      "type": "event_MotionSensorOnEvent",
      "message0": "Motion Sensor On Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a motion sensor is activated",
      "helpUrl": ""
    },
    {
      "type": "event_MotionSensorOffEvent",
      "message0": "Motion Sensor Off Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a motion sensor is deactivated",
      "helpUrl": ""
    },
    {
      "type": "event_SensorValueChangeEvent",
      "message0": "Sensor Value Change Event",
      "output": "Event",
      "colour": 20,
      "tooltip": "Triggered when a sensor value changes",
      "helpUrl": ""
    },

    // Automation Block with Mutator for Multiple Events
    // This block is defined separately below due to its complexity.

    // Condition Block
    {
      "type": "condition",
      "message0": "if %1",
      "args0": [
        {
          "type": "input_value",
          "name": "CONDITION"
        }
      ],
      "previousStatement": null,
      "nextStatement": null,
      "colour": 210,
      "tooltip": "Condition block",
      "helpUrl": ""
    },

    // Action Block
    {
      "type": "action",
      "message0": "run %1",
      "args0": [
        {
          "type": "input_value",
          "name": "ACTION"
        }
      ],
      "previousStatement": null,
      "nextStatement": null,
      "colour": 160,
      "tooltip": "Action block",
      "helpUrl": ""
    },

    // get_device Function Block
    {
      "type": "get_device",
      "message0": "get_device()",
      "output": "String",
      "colour": 65,
      "tooltip": "Returns the device that triggered the event",
      "helpUrl": ""
    },

    // get_integration Function Block
    {
      "type": "get_integration",
      "message0": "get_integration()",
      "output": "String",
      "colour": 65,
      "tooltip": "Returns the integration that sent the event",
      "helpUrl": ""
    },

    // event_date Function Block
    {
      "type": "event_date",
      "message0": "event_date()",
      "output": "String",
      "colour": 65,
      "tooltip": "Returns the date the event was emitted",
      "helpUrl": ""
    },

    // event_time Function Block
    {
      "type": "event_time",
      "message0": "event_time()",
      "output": null,
      "colour": 65,
      "tooltip": "Returns the time of the event",
      "helpUrl": ""
    },

    // time Function Block
    {
      "type": "time_function",
      "message0": "time( %1 )",
      "args0": [
        {
          "type": "input_value",
          "name": "TIME_STRING",
          "check": "String"
        }
      ],
      "output": null,
      "colour": 65,
      "tooltip": "Converts a string to Time type",
      "helpUrl": ""
    },

    // turn_on_device Function Block
    {
      "type": "turn_on_device",
      "message0": "turn_on_device( %1 )",
      "args0": [
        {
          "type": "input_value",
          "name": "DEVICE_ID",
          "check": "String"
        }
      ],
      "output": null,
      "colour": 65,
      "tooltip": "Turns on the device with the given ID",
      "helpUrl": ""
    },

    // turn_off_device Function Block
    {
      "type": "turn_off_device",
      "message0": "turn_off_device( %1 )",
      "args0": [
        {
          "type": "input_value",
          "name": "DEVICE_ID",
          "check": "String"
        }
      ],
      "output": null,
      "colour": 65,
      "tooltip": "Turns off the device with the given ID",
      "helpUrl": ""
    },

    // device_status Function Block
    {
      "type": "device_status",
      "message0": "device_status( %1 )",
      "args0": [
        {
          "type": "input_value",
          "name": "DEVICE_ID",
          "check": "String"
        }
      ],
      "output": null,
      "colour": 65,
      "tooltip": "Returns the status of the device",
      "helpUrl": ""
    },

    // set_device_temperature Function Block
    {
      "type": "set_device_temperature",
      "message0": "set_device_temperature( %1 , %2 )",
      "args0": [
        {
          "type": "input_value",
          "name": "DEVICE_ID",
          "check": "String"
        },
        {
          "type": "input_value",
          "name": "TEMPERATURE",
          "check": "Number"
        }
      ],
      "output": null,
      "colour": 65,
      "tooltip": "Sets the temperature of the device",
      "helpUrl": ""
    },

    // toggle_device Function Block
    {
      "type": "toggle_device",
      "message0": "toggle_device( %1 )",
      "args0": [
        {
          "type": "input_value",
          "name": "DEVICE_ID",
          "check": "String"
        }
      ],
      "output": null,
      "colour": 65,
      "tooltip": "Toggles the device state",
      "helpUrl": ""
    },

    // Constant String Block
    {
      "type": "const_string",
      "message0": "\" %1 \"",
      "args0": [
        {
          "type": "field_input",
          "name": "VALUE",
          "text": "string"
        }
      ],
      "output": "String",
      "colour": 160,
      "tooltip": "String constant",
      "helpUrl": ""
    },

    // Constant Number Block
    {
      "type": "const_number",
      "message0": "%1",
      "args0": [
        {
          "type": "field_number",
          "name": "VALUE",
          "value": 0
        }
      ],
      "output": "Number",
      "colour": 160,
      "tooltip": "Number constant",
      "helpUrl": ""
    },

    // Constant Boolean Block
    {
      "type": "const_boolean",
      "message0": "%1",
      "args0": [
        {
          "type": "field_dropdown",
          "name": "VALUE",
          "options": [
            ["true", "true"],
            ["false", "false"]
          ]
        }
      ],
      "output": "Boolean",
      "colour": 160,
      "tooltip": "Boolean constant",
      "helpUrl": ""
    },

    // Constant Null Block
    {
      "type": "const_null",
      "message0": "null",
      "output": null,
      "colour": 160,
      "tooltip": "Null constant",
      "helpUrl": ""
    },

    // Arithmetic Operation Block
    {
      "type": "arithmetic_operation",
      "message0": "%1 %2 %3",
      "args0": [
        {
          "type": "input_value",
          "name": "LEFT"
        },
        {
          "type": "field_dropdown",
          "name": "OP",
          "options": [
            ["+", "ADD"],
            ["-", "SUB"],
            ["*", "MUL"],
            ["/", "DIV"]
          ]
        },
        {
          "type": "input_value",
          "name": "RIGHT"
        }
      ],
      "output": null,
      "colour": 230,
      "tooltip": "Arithmetic operation",
      "helpUrl": ""
    },

    // Comparison Operation Block
    {
      "type": "comparison_operation",
      "message0": "%1 %2 %3",
      "args0": [
        {
          "type": "input_value",
          "name": "LEFT"
        },
        {
          "type": "field_dropdown",
          "name": "OP",
          "options": [
            [">", "GREATER"],
            ["<", "LESSER"],
            ["==", "EQUALS"]
          ]
        },
        {
          "type": "input_value",
          "name": "RIGHT"
        }
      ],
      "output": "Boolean",
      "colour": 230,
      "tooltip": "Comparison operation",
      "helpUrl": ""
    },

    // Logical Operation Block
    {
      "type": "logical_operation",
      "message0": "%1 %2 %3",
      "args0": [
        {
          "type": "input_value",
          "name": "LEFT",
          "check": "Boolean"
        },
        {
          "type": "field_dropdown",
          "name": "OP",
          "options": [
            ["and", "AND"],
            ["or", "OR"]
          ]
        },
        {
          "type": "input_value",
          "name": "RIGHT",
          "check": "Boolean"
        }
      ],
      "output": "Boolean",
      "colour": 230,
      "tooltip": "Logical operation",
      "helpUrl": ""
    },

    // Logical NOT Block
    {
      "type": "logical_not",
      "message0": "not %1",
      "args0": [
        {
          "type": "input_value",
          "name": "VALUE",
          "check": "Boolean"
        }
      ],
      "output": "Boolean",
      "colour": 230,
      "tooltip": "Logical NOT",
      "helpUrl": ""
    }
  ]);

  Blockly.Blocks['automation'] = {
    init: function () {
      this.appendDummyInput()
        .appendField("automation")
        .appendField(new Blockly.FieldTextInput("Automation Name"), "NAME");
      this.appendDummyInput()
        .appendField("when");
      this.eventCount_ = 1;
      this.updateShape_();
      this.appendDummyInput()
        .appendField("do");
      this.appendStatementInput("BODY")
        .setCheck(null);
      this.setColour(230);
      this.setTooltip("Define an automation with multiple events");
      this.setHelpUrl("");
      //this.setMutator(Blockly.Blocks['automation_event']);
    },
    mutationToDom: function () {
      var container = document.createElement('mutation');
      container.setAttribute('event_count', this.eventCount_);
      return container;
    },
    domToMutation: function (xmlElement) {
      this.eventCount_ = parseInt(xmlElement.getAttribute('event_count'), 10);
      this.updateShape_();
    },
    decompose: function (workspace) {
      var containerBlock = workspace.newBlock('automation_container');
      containerBlock.initSvg();
      var connection = containerBlock.getInput('STACK').connection;
      for (var i = 0; i < this.eventCount_; i++) {
        var eventBlock = workspace.newBlock('automation_event');
        eventBlock.initSvg();
        connection.connect(eventBlock.previousConnection);
        connection = eventBlock.nextConnection;
      }
      return containerBlock;
    },
    compose: function (containerBlock) {
      var eventBlock = containerBlock.getInputTargetBlock('STACK');
      var connections = [];
      while (eventBlock) {
        connections.push(eventBlock.valueConnection_);
        eventBlock = eventBlock.nextConnection &&
          eventBlock.nextConnection.targetBlock();
      }
      this.eventCount_ = connections.length;
      this.updateShape_();
      // Reconnect child blocks.
      for (var i = 0; i < this.eventCount_; i++) {
        var connection = this.getInput('EVENT' + i).connection;
        if (connections[i]) {
          connection.connect(connections[i]);
        }
      }
    },
    saveConnections: function (containerBlock) {
      var eventBlock = containerBlock.getInputTargetBlock('STACK');
      var i = 0;
      while (eventBlock) {
        var input = this.getInput('EVENT' + i);
        eventBlock.valueConnection_ = input && input.connection.targetConnection;
        i++;
        eventBlock = eventBlock.nextConnection &&
          eventBlock.nextConnection.targetBlock();
      }
    },
    updateShape_: function () {
      // Remove all event inputs
      var i = 0;
      while (this.getInput('EVENT' + i)) {
        this.removeInput('EVENT' + i);
        i++;
      }
      // Add new event inputs
      for (var i = 0; i < this.eventCount_; i++) {
        this.appendValueInput('EVENT' + i)
          .setCheck('Event')
          .appendField(i === 0 ? 'event' : 'and');
      }
      // Move the 'do' and 'BODY' inputs to the end
      if (this.getInput('BODY')) {
        this.moveInputBefore('BODY', null);
      }
    }
  };

  // Mutator blocks for 'automation'
  Blockly.Blocks['automation_container'] = {
    init: function () {
      this.appendDummyInput()
        .appendField('events');
      this.appendStatementInput('STACK');
      this.setColour(230);
      this.setTooltip('');
      this.contextMenu = false;
    }
  };

  Blockly.Blocks['automation_event'] = {
    init: function () {
      this.appendDummyInput()
        .appendField('event');
      this.setPreviousStatement(true);
      this.setNextStatement(true);
      this.setColour(230);
      this.setTooltip('');
      this.contextMenu = false;
    }
  };

  // Define the 'echo' block with mutator
  Blockly.Blocks['echo'] = {
    init: function () {
      this.appendDummyInput()
        .appendField("echo");
      this.setColour(160);
      this.setTooltip("Prints all the parameters to the console");
      this.setHelpUrl("");
      //this.setMutator(Blockly.Blocks['echo_mutator']);
      this.parametersCount_ = 0;
      this.updateShape_();
      this.setOutput(true);
    },
    mutationToDom: function () {
      var container = document.createElement('mutation');
      container.setAttribute('parameters', this.parametersCount_);
      return container;
    },
    domToMutation: function (xmlElement) {
      this.parametersCount_ = parseInt(xmlElement.getAttribute('parameters'), 10);
      this.updateShape_();
    },
    decompose: function (workspace) {
      var containerBlock = workspace.newBlock('echo_container');
      containerBlock.initSvg();
      var connection = containerBlock.getInput('STACK').connection;
      for (var i = 0; i < this.parametersCount_; i++) {
        var paramBlock = workspace.newBlock('echo_mutator');
        paramBlock.initSvg();
        connection.connect(paramBlock.previousConnection);
        connection = paramBlock.nextConnection;
      }
      return containerBlock;
    },
    compose: function (containerBlock) {
      var paramBlock = containerBlock.getInputTargetBlock('STACK');
      var connections = [];
      while (paramBlock) {
        connections.push(paramBlock.valueConnection_);
        paramBlock = paramBlock.nextConnection &&
          paramBlock.nextConnection.targetBlock();
      }
      this.parametersCount_ = connections.length;
      this.updateShape_();
      // Reconnect child blocks.
      for (var i = 0; i < this.parametersCount_; i++) {
        var connection = this.getInput('PARAM' + i).connection;
        if (connections[i]) {
          connection.connect(connections[i]);
        }
      }
    },
    saveConnections: function (containerBlock) {
      var paramBlock = containerBlock.getInputTargetBlock('STACK');
      var i = 0;
      while (paramBlock) {
        var input = this.getInput('PARAM' + i);
        paramBlock.valueConnection_ = input && input.connection.targetConnection;
        i++;
        paramBlock = paramBlock.nextConnection &&
          paramBlock.nextConnection.targetBlock();
      }
    },
    updateShape_: function () {
      // Remove all inputs.
      if (this.parametersCount_) {
        if (this.getInput('EMPTY')) {
          this.removeInput('EMPTY');
        }
      } else {
        if (!this.getInput('EMPTY')) {
          this.appendDummyInput('EMPTY')
            .appendField("(no parameters)");
        }
      }
      // Rebuild block inputs.
      for (var i = 0; i < this.parametersCount_; i++) {
        if (!this.getInput('PARAM' + i)) {
          var input = this.appendValueInput('PARAM' + i)
            .setCheck(null)
            .appendField(i === 0 ? 'with' : 'and');
        }
      }
      // Remove inputs that are no longer needed.
      while (this.getInput('PARAM' + i)) {
        this.removeInput('PARAM' + i);
        i++;
      }
    }
  };

  // Mutator blocks for 'echo'
  Blockly.Blocks['echo_container'] = {
    init: function () {
      this.appendDummyInput()
        .appendField('parameters');
      this.appendStatementInput('STACK');
      this.setColour(160);
      this.setTooltip('');
      this.contextMenu = false;
    }
  };

  Blockly.Blocks['echo_mutator'] = {
    init: function () {
      this.appendDummyInput()
        .appendField('parameter');
      this.setPreviousStatement(true);
      this.setNextStatement(true);
      this.setColour(160);
      this.setTooltip('');
      this.contextMenu = false;
    }
  };

  javascriptGenerator.forBlock['automation'] = function(block) {
    var name = block.getFieldValue('NAME');
    var events = [];
    for (var i = 0; i < block.eventCount_; i++) {
      var event = javascriptGenerator.valueToCode(block, 'EVENT' + i, Order.NONE) || 'null';
      events.push(event);
    }
    var eventList = events.join(', ');
    var statements_body = javascriptGenerator.statementToCode(block, 'BODY');
    var code = 'automation "' + name + '" (' + eventList + ') {\n' + statements_body + '}\n\n';
    return code;
  };

  // Event Code Generators
  javascriptGenerator.forBlock['event_DoorOpenEvent'] = function(block) {
    return ['DoorOpenEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_DoorCloseEvent'] = function(block) {
    return ['DoorCloseEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_LightOnEvent'] = function(block) {
    return ['LightOnEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_LightOffEvent'] = function(block) {
    return ['LightOffEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_PowerOutletOnEvent'] = function(block) {
    return ['PowerOutletOnEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_PowerOutletOffEvent'] = function(block) {
    return ['PowerOutletOffEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_MotionSensorOnEvent'] = function(block) {
    return ['MotionSensorOnEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_MotionSensorOffEvent'] = function(block) {
    return ['MotionSensorOffEvent', Order.ATOMIC];
  };

  javascriptGenerator.forBlock['event_SensorValueChangeEvent'] = function(block) {
    return ['SensorValueChangeEvent', Order.ATOMIC];
  };

  // Condition Code Generator
  javascriptGenerator.forBlock['condition'] = function(block) {
    var value_condition = javascriptGenerator.valueToCode(block, 'CONDITION', Order.NONE) || 'false';
    var code = 'if ' + value_condition + '\n';
    return code;
  };

  // Action Code Generator
  javascriptGenerator.forBlock['action'] = function(block) {
    var value_action = javascriptGenerator.valueToCode(block, 'ACTION', Order.NONE) || '';
    var code = 'run ' + value_action + '\n';
    return code;
  };

  // get_device Code Generator
  javascriptGenerator.forBlock['get_device'] = function(block) {
    var code = 'get_device()';
    return [code, Order.FUNCTION_CALL];
  };

  // get_integration Code Generator
  javascriptGenerator.forBlock['get_integration'] = function(block) {
    var code = 'get_integration()';
    return [code, Order.FUNCTION_CALL];
  };

  // event_date Code Generator
  javascriptGenerator.forBlock['event_date'] = function(block) {
    var code = 'event_date()';
    return [code, Order.FUNCTION_CALL];
  };

  // event_time Code Generator
  javascriptGenerator.forBlock['event_time'] = function(block) {
    var code = 'event_time()';
    return [code, Order.FUNCTION_CALL];
  };

  // time Function Code Generator
  javascriptGenerator.forBlock['time_function'] = function(block) {
    var value_time_string = javascriptGenerator.valueToCode(block, 'TIME_STRING', Order.NONE) || '""';
    var code = 'time(' + value_time_string + ')';
    return [code, Order.FUNCTION_CALL];
  };

  // turn_on_device Code Generator
  javascriptGenerator.forBlock['turn_on_device'] = function(block) {
    var value_device_id = javascriptGenerator.valueToCode(block, 'DEVICE_ID', Order.NONE) || '""';
    var code = 'turn_on_device(' + value_device_id + ')';
    return [code, Order.FUNCTION_CALL];
  };

  // turn_off_device Code Generator
  javascriptGenerator.forBlock['turn_off_device'] = function(block) {
    var value_device_id = javascriptGenerator.valueToCode(block, 'DEVICE_ID', Order.NONE) || '""';
    var code = 'turn_off_device(' + value_device_id + ')';
    return [code, Order.FUNCTION_CALL];
  };

  // device_status Code Generator
  javascriptGenerator.forBlock['device_status'] = function(block) {
    var value_device_id = javascriptGenerator.valueToCode(block, 'DEVICE_ID', Order.NONE) || '""';
    var code = 'device_status(' + value_device_id + ')';
    return [code, Order.FUNCTION_CALL];
  };

  // set_device_temperature Code Generator
  javascriptGenerator.forBlock['set_device_temperature'] = function(block) {
    var value_device_id = javascriptGenerator.valueToCode(block, 'DEVICE_ID', Order.NONE) || '""';
    var value_temperature = javascriptGenerator.valueToCode(block, 'TEMPERATURE', Order.NONE) || '0';
    var code = 'set_device_temperature(' + value_device_id + ', ' + value_temperature + ')';
    return [code, Order.FUNCTION_CALL];
  };

  // toggle_device Code Generator
  javascriptGenerator.forBlock['toggle_device'] = function(block) {
    var value_device_id = javascriptGenerator.valueToCode(block, 'DEVICE_ID', Order.NONE) || '""';
    var code = 'toggle_device(' + value_device_id + ')';
    return [code, Order.FUNCTION_CALL];
  };

  // Constants Code Generators

  // String Constant
  javascriptGenerator.forBlock['const_string'] = function(block) {
    var value = block.getFieldValue('VALUE');
    var code = '"' + value + '"';
    return [code, Order.ATOMIC];
  };

  // Number Constant
  javascriptGenerator.forBlock['const_number'] = function(block) {
    var number_value = block.getFieldValue('VALUE');
    var code = number_value;
    return [code, Order.ATOMIC];
  };

  // Boolean Constant
  javascriptGenerator.forBlock['const_boolean'] = function(block) {
    var dropdown_value = block.getFieldValue('VALUE');
    var code = dropdown_value;
    return [code, Order.ATOMIC];
  };

  // Null Constant
  javascriptGenerator.forBlock['const_null'] = function(block) {
    var code = 'null';
    return [code, Order.ATOMIC];
  };

  // Operations Code Generators

  // Arithmetic Operation
  javascriptGenerator.forBlock['arithmetic_operation'] = function(block) {
    var value_left = javascriptGenerator.valueToCode(block, 'LEFT', Order.NONE) || '0';
    var dropdown_op = block.getFieldValue('OP');
    var value_right = javascriptGenerator.valueToCode(block, 'RIGHT', Order.NONE) || '0';
    var operator = {'ADD': '+', 'SUB': '-', 'MUL': '*', 'DIV': '/'}[dropdown_op];
    var code = value_left + ' ' + operator + ' ' + value_right;
    return [code, javascriptGenerator.ORDER_NONE];
  };

  // Comparison Operation
  javascriptGenerator.forBlock['comparison_operation'] = function(block) {
    var value_left = javascriptGenerator.valueToCode(block, 'LEFT', Order.NONE) || '0';
    var dropdown_op = block.getFieldValue('OP');
    var value_right = javascriptGenerator.valueToCode(block, 'RIGHT', Order.NONE) || '0';
    var operator = {'GREATER': '>', 'LESSER': '<', 'EQUALS': '=='}[dropdown_op];
    var code = value_left + ' ' + operator + ' ' + value_right;
    return [code, Order.RELATIONAL];
  };

  // Logical Operation
  javascriptGenerator.forBlock['logical_operation'] = function(block) {
    var value_left = javascriptGenerator.valueToCode(block, 'LEFT', Order.NONE) || 'false';
    var dropdown_op = block.getFieldValue('OP');
    var value_right = javascriptGenerator.valueToCode(block, 'RIGHT', Order.NONE) || 'false';
    var operator = {'AND': 'and', 'OR': 'or'}[dropdown_op];
    var code = value_left + ' ' + operator + ' ' + value_right;
    return [code, Order.LOGICAL_AND];
  };

  // Logical NOT
  javascriptGenerator.forBlock['logical_not'] = function(block) {
    var value = javascriptGenerator.valueToCode(block, 'VALUE', Order.NONE) || 'false';
    var code = 'not ' + value;
    return [code, Order.LOGICAL_NOT];
  };

  // echo Code Generator
  javascriptGenerator.forBlock['echo'] = function(block) {
    var code = 'echo(';
    var params = [];
    for (var i = 0; i < block.parametersCount_; i++) {
      var param = javascriptGenerator.valueToCode(block, 'PARAM' + i, Order.NONE) || 'null';
      params.push(param);
    }
    code += params.join(', ') + ')';
    return [code, Order.FUNCTION_CALL];
  };

  return javascriptGenerator;
}
