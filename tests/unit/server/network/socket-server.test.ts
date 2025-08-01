// noinspection t

import type { Server as HTTPServer } from 'node:http';
import { createServer } from 'node:http';
import { type Socket as ClientSocket, io as ioc } from 'socket.io-client';
import { SocketServer } from '../../../../server/network/socket-server';
import { SessionManager } from '../../../../server/session-manager';

describe('SocketServer', () => {
    let httpServer: HTTPServer;
    let socketServer: SocketServer;
    let clientSocket: ClientSocket;
    let serverPort: number;
    let sessionManager: SessionManager;

    beforeAll((done) => {
        // Create a simple HTTP server
        httpServer = createServer();

        // Start listening on a random port
        httpServer.listen(() => {
            const address = httpServer.address();
            serverPort =
                typeof address === 'object' && address !== null
                    ? address.port
                    : 3000;
            done();
        });
    });

    beforeEach(() => {
        // Get the singleton instance of SessionManager
        sessionManager = SessionManager.getInstance();

        // clear any existing sessions
        sessionManager.removeSessions(sessionManager.getAllSessions());

        // Create new socket server for each test
        socketServer = new SocketServer(httpServer);
        socketServer.start();
    });

    afterEach(() => {
        // Cleanup client socket if exists
        if (clientSocket?.connected) {
            clientSocket.disconnect();
        }

        // Clear all sessions after each test
        sessionManager.removeSessions(sessionManager.getAllSessions());
    });

    afterAll((done) => {
        // Close the HTTP server after all tests
        if (socketServer) {
            socketServer.stop(() => {
                httpServer.close(() => {
                    done();
                });
            });
        } else {
            httpServer.close(() => {
                done();
            });
        }
    });

    describe('Constructor', () => {
        it('should initialize with SessioinManager singleton', () => {
            expect(socketServer).toBeDefined();
        });
    });

    describe('Connection handling', () => {
        it('should create a session when client connects', (done) => {
            clientSocket = ioc(`http://localhost:${serverPort}`, {
                autoConnect: true,
                reconnection: false,
            });

            clientSocket.on(
                'welcome',
                (data: { userId: number; udpSecret: string }) => {
                    // Verify welcome message
                    expect(data).toHaveProperty('userId');
                    expect(data).toHaveProperty('udpSecret');
                    expect(typeof data.userId).toBe('number');
                    expect(data.userId).toBeGreaterThanOrEqual(1);
                    expect(data.userId).toBeLessThanOrEqual(65535);
                    expect(typeof data.udpSecret).toBe('string');
                    expect(data.udpSecret.length).toBe(44); // Base64 encoded 32

                    // Verify session in SessionManager
                    const session = sessionManager.getSessionByUserId(
                        data.userId,
                    );
                    expect(session).toBeDefined();
                    expect(session.socketId).toBe(clientSocket.id);

                    done();
                },
            );
        });
        it('should create unique sessions for multiple clients', (done) => {
            const clients: ClientSocket[] = [];
            const userIds: Set<number> = new Set();
            let connectedCount = 0;
            const totalClients = 5;

            for (let i = 0; i < totalClients; i++) {
                const client = ioc(`http://localhost:${serverPort}`, {
                    autoConnect: true,
                    reconnection: false,
                    forceNew: true,
                });

                clients.push(client);

                client.on(
                    'welcome',
                    (data: { userId: number; udpSecret: string }) => {
                        userIds.add(data.userId);
                        connectedCount++;

                        if (connectedCount === totalClients) {
                            // All clients connected, verify unique user IDs
                            expect(userIds.size).toBe(totalClients);

                            // Verify all sessions exist
                            const sessions = sessionManager.getAllSessions();
                            expect(sessions.length).toBe(totalClients);

                            // Clean up
                            clients.forEach((c) => c.disconnect());
                            done();
                        }
                    },
                );
            }
        });

        it('should remove session when client disconnects', (done) => {
            let userId: number;

            clientSocket = ioc(`http://localhost:${serverPort}`, {
                autoConnect: true,
                reconnection: false,
            });

            clientSocket.on(
                'welcome',
                (data: { userId: number; udpSecret: string }) => {
                    userId = data.userId;

                    // Verify session exists
                    const session = sessionManager.getSessionByUserId(userId);
                    expect(session).toBeDefined();

                    // Disconnect the client
                    clientSocket.disconnect();

                    // Give some time for disconnection to process
                    setTimeout(() => {
                        // Verify session is removed
                        const removedSession =
                            sessionManager.getSessionByUserId(userId);
                        expect(removedSession).toBeUndefined();
                        done();
                    }, 50);
                },
            );
        });

        it('should handle rapid connect/disconnect', (done) => {
            let connectCount = 0;
            const cycles = 5;

            function performCycle() {
                const client = ioc(`http://localhost:${serverPort}`, {
                    autoConnect: true,
                    reconnection: false,
                    forceNew: true,
                });

                client.on(
                    'welcome',
                    (_data: { userId: number; udpSecret: string }) => {
                        // Immediately disconnect
                        client.disconnect();
                        connectCount++;

                        if (connectCount < cycles) {
                            // Perform next cycle
                            setTimeout(performCycle, 10);
                        } else {
                            //  Verify all sessions are removed
                            setTimeout(() => {
                                const sessions =
                                    sessionManager.getAllSessions();
                                expect(sessions.length).toBe(0);
                                done();
                            }, 100);
                        }
                    },
                );
            }

            performCycle();
        });
    });

    describe('Event Handlers', () => {
        let connectedSocket: ClientSocket;
        let userId: number;

        beforeEach((done) => {
            // Setup a connected client for each test
            connectedSocket = ioc(`http://localhost:${serverPort}`, {
                autoConnect: true,
                reconnection: false,
            });

            connectedSocket.on(
                'welcome',
                (data: { userId: number; udpSecret: string }) => {
                    userId = data.userId;
                    done();
                },
            );
        });

        afterEach(() => {
            if (connectedSocket?.connected) {
                connectedSocket.disconnect();
            }
        });

        describe('channel:join', () => {
            it('should allow user to join a channel', (done) => {
                const channelId = 1;

                connectedSocket.emit(
                    'channel:join',
                    channelId,
                    (response: { success: boolean }) => {
                        expect(response.success).toBe(true);

                        // Verify session has joined the channel
                        const session =
                            sessionManager.getSessionByUserId(userId);
                        expect(session?.subscribedChannels.has(channelId)).toBe(
                            true,
                        );

                        done();
                    },
                );
            });
        });
    });
});
