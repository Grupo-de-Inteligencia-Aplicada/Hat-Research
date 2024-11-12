const AutomationBlock = {
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
      "name": "NAME"
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
                    

export default AutomationBlock;
