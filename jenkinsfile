pipeline {
    agent any

    environment {
        GHCR_CREDENTIALS = 'ghcr'
    }

    stages {
        stage('Clone Repository') {
            steps {
                checkout scm
                sh 'git submodule update --init'
            }
        }

        stage('Initialize') {
            steps {
                script {
                    env.GIT_COMMIT_HASH = sh(returnStdout: true, script: 'git rev-parse HEAD | cut -c 1-10').trim()
                    env.DOCKER_IMAGE_NAME = "ghrc.io/peacock-123/peacock-server"
                }
            }
        }

        stage('Build Docker Image') {
            steps {
                script {
                    docker.build("${env.DOCKER_IMAGE_NAME}:${env.GIT_COMMIT_HASH}", "--file Dockerfile .")
                }
            }
        }

        stage('Push to Github Package') {
            steps {
                script {
                    docker.withRegistry("https://ghcr.io/v2/", GHCR_CREDENTIALS) {
                        def customImage = docker.image("${env.DOCKER_IMAGE_NAME}:${env.GIT_COMMIT_HASH}")
                        customImage.push("${env.GIT_COMMIT_HASH}")
                        customImage.push("latest")
                    }
                }
            }
        }
    }

    post {
        always {
            script {
                sh 'docker logout'
            }
        }
    }
}
