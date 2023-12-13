variable "github_token" {
  type        = string
  description = "Your Github OAuth token"
}

variable "github_owner" {
  type        = string
  description = "The owner of the Github repository"
}

variable "github_name" {
  type        = string
  description = "The name of the Github repository"
}

variable "github_description" {
  type        = string
  description = "The description of the Github repository"
}

variable "github_visibility" {
  type        = string
  default     = "private"
  description = "The visibility of the Github repository (public or private)"
}

variable "gha_op_service_account" {
  type        = string
  description = "A 1Password service account token with access to the right vault"
}

variable "fly_token" {
  type        = string
  description = "A Fly.io API token with access to the right organization"
}

terraform {
  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 5.0"
    }

    fly = {
      source = "pi3ch/fly"
      version = "0.0.24"
    }
  }
}

provider "github" {
  token = var.github_token
  owner = var.github_owner
}

provider "fly" {
  fly_api_token = var.fly_token
}

resource "github_repository" "repository" {
  name        = var.github_name
  description = var.github_description
  visibility  = var.github_visibility

  has_issues           = true
  has_projects         = false
  has_wiki             = false
  has_downloads        = false
  vulnerability_alerts = true

  allow_merge_commit     = false
  allow_squash_merge     = true
  allow_rebase_merge     = false
  allow_auto_merge       = true
  delete_branch_on_merge = true
}

resource "github_actions_secret" "op_service_account" {
  repository = var.github_name

  secret_name     = "OP_SERVICE_ACCOUNT"
  plaintext_value = var.gha_op_service_account
}

resource "fly_app" "app_backend" {
  name = "${var.github_name}-backend"
}

resource "fly_machine" "machine_backend" {
  app = fly_app.app_backend.name
  image = "registry.fly.io/${var.github_name}-backend:latest"
  region = "iad"
  services = [
    {
      ports = [
        {
          port = 443
          handlers = ["tls", "http"]
        },
        {
          port = 80
          handlers = ["http"]
        }
      ]
      internal_port = 3000
      protocol = "tcp"
    }
  ]
}

resource "fly_ip" "ip_backend" {
  app = fly_app.app_backend.name
  type = "v4"
}

resource "fly_app" "app_frontend" {
  name = "${var.github_name}-frontend"
}

resource "fly_machine" "machine_frontend" {
  app = fly_app.app_frontend.name
  image = "registry.fly.io/${var.github_name}-frontend:latest"
  region = "iad"
  services = [
    {
      ports = [
        {
          port = 443
          handlers = ["tls", "http"]
        },
        {
          port = 80
          handlers = ["http"]
        }
      ]
      internal_port = 8080
      protocol = "tcp"
    }
  ]
}

resource "fly_ip" "ip_frontend" {
  app = fly_app.app_frontend.name
  type = "v4"
}
