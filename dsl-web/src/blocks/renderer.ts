
import * as Blockly from 'blockly/core';
import { ConnectionType } from 'blockly/core';

class CustomRenderer extends Blockly.zelos.Renderer {
  constructor(name: string) {
    super(name);
  }

  protected makeConstants_(): Blockly.zelos.ConstantProvider {
    return new CustomConstantProvider();
  }
}

class CustomConstantProvider extends Blockly.zelos.ConstantProvider {
  constructor() {
    super();

    this.CORNER_RADIUS = 1;
  }

  shapeFor(connection: Blockly.RenderedConnection) {
    let checks = connection.getCheck();
    if (!checks && connection.targetConnection) {
      checks = connection.targetConnection.getCheck();
    }
    let outputShape;
    switch (connection.type) {
      case ConnectionType.INPUT_VALUE:
      case ConnectionType.OUTPUT_VALUE:
        outputShape = connection.getSourceBlock().getOutputShape();
        // If the block has an output shape set, use that instead.
        if (outputShape !== null) {
          switch (outputShape) {
            case this.SHAPES.HEXAGONAL:
              return this.HEXAGONAL!;
            case this.SHAPES.ROUND:
              return this.ROUNDED!;
            case this.SHAPES.SQUARE:
              return this.SQUARED!;
          }
        }
        // Includes doesn't work in IE.
        if (checks && checks.includes('Boolean')) {
          return this.HEXAGONAL!;
        }
        if (checks && checks.includes('Number')) {
          return this.ROUNDED!;
        }
        if (checks && checks.includes('String')) {
          return this.ROUNDED!;
        }
        if (checks && checks.includes('event_block')) {
          return this.SQUARED!;
        }
        return this.ROUNDED!;
      case ConnectionType.PREVIOUS_STATEMENT:
      case ConnectionType.NEXT_STATEMENT:
        return this.NOTCH!;
      default:
        throw Error('Unknown type');
    }
  }
}

export function registerRenderer() {
  Blockly.blockRendering.register('custom_renderer', CustomRenderer);
}

