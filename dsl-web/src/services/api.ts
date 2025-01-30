
export const DeviceTypes = [
  "Dummy",
  "DoorSensor",
  "Light",
  "Sensor",
  "Switch",
  "MotionSensor",
  "Button",
  "Unknown",
] as const;

export type DeviceType = typeof DeviceTypes[number];

export const deviceTypePriorities: Record<DeviceType, number> = {
  DoorSensor: 3,
  Switch: 3,
  MotionSensor: 3,
  Button: 3,
  Light: 2,
  Sensor: 1,
  Dummy: 0,
  Unknown: 0
};

export interface Device {
  integration: string;
  id: string;
  name: string | null;
  typ: DeviceType;
  state: string | null;
}

export interface RuntimeEvent {
  event: string;
  description: string;
  relatedDeviceType: DeviceType,
};

export interface ApiError {
  code: number,
  errors: {
    description: string
  }[]
}

export class HatApi {
  private baseUrl: string;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  private async request(
    endpoint: string,
    method: string,
    body?: any,
    headers: Record<string, string> = {}
  ): Promise<any> {
    const url = `${this.baseUrl}${endpoint}`;
    const options: RequestInit = {
      method,
      headers: {
        'Content-Type': 'application/json',
        ...headers
      },
      body,
    };

    let response;
    try {
      response = await fetch(url, options);
    } catch (e) {
      console.error(`Failed to fetch ${endpoint}:`, e)
      let error: ApiError = {
        code: 408,
        errors: [{ description: String(e) }]
      };
      throw error;
    }

    if (!response.ok) {
      let error: ApiError = {
        code: response.status,
        errors: [{ description: 'Request failed' }]
      };
      try {
        const json = await response.json();
        error.errors = json.errors;
      } catch (_) { }
      throw error;
    }

    return await response.json();
  }

  async get(endpoint: string, headers: Record<string, string> = {}): Promise<any> {
    return this.request(endpoint, 'GET', undefined, headers);
  }

  async post(endpoint: string, body: any, headers: Record<string, string> = {}): Promise<any> {
    return this.request(endpoint, 'POST', body, headers);
  }

  async put(endpoint: string, body: any, headers: Record<string, string> = {}): Promise<any> {
    return this.request(endpoint, 'PUT', body, headers);
  }

  async delete(endpoint: string, headers: Record<string, string> = {}): Promise<any> {
    return this.request(endpoint, 'DELETE', undefined, headers);
  }

  async listDevices(): Promise<Device[]> {
    const devices = await this.get("/devices");
    const filteredDevices = (devices as Device[]).filter((dev) => !isDeviceBlacklisted(dev));
    return filteredDevices;
  }

  async getDevice(id: string): Promise<Device> {
    const device = await this.get("/device?id=" + encodeURI(id));
    return device;
  }

  async listPossibleEvents(): Promise<RuntimeEvent[]> {
    const events = await this.get("/possible_events");
    return (events as RuntimeEvent[]).filter(e => e.event != "Dummy");
  }

  async updateSource(source: string): Promise<void> {
    return this.post('/update_code', source);
  }
}

function isDeviceBlacklisted(device: Device) {
  if (device.id.includes("fleap_iphone")) {
    return true;
  }
  if (device.id.includes("a35_isabel")) {
    return true;
  }
  if (device.integration.includes("DummyIntegration")) return true;
  return false;
}
