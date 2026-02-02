  export interface InputNodeValue {
    value: string;
  }

  export interface DropdownNodeValue {
    models: string[];
    selectedIndex: number;
  }

  export interface ToggleNodeValue {
    checked: boolean;
  }

  export interface NumberNodeValue {
    value: number;
    min: number;
    max: number;
    step: number;
    isFloat: boolean;
  }


  export type ComfyUINode =
    | { type: "InputNode"; title: string; value: InputNodeValue }
    | { type: "DropdownNode"; title: string; value: DropdownNodeValue }
    | { type: "ToggleNode"; title: string; value: ToggleNodeValue }
    | { type: "NumberNode"; title: string; value: NumberNodeValue };
