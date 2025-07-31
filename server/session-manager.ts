import { randomBytes } from 'node:crypto';
import type { Session } from '../shared/session';
import { isIp } from '../shared/validation';

export class SessionManager {
  private static instance: SessionManager;

  private sessionsByUserId: { [userId: number]: Session } = {};
  private sessionsBySocketId: { [socketId: string]: Session } = {};
  private sessionsByUdpKey: { [udpKey: string]: Session } = {};
  private nextUserId: number = 1; // Start at 1 to reserve 0 for "invalid"

  /**
   * Private constructor to enforce singleton pattern.
   * Use SessionManager.getInstance() to access the singleton instance.
   */
  private constructor() {
    // add stuff here!
  }

  /**
   * Get the singleton instance of SessionManager.
   * @returns {SessionManager} The singleton instance.
   */
  public static getInstance(): SessionManager {
    if (!SessionManager.instance) {
      SessionManager.instance = new SessionManager();
    }

    return SessionManager.instance;
  }

  /**
   * Create a new session for a user.
   * @param {string} socketId - The Socket.IO connection ID.
   * @param {string} [clientVersion] - Optional client version string.
   * @returns {Session} The created session object.
   */
  public createSession(socketId: string, clientVersion?: string): Session {
    // Allocate a unique user ID
    const userId = this.allocateUserId();

    // Generate a secure random secret for UDP HMAC (32 bytes)
    const udpSecret = randomBytes(32); // 256 bits of entropy
    const session: Session = {
      userId,
      socketId,
      udpSecret,
      permissions: new Set(),
      subscribedChannels: new Set(),
      connectedAt: Date.now(), // UTC Unix timestamp
      clientVersion,
    };

    // Store the session in all lookup tables
    this.sessionsByUserId[userId] = session;
    this.sessionsBySocketId[socketId] = session;
    // Add UDP endpoint when we receive the first packet from the user.

    return session;
  }

  /**
   * Update the UDP endpoint for a session.
   * This method is called when the first UDP packet is received from the user.
   * @param userId
   * @param address
   * @param port
   */
  public updateUdpEndpoint(
    userId: number,
    address: string,
    port: number,
  ): boolean {
    // validate address and port, ensure they are not empty and within valid ranges
    if (!address || !port || port < 1 || port > 65535 || !isIp(address)) {
      return false; // Invalid address or port
    }

    const session = this.getSessionByUserId(userId);
    if (!session) {
      return false; // Session not found
    }

    // Check if the session already has a different UDP endpoint (NAT change)
    if (session.udpAddress && session.udpPort) {
      const oldUdpKey = this.getUdpKey(session.udpAddress, session.udpPort);
      delete this.sessionsByUdpKey[oldUdpKey]; // Remove old UDP key
    }

    // Update the UDP endpoint
    session.udpAddress = address;
    session.udpPort = port;
    session.lastUdpActivity = Date.now(); // Update last activity unix timestamp

    // Store the session in the UDP key lookup
    const udpKey = this.getUdpKey(address, port);
    this.sessionsByUdpKey[udpKey] = session;

    return true; // Successfully updated
  }

  /**
   * Remove a session by user ID.
   * @param userId
   * @return {boolean} True if the session was successfully removed, false if not found.
   */
  public removeSession(userId: number): boolean {
    const session = this.sessionsByUserId[userId];
    if (!session) {
      return false; // Session not found
    }

    // Remove from all lookup tables.
    delete this.sessionsByUserId[userId];
    delete this.sessionsBySocketId[session.socketId];

    // Remove UDP endpoint if it exists
    if (session.udpAddress && session.udpPort) {
      const udpKey = this.getUdpKey(session.udpAddress, session.udpPort);
      delete this.sessionsByUdpKey[udpKey];
    }

    return true; // Successfully removed
  }

  /**
   * Get a session by its user ID.
   * @param userId
   */
  public getSessionByUserId(userId: number): Session | undefined {
    return this.sessionsByUserId[userId];
  }

  /**
   * Get a session by its Socket.IO connection ID.
   * @param socketId
   */
  public getSessionBySocketId(socketId: string): Session | undefined {
    return this.sessionsBySocketId[socketId];
  }

  /**
   * Get a session by its UDP endpoint.
   * @param address
   * @param port
   */
  public getSessionByUdpEndpoint(
    address: string,
    port: number,
  ): Session | undefined {
    const udpKey = this.getUdpKey(address, port);
    return this.sessionsByUdpKey[udpKey];
  }

  /**
   * Allocate a unique user ID for a new session.
   * This method ensures that the user ID is not already in use.
   * @returns {number} The allocated user ID.
   * @throws {Error} If no available user IDs are found.
   */
  private allocateUserId(): number {
    const startId = this.nextUserId;

    // Find the next available ID
    while (this.sessionsByUserId[this.nextUserId]) {
      this.advanceUserId();

      // Safety check: if we've checked all IDs, the server is full and this is the most
      // efficient and powerful voice server in the universe. All hail CephyPi!
      if (this.nextUserId === startId) {
        throw new Error('No available user IDs');
      }
    }

    const allocatedId = this.nextUserId;
    this.advanceUserId();

    return allocatedId;
  }

  /**
   * Advance the next user ID to the next available value.
   * This method wraps around to 1 if it exceeds 65535.
   */
  private advanceUserId(): void {
    this.nextUserId++;
    if (this.nextUserId > 65535) {
      this.nextUserId = 1; // Wrap to start and don't use 0 since that is the invalid ID.
    }
  }

  /**
   * Update the UDP endpoint for a session.
   * This method is called when the first UDP packet is received from the user.
   * @param address The UDP address.
   * @param port The UDP port.
   */
  private getUdpKey(address: string, port: number): string {
    return `${address}:${port}`;
  }
}
