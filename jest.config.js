/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
    projects: [
        {
            displayName: 'server',
            preset: 'ts-jest',
            testEnvironment: 'node',
            testMatch: ['<rootDir>/server/**/*.test.ts', '<rootDir>/tests/unit/server/**/*.test.ts'],
            moduleNameMapper: {
                '^@shared/(.*)$': '<rootDir>/shared/$1'
            }
        },
        {
            displayName: 'client-main',
            preset: 'ts-jest',
            testEnvironment: 'node',
            testMatch: ['<rootDir>/client/main/**/*.test.ts',
                '<rootDir>/tests/unit/client-main/**/*.test.ts'],
            moduleNameMapper: {
                '^@shared/(.*)$': '<rootDir>/shared/$1'
            }
        },
        {
            displayName: 'client-renderer',
            preset: 'ts-jest',
            testEnvironment: 'jsdom',
            testMatch: ['<rootDir>/client/renderer/**/*.test.ts',
                '<rootDir>/tests/unit/client-renderer/**/*.test.ts'],
            moduleNameMapper: {
                '^@shared/(.*)$': '<rootDir>/shared/$1'
            }
        },
        {
            displayName: 'shared',
            preset: 'ts-jest',
            testEnvironment: 'node',
            testMatch: ['<rootDir>/shared/**/*.test.ts', '<rootDir>/tests/unit/shared/**/*.test.ts'],
            transform: {
                '^.+\\.ts$': ['ts-jest', {
                    tsconfig: '<rootDir>/shared/tsconfig.json'
                }]
            }
        }
    ],
    coverageDirectory: '<rootDir>/coverage',
    collectCoverageFrom: [
        'server/**/*.ts',
        'client/**/*.ts',
        'shared/**/*.ts',
        '!**/*.test.ts',
        '!**/node_modules/**',
        '!**/*.d.ts'
    ]
};