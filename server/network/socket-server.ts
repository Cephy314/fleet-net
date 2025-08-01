import type { Server as HTTPServer } from 'node:http';
import { type Socket, Server as SocketIOServer } from 'socket.io';
import type {
    ChannelEventResponse,
    SocketResponse,
} from '../../shared/types/socket-events';
import { SessionManager } from '../session-manager';

export class SocketServer {
    private io: SocketIOServer;
    private readonly sessionManager: SessionManager;

    constructor(httpServer: HTTPServer) {
        this.sessionManager = SessionManager.getInstance();

        // ensure the session manager is initialized
        if (!this.sessionManager) {
            throw new Error('SessionManager is not initialized');
        }
        // initialize Socket.IO with CORS settings for development
        this.io = new SocketIOServer(httpServer, {
            cors: {
                origin: '*', // TODO: resrict to specific origins in production
                methods: ['GET', 'POST'],
            },
        });

        this.setupEventHandlers();
    }

    private setupEventHandlers(): void {
        this.io.on('connection', (socket: Socket) => {
            console.log(`Client connected: ${socket.id}`);

            // Handle initial connection.
            this.handleConnection(socket);

            // Setup disconnection handler
            socket.on('disconnect', () => {
                this.handleDisconnection(socket);
            });
            socket.on(
                'channel:join',
                (
                    channelId: number,
                    callback: (
                        response: SocketResponse<ChannelEventResponse>,
                    ) => void,
                ) => {
                    this.handleChannelJoin(socket, channelId, callback);
                },
            );
        });
    }

    private handleChannelJoin(
        socket: Socket,
        channelId: number,
        callback: (response: SocketResponse<ChannelEventResponse>) => void,
    ): void {
        const session = this.sessionManager.getSessionBySocketId(socket.id);
        if (!session) {
            console.error(`Session not found for socket ID: ${socket.id}`);
            callback({
                success: false,
                error: 'Session not found',
            });
            return;
        }

        // Add channel to user's subscribed channels
        session.subscribedChannels.add(channelId);

        // Join the Socket.IO room for the channel
        socket.join(`channel:${channelId}`);

        // TODO: Notify other users in the channel

        callback({ success: true, data: { channelId } });
    }

    private handleConnection(socket: Socket): void {
        // Create a new session for the connected client.
        const session = this.sessionManager.createSession(socket.id);

        // Send welcome message with userId;
        socket.emit('welcome', {
            userId: session.userId,
            udpSecret: session.udpSecret.toString('base64'),
        });
    }

    private handleDisconnection(socket: Socket): void {
        console.log(`Client disconnected: ${socket.id}`);

        // find and remove the session.
        const session = this.sessionManager.getSessionBySocketId(socket.id);
        if (session) {
            this.sessionManager.removeSession(session.userId);
            console.log(`Session for user ${session.userId} removed.`);
        }
    }

    public start(): void {
        console.log('Socket.IO server started');
        // TODO: Start the server.
    }

    public async stop(callback?: () => void): Promise<void> {
        console.log('Socket.IO server stopped');
        await this.io.close(() => {
            if (callback) {
                callback();
            }
        });
    }
}
