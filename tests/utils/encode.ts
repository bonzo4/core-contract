export function encodeUUID(uuid: string) {
    const hex = uuid.replace(/-/g, ""); // Remove hyphens
    const buffer = Buffer.from(hex, "hex"); // Convert hex to binary buffer
    return buffer.toString("base64"); // Convert binary buffer to base64
  }