import * as Blockly from "blockly";
import setupEventBlocks from './events';
import generateToolbox from "./toolbox";
import setupDeviceBlocks, { getIconFor, getLabelFor } from "./devices";
import setupActionBlocks from './actions';
import setupConditionBlocks from "./conditions";
import type { Device, HatApi, RuntimeEvent } from "../services/api";
import setupAutomationBlock from "./automation";
import { validateTime } from "../utils";

let globalWorkspace: Blockly.Workspace | undefined;

function setupExtensions() {
  Blockly.Extensions.register(
    'automation_name_validator',
    function () { // this refers to the block that the extension is being run on
      var thisBlock = (this as any) as Blockly.Block;
      thisBlock.getField('NAME')?.setValidator(name => {
        if (globalWorkspace) {
          const allAutomations = globalWorkspace.getBlocksByType('automation')
            .concat(globalWorkspace.getBlocksByType('automation_time_based')) as Blockly.Block[];

          const allNames = allAutomations
            .filter(block => block != thisBlock)
            .map(block => block.getFieldValue('NAME') as string);

          console.log(allNames, name);
          if (allNames.includes(name)) {
            let idx = 1;
            while (allNames.includes(`Automação ${idx}`)) {
              idx += 1;
            }
            return `Automação ${idx}`;
          } else {
            return name;
          }
        }
      });
    });
  Blockly.Extensions.register(
    'time_validator',
    function () { // this refers to the block that the extension is being run on
      var thisBlock = (this as any) as Blockly.Block;
      let idx = 0;
      while (true) {
        const fieldName = `TIME${idx}`;
        const field = thisBlock.getField(fieldName);
        if (field) {
          field.setValidator(validateTime);
        } else {
          break;
        }
        idx += 1;
      }
    });
}

export function setupBlockly(api: HatApi, devices: Device[], possibleEvents: RuntimeEvent[]) {

  setupExtensions();

  const toolbox = generateToolbox(possibleEvents, devices);
  setupEventBlocks(possibleEvents);
  setupDeviceBlocks(devices);
  setupActionBlocks();
  setupConditionBlocks();
  setupAutomationBlock();

  let options = {
    toolbox,
    trashcan: true,
    renderer: 'zelos',
  };

  let workspace = Blockly.inject("blocklyDiv", options);

  globalWorkspace = workspace;

  const customTooltip = function (div, element) {
    if (element instanceof Blockly.BlockSvg) {
      // You can access the block being moused over.
      // Here we get the color of the block to set the background color.
      div.style.backgroundColor = element.getColour();
    }
    const text = document.createElement('pre');


    const blockType = element.type as string;

    if (blockType.startsWith('device_')) {
      const deviceId = blockType.substring("device_".length);

      text.innerHTML = "Carregando...";

      (async () => {
        try {
          const device = await api.getDevice(deviceId);
          text.innerHTML = "";
          text.innerHTML += `Nome: ${device.name}\n`;
          text.innerHTML += `Tipo: ${getLabelFor(device.typ)}\n`;
          text.innerHTML += `Valor atual: ${device.state}\n`;
        } catch (e) {
          console.error('Request failed:', e);
          text.innerHTML = "Falha ao fazer requisição!"
        }
      })();
    } else if (blockType.startsWith("event_")) {
      const eventType = blockType.substring("event_".length);

      for (const event of possibleEvents) {
        if (event.event == eventType) {
          text.innerHTML = `Evento relacionado aos dispotivos\ndo tipo: ${getIconFor(event.relatedDeviceType)} ${getLabelFor(event.relatedDeviceType)}`;
        }
      }
    } else {
      const tooltip = Blockly.Tooltip.getTooltipOfObject(element);
      text.innerHTML = tooltip;
    }

    if (text.innerHTML.trim().length != 0) {
      text.style.padding = "15px";
      text.style.fontFamily = 'inherit';
      text.style.whiteSpace = 'pre-wrap';
      text.style.maxWidth = '400px';
      text.style.overflow = 'none';
    }

    const container = document.createElement('div');
    container.style.display = 'flex';
    container.appendChild(text);
    div.appendChild(container);
  };
  // Register the function in the Blockly.Tooltip so that Blockly calls it
  // when needed.
  Blockly.Tooltip.setCustomTooltip(customTooltip);

  return workspace;
}
