
export const DeviceTypes = [
  "Dummy",
  "DoorSensor",
  "Light",
  "Sensor",
  "PowerOutlet",
  "MotionSensor",
  "Unknown",
] as const;

export type DeviceType = typeof DeviceTypes[number];

export const deviceTypePriorities: Record<DeviceType, number> = {
  DoorSensor: 3,
  PowerOutlet: 3,
  MotionSensor: 3,
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
};

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
      body: body ? JSON.stringify(body) : undefined
    };

    try {
      const response = await fetch(url, options);

      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }

      return await response.json();
    } catch (error) {
      console.error('API request error:', error);
      throw error;
    }
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
    return devices;
  }

  async listPossibleEvents(): Promise<RuntimeEvent[]> {
    const events = await this.get("/possible_events");
    return events;
  }
}


