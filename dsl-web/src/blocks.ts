import * as Blockly from "blockly";
import { javascriptGenerator, Order } from 'blockly/javascript';

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
    {
      "type": "echo",
      "message0": "echo %1",
      "args0": [
        {
          "type": "input_value",
          "name": "ACTION"
        }
      ],
      "output": "String",
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

  // AUTOMATION BLOCK

  // Define the 'automation' block with mutator for multiple events
  Blockly.Blocks['automation'] = {
    init: function () {
      this.appendDummyInput()
        .appendField("Automation")
        .appendField(new Blockly.FieldTextInput("Automation Name"), "NAME");

      this.appendValueInput('EVENT')
        .setCheck('Event')
        .appendField('on');

      this.appendDummyInput()
        .appendField();

      this.appendStatementInput("BODY")
        .setCheck(null);

      this.setColour(230);

      this.setTooltip("Define an automation with multiple events");
    },
  };


  // AUTOMATION BLOCK

}
