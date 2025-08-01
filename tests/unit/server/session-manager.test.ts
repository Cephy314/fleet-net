import { SessionManager } from '../../../server/session-manager';
import type { Session } from '../../../shared/session';

describe('SessionManager', () => {
  let sessionManager: SessionManager;

  beforeEach(() => {
    // Get fresh instance of SessionManager before each test
    sessionManager = SessionManager.getInstance();
  });

  afterEach(() => {
    // Clear all sessions after each test
    sessionManager.removeSessions(sessionManager.getAllSessions());
  });

  describe('Singleton Pattern', () => {
    it('should return the same instance on multiple calls', () => {
      const instance1 = SessionManager.getInstance();
      const instance2 = SessionManager.getInstance();
      expect(instance1).toBe(instance2);
    });
  });

  describe('createSession', () => {
    it('should create a session with required fields', () => {
      const socketId = 'test-socket-id';
      const session = sessionManager.createSession(socketId);

      expect(session.userId).toBeGreaterThanOrEqual(1);
      expect(session.userId).toBeLessThanOrEqual(65535); // 16-bit ID
      expect(session.socketId).toBe(socketId);
      expect(session.udpSecret).toBeInstanceOf(Buffer);
      expect(session.udpSecret.length).toBe(32); // 256 bits
      expect(session.permissions).toBeInstanceOf(Set);
      expect(session.subscribedChannels).toBeInstanceOf(Set);
      expect(session.connectedAt).toBeGreaterThan(0); // Unix timestamp
      expect(session.clientVersion).toBeUndefined(); // Not set by default
    });

    it('should create a session with client version', () => {
      const socketId = 'test-socket-id';
      const clientVersion = '1.0.0';
      const session = sessionManager.createSession(socketId, clientVersion);

      expect(session.clientVersion).toBe(clientVersion);
    });

    it('should allocate unique user IDs', () => {
      const sessions: Session[] = [];
      const userIds = new Set<number>();

      for (let i = 0; i < 10; i++) {
        const session = sessionManager.createSession(`socket-${i}`);
        sessions.push(session);
        userIds.add(session.userId);
      }

      // All user IDs should be unique
      expect(userIds.size).toBe(10);

      // Clean up
      sessions.forEach((session) => {
        sessionManager.removeSession(session.userId);
      });
    });
  });

  describe('Session lookup methods', () => {
    let testSession: Session;

    beforeEach(() => {
      testSession = sessionManager.createSession('lookup-test-socket');
    });

    afterEach(() => {
      sessionManager.removeSession(testSession.userId);
    });

    it('should find session by userId', () => {
      const found = sessionManager.getSessionByUserId(testSession.userId);
      expect(found).toBe(testSession);
    });

    it('should find session by socketId', () => {
      const found = sessionManager.getSessionBySocketId(testSession.socketId);
      expect(found).toBe(testSession);
    });

    it('should return undefined for non-existent userId', () => {
      const found = sessionManager.getSessionByUserId(9999); // Non-existent ID
      expect(found).toBeUndefined();
    });

    it('should return undefined for non-existent socketId', () => {
      const found = sessionManager.getSessionBySocketId('non-existent-socket');
      expect(found).toBeUndefined();
    });
  });

  describe('updateUdpEndpoint', () => {
    let testSession: Session;

    beforeEach(() => {
      testSession = sessionManager.createSession('udp-test-socket');
    });

    afterEach(() => {
      sessionManager.removeSession(testSession.userId);
    });

    it('should update UDP endpoint with valid data', () => {
      const address = '192.168.1.100';
      const port = 5000;

      const result = sessionManager.updateUdpEndpoint(
        testSession.userId,
        address,
        port,
      );

      expect(result).toBe(true);
      expect(testSession.udpAddress).toBe(address);
      expect(testSession.udpPort).toBe(port);
      expect(testSession.lastUdpActivity).toBeDefined();
      expect(testSession.lastUdpActivity).toBeLessThanOrEqual(Date.now());
    });

    it('should find session by UDP endpoint after update', () => {
      const address = '10.0.0.1';
      const port = 6000;

      sessionManager.updateUdpEndpoint(testSession.userId, address, port);
      const found = sessionManager.getSessionByUdpEndpoint(address, port);

      expect(found).toBe(testSession);
    });

    it('should handle NAT changes correctly', () => {
      const address1 = '192.168.1.1';
      const port1 = 5001;
      const address2 = '192.168.1.2';
      const port2 = 5002;

      // Set initial UDP endpoint
      sessionManager.updateUdpEndpoint(testSession.userId, address1, port1);

      // Update to a new NAT address
      sessionManager.updateUdpEndpoint(testSession.userId, address2, port2);

      // Old endpoint should not find session
      const oldLookup = sessionManager.getSessionByUdpEndpoint(address1, port1);
      expect(oldLookup).toBeUndefined();

      // New endpoint should find session
      const newLookup = sessionManager.getSessionByUdpEndpoint(address2, port2);
      expect(newLookup).toBe(testSession);
    });

    it('should reject invalid addresses and ports', () => {
      const invalidCases = [
        { address: '', port: 5000 }, // Empty address
        { address: 'not-an-ip', port: 5000 }, // Invalid IP
        { address: '256.256.256.256', port: 5000 }, // Out of range IP
        { address: '192.168.1.1', port: 0 }, // Invalid port (0)
        { address: '192.168.1.1', port: 70000 }, // Invalid port (too high)
        { address: '192.168.1.1', port: -1 }, // Invalid port (negative)
      ];

      invalidCases.forEach(({ address, port }) => {
        const result = sessionManager.updateUdpEndpoint(
          testSession.userId,
          address,
          port,
        );
        expect(result).toBe(false);
      });
    });

    it('should return flase for non-existent session', () => {
      const result = sessionManager.updateUdpEndpoint(
        9999,
        '192.168.1.1',
        5000,
      );
      expect(result).toBe(false); // Non-existent session should return false
    });
  });

  describe('removeSession', () => {
    it('should remove session and clean up all reference', () => {
      const socketId = 'remove-test-socket';
      const session = sessionManager.createSession(socketId);
      const userId = session.userId;

      // Add UDP endpoint
      sessionManager.updateUdpEndpoint(userId, '192.168.1.1', 5000);

      // Remove session
      const result = sessionManager.removeSession(userId);
      expect(result).toBe(true);

      // Verify all lookups return undefined
      expect(sessionManager.getSessionByUserId(userId)).toBeUndefined();
      expect(sessionManager.getSessionBySocketId(socketId)).toBeUndefined();
      expect(
        sessionManager.getSessionByUdpEndpoint('192.168.1.1', 5000),
      ).toBeUndefined();
    });

    it('should return false when removing non-existent session', () => {
      const result = sessionManager.removeSession(9999); // Non-existent userId
      expect(result).toBe(false); // Should return false
    });
  });

  describe('User ID allocoation', () => {
    it('should wrap around after reaching max ID', () => {});
  });

  describe('User ID allocation', () => {
    it('should wrap around after reaching max ID', () => {
      // Set the next user ID to the maximum value
      sessionManager.setNextUserId(65535);

      sessionManager.createSession('wrap-test-socket'); // Create session at max ID
      expect(sessionManager.getSessionByUserId(65535)).toBeDefined();
      const nextSession = sessionManager.createSession('wrap-test-socket');
      expect(nextSession.userId).toBe(1); // Should wrap to 1
    });
  });
});
