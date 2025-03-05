
import * as Blockly from 'blockly/core';
import { ConnectionType } from 'blockly/core';
import * as Colors from './colors';

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

  override getCSS_(selector: string) {
    return [
      // Text.
      `${selector} .blocklyText,`,
      `${selector} .blocklyFlyoutLabelText {`,
      `font: ${this.FIELD_TEXT_FONTWEIGHT} ${this.FIELD_TEXT_FONTSIZE}` +
        `pt ${this.FIELD_TEXT_FONTFAMILY};`,
      `}`,

      `${selector} .blocklyTextInputBubble textarea {`,
      `font-weight: normal;`,
      `}`,

      // Fields.
      `${selector} .blocklyText {`,
      `fill: ${Colors.TEXT_COLOR};`,
      `}`,
      `${selector} .blocklyNonEditableText>rect:not(.blocklyDropdownRect),`,
      `${selector} .blocklyEditableText>rect:not(.blocklyDropdownRect) {`,
      `fill: ${Colors.INPUT_COLOR};`,
      `}`,
      `${selector} .blocklyNonEditableText>text,`,
      `${selector} .blocklyEditableText>text,`,
      `${selector} .blocklyNonEditableText>g>text,`,
      `${selector} .blocklyEditableText>g>text {`,
      `fill: ${Colors.INPUT_TEXT_COLOR};`,
      `}`,

      // Flyout labels.
      `${selector} .blocklyFlyoutLabelText {`,
      `fill: #575E75;`,
      `}`,

      // Bubbles.
      `${selector} .blocklyText.blocklyBubbleText {`,
      `fill: #575E75;`,
      `}`,

      // Editable field hover.
      `${selector} .blocklyDraggable:not(.blocklyDisabled)`,
      ` .blocklyEditableText:not(.editing):hover>rect,`,
      `${selector} .blocklyDraggable:not(.blocklyDisabled)`,
      ` .blocklyEditableText:not(.editing):hover>.blocklyPath {`,
      `stroke: #fff;`,
      `stroke-width: 2;`,
      `}`,

      // Text field input.
      `${selector} .blocklyHtmlInput {`,
      `font-family: ${this.FIELD_TEXT_FONTFAMILY};`,
      `font-weight: ${this.FIELD_TEXT_FONTWEIGHT};`,
      `color: #575E75;`,
      `}`,

      // Dropdown field.
      `${selector} .blocklyDropdownText {`,
      `fill: ${Colors.DROPDOWN_TEXT_COLOR} !important;`,
      `}`,

      // Widget and Dropdown Div
      `${selector}.blocklyWidgetDiv .goog-menuitem,`,
      `${selector}.blocklyDropDownDiv .goog-menuitem {`,
      `font-family: ${this.FIELD_TEXT_FONTFAMILY};`,
      `}`,
      `${selector}.blocklyDropDownDiv .goog-menuitem-content {`,
      `color: ${Colors.TEXT_COLOR};`,
      `}`,

      // Connection highlight.
      `${selector} .blocklyHighlightedConnectionPath {`,
      `stroke: ${this.SELECTED_GLOW_COLOUR};`,
      `}`,

      // Disabled outline paths.
      `${selector} .blocklyDisabled > .blocklyOutlinePath {`,
      `fill: url(#blocklyDisabledPattern${this.randomIdentifier})`,
      `}`,

      // Insertion marker.
      `${selector} .blocklyInsertionMarker>.blocklyPath {`,
      `fill-opacity: ${this.INSERTION_MARKER_OPACITY};`,
      `stroke: none;`,
      `}`,
    ];
  }

}

export function registerRenderer() {
  Blockly.blockRendering.register('custom_renderer', CustomRenderer);
}

