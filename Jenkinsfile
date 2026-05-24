// CasGarage Jenkins Pipeline
// CI/CD automation for multi-platform builds and releases
// Server: jenkins.casjay.cc

pipeline {
    agent none

    options {
        buildDiscarder(logRotator(numToKeepStr: '10'))
        disableConcurrentBuilds()
        timeout(time: 2, unit: 'HOURS')
        timestamps()
    }

    environment {
        PROJECT_NAME = 'casgarage'
        ORG = 'casapps'
        GHCR_REPO = "ghcr.io/${ORG}/${PROJECT_NAME}"

        // Credentials
        GITHUB_CREDENTIALS = credentials('github-token')
        DOCKER_CREDENTIALS = credentials('ghcr-token')

        // Version from release.txt
        VERSION = sh(script: 'cat release.txt 2>/dev/null || echo "0.1.0"', returnStdout: true).trim()
    }

    stages {
        stage('Preparation') {
            agent { label 'amd64' }
            steps {
                echo "🚀 Starting CasGarage build pipeline"
                echo "Version: ${VERSION}"
                echo "Branch: ${env.BRANCH_NAME}"

                // Clean workspace
                cleanWs()

                // Checkout code
                checkout scm

                // Install dependencies
                sh '''
                    echo "📦 Installing build dependencies..."
                    command -v rustc || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    command -v cross || cargo install cross
                    command -v trunk || cargo install trunk
                '''
            }
        }

        stage('Code Quality') {
            parallel {
                stage('Lint (amd64)') {
                    agent { label 'amd64' }
                    steps {
                        echo "🔍 Running linters on amd64..."
                        sh '''
                            cargo fmt -- --check || true
                            cargo clippy --all-targets --all-features -- -D warnings || true
                        '''
                    }
                }

                stage('Security Audit') {
                    agent { label 'amd64' }
                    steps {
                        echo "🔒 Running security audit..."
                        sh '''
                            cargo install cargo-audit || true
                            cargo audit || true
                        '''
                    }
                }
            }
        }

        stage('Test') {
            parallel {
                stage('Unit Tests (amd64)') {
                    agent { label 'amd64' }
                    steps {
                        echo "🧪 Running unit tests on amd64..."
                        sh 'cargo test --all-features --workspace'
                    }
                }

                stage('Unit Tests (arm64)') {
                    agent { label 'arm64' }
                    steps {
                        echo "🧪 Running unit tests on arm64..."
                        sh 'cargo test --all-features --workspace'
                    }
                }
            }
        }

        stage('Build') {
            parallel {
                stage('Build amd64') {
                    agent { label 'amd64' }
                    steps {
                        echo "🏗️ Building for amd64..."
                        sh '''
                            make build
                            ls -lh binaries/
                        '''

                        // Archive binaries
                        archiveArtifacts artifacts: 'binaries/*amd64*', fingerprint: true
                    }
                }

                stage('Build arm64') {
                    agent { label 'arm64' }
                    steps {
                        echo "🏗️ Building for arm64..."
                        sh '''
                            make build
                            ls -lh binaries/
                        '''

                        // Archive binaries
                        archiveArtifacts artifacts: 'binaries/*arm64*', fingerprint: true
                    }
                }
            }
        }

        stage('Docker Build') {
            when {
                anyOf {
                    branch 'main'
                    branch 'develop'
                    tag pattern: "v\\d+\\.\\d+\\.\\d+", comparator: "REGEXP"
                }
            }
            agent { label 'amd64' }
            steps {
                echo "🐳 Building multi-arch Docker images..."
                sh '''
                    # Login to GitHub Container Registry
                    echo "${DOCKER_CREDENTIALS_PSW}" | docker login ghcr.io -u "${DOCKER_CREDENTIALS_USR}" --password-stdin

                    # Build and push
                    make docker
                '''
            }
        }

        stage('Integration Tests') {
            when {
                anyOf {
                    branch 'main'
                    branch 'develop'
                }
            }
            agent { label 'amd64' }
            steps {
                echo "🔗 Running integration tests..."
                sh '''
                    # Start test environment in Docker
                    docker compose -f docker-compose.yml up -d

                    # Wait for service to be ready
                    timeout 60 bash -c 'until docker exec casgarage /usr/local/bin/casgarage --status; do sleep 2; done'

                    # Run integration tests
                    cargo test --test '*' -- --test-threads=1

                    # Cleanup
                    docker compose -f docker-compose.yml down -v
                '''
            }
        }

        stage('Release') {
            when {
                tag pattern: "v\\d+\\.\\d+\\.\\d+", comparator: "REGEXP"
            }
            agent { label 'amd64' }
            steps {
                echo "📦 Creating GitHub release..."
                sh '''
                    # Ensure gh CLI is installed
                    command -v gh || {
                        curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
                        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
                        sudo apt update && sudo apt install gh -y
                    }

                    # Authenticate
                    echo "${GITHUB_CREDENTIALS_PSW}" | gh auth login --with-token

                    # Create release
                    make release
                '''
            }
        }

        stage('Deploy Documentation') {
            when {
                branch 'main'
            }
            agent { label 'amd64' }
            steps {
                echo "📚 Deploying documentation..."
                sh '''
                    # Build documentation
                    cargo doc --no-deps --workspace

                    # Deploy to GitHub Pages (if configured)
                    # Or deploy to ReadTheDocs via webhook
                    # Implementation depends on docs hosting setup
                    echo "Documentation build complete"
                '''
            }
        }
    }

    post {
        success {
            echo "✅ Pipeline completed successfully!"
        }

        failure {
            echo "❌ Pipeline failed!"
            // Send notifications (email, Slack, etc.)
        }

        always {
            echo "🧹 Cleaning up..."
            node('amd64') {
                // Cleanup Docker resources
                sh '''
                    docker system prune -f || true
                    docker volume prune -f || true
                '''
            }
        }
    }
}
